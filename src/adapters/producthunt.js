import { createStory, SOURCES } from '../models/story.js';
import { extractKeywords } from '../infrastructure/keyword_engine.js';

// Using rss2json.com bridge to bypass CORS/Auth issues with official PH API
const RSS_FEED = 'https://www.producthunt.com/feed';
const API_URL = `https://api.rss2json.com/v1/api.json?rss_url=${encodeURIComponent(RSS_FEED)}`;

/**
 * Fetch top products from Product Hunt RSS feed.
 *
 * @param {Object} options
 * @param {number} [options.limit=30]
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
export async function fetchStories({ limit = 30 } = {}) {
    try {
        const response = await fetch(API_URL);
        if (!response.ok) throw new Error(`RSS Bridge error: ${response.status}`);

        const data = await response.json();
        if (data.status !== 'ok') throw new Error('RSS Bridge returned error status');

        // rss2json returns a limited set, we just take what we get (usually 10-20 items)
        return (data.items || []).slice(0, limit).map(mapItemToStory);
    } catch (error) {
        console.error('[producthunt] Fetch failed:', error);
        return [];
    }
}

function mapItemToStory(item) {
    // item.guid is usually the permalink, use it to generate ID
    const guid = item.guid && typeof item.guid === 'string' ? item.guid : '';
    const id = guid.split('/').pop() || item.title.replace(/\s+/g, '-');

    return createStory({
        id: `ph-${id}`,
        title: item.title,
        url: item.link,
        source: SOURCES.PRODUCTHUNT,
        score: 80, // RSS doesn't provide votes, default to high visibility
        comments: 0,
        author: item.author || 'Product Hunt',
        timestamp: new Date(item.pubDate).getTime() / 1000,
        tags: extractKeywords(`${item.title} ${item.categories ? item.categories.join(' ') : ''}`),
        discussionUrl: item.link,
    });
}
