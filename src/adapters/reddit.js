import { createStory, SOURCES } from '../models/story.js';
import { extractKeywords } from '../infrastructure/keyword_engine.js';

const SUBREDDITS = ['programming', 'technology', 'machinelearning', 'javascript', 'webdev'];

/**
 * Fetch hot posts from tech subreddits.
 *
 * @param {Object} options
 * @param {number} [options.limit=30]
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
export async function fetchStories({ limit = 30 } = {}) {
    try {
        // Fetch from all subreddits in parallel
        const promises = SUBREDDITS.map(sub => fetchSubreddit(sub, Math.ceil(limit / SUBREDDITS.length)));
        const results = await Promise.allSettled(promises);

        const stories = [];
        for (const result of results) {
            if (result.status === 'fulfilled') {
                stories.push(...result.value);
            }
        }
        return stories;
    } catch (error) {
        console.error('[reddit] Fetch failed:', error);
        return [];
    }
}

async function fetchSubreddit(subreddit, limit) {
    try {
        // Use CORS proxy to bypass localhost restrictions
        const PROXY = 'https://corsproxy.io/?';
        const targetUrl = encodeURIComponent(`https://www.reddit.com/r/${subreddit}/hot.json?limit=${limit}`);
        const response = await fetch(`${PROXY}${targetUrl}`);
        if (!response.ok) throw new Error(`Reddit API error: ${response.status}`);

        const data = await response.json();
        if (!data.data || !data.data.children) return [];

        return data.data.children
            .filter(child => !child.data.stickied) // Skip stickied posts
            .map(child => mapItemToStory(child.data));
    } catch (e) {
        console.warn(`[reddit] Failed to fetch r/${subreddit}:`, e);
        return [];
    }
}

function mapItemToStory(item) {
    return createStory({
        id: `reddit-${item.id}`,
        title: item.title,
        url: item.url,
        source: SOURCES.REDDIT,
        score: normalizeScore(item.score),
        comments: item.num_comments,
        author: item.author,
        timestamp: item.created_utc,
        tags: extractKeywords(`${item.title} ${item.subreddit}`),
        discussionUrl: `https://reddit.com${item.permalink}`,
    });
}

// Normalize upvotes. 
// Assumption: 500 upvotes is a "100" score story.
function normalizeScore(upvotes) {
    if (upvotes > 500) return 100;
    return Math.min(100, Math.round((upvotes / 500) * 100));
}
