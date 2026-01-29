import { createStory, SOURCES } from '../models/story.js';
import { extractKeywords } from '../infrastructure/keyword_engine.js';

const API_BASE = 'https://api.github.com/search/repositories';

/**
 * Fetch trending repositories from GitHub.
 * Uses search API for repos created in the last 7 days, sorted by stars.
 *
 * @param {Object} options
 * @param {number} [options.limit=30]
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
export async function fetchStories({ limit = 30 } = {}) {
    try {
        const date = new Date();
        date.setDate(date.getDate() - 7);
        const dateStr = date.toISOString().split('T')[0];

        const query = `created:>${dateStr}`;
        const url = `${API_BASE}?q=${encodeURIComponent(query)}&sort=stars&order=desc&per_page=${limit}`;

        const response = await fetch(url, {
            headers: {
                'Accept': 'application/vnd.github.v3+json'
            }
        });

        if (!response.ok) {
            // Handle rate limiting gracefully
            if (response.status === 403 || response.status === 429) {
                console.warn('[github] Rate limit exceeded, returning empty list');
                return [];
            }
            throw new Error(`GitHub API error: ${response.status}`);
        }

        const data = await response.json();
        return (data.items || []).map(mapItemToStory);
    } catch (error) {
        console.error('[github] Fetch failed:', error);
        return [];
    }
}

function mapItemToStory(item) {
    return createStory({
        id: `gh-${item.id}`,
        title: `${item.full_name}: ${item.description || 'No description'}`,
        url: item.html_url,
        source: SOURCES.GITHUB,
        score: normalizeScore(item.stargazers_count),
        comments: item.forks_count, // Using forks as a proxy for "discussion/activity"
        author: item.owner ? item.owner.login : 'unknown',
        timestamp: new Date(item.created_at).getTime() / 1000,
        tags: extractKeywords(`${item.name} ${item.description || ''} ${item.language || ''}`),
        discussionUrl: `${item.html_url}/issues`,
    });
}

// Normalize stars to a 0-100 score. 
// Assumption: 1000 stars in a week is a "100" score story.
function normalizeScore(stars) {
    if (stars > 1000) return 100;
    return Math.min(100, Math.round((stars / 1000) * 100));
}
