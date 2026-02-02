/**
 * Knowledge Store Service
 *
 * Tracks what the user reads to build a "Knowledge Profile".
 * Used to identify "Blind Spots" (topics the user rarely reads).
 *
 * Responsibilities:
 * - Record clicks on stories.
 * - Aggregate read counts by tag/category.
 * - Persist to localStorage.
 */

import { createStore } from '../infrastructure/storage.js';

const STORAGE_KEY = 'user_knowledge';

const store = createStore(STORAGE_KEY, {
    version: 1,
    defaultValue: () => ({
        history: [], // List of { storyId, timestamp, tags }
        tagCounts: {}, // { "tag": count }
    }),
});

/**
 * Record that a story was "read" (clicked).
 * @param {import('../models/story.js').Story} story
 */
export function markAsRead(story) {
    const data = store.read();

    // Avoid duplicate records for the same story (simple dedup)
    if (data.history.some(h => h.storyId === story.id)) {
        return;
    }

    // Record history
    data.history.push({
        storyId: story.id,
        timestamp: Date.now(),
        tags: story.tags
    });

    // Update tag profile
    story.tags.forEach(tag => {
        const key = tag.toLowerCase();
        data.tagCounts[key] = (data.tagCounts[key] || 0) + 1;
    });

    store.write(data);
}

/**
 * Get the user's top interests based on read history.
 * @param {number} topN
 * @returns {Array<{ tag: string, count: number }>}
 */
export function getInterests(topN = 10) {
    const data = store.read();
    return Object.entries(data.tagCounts)
        .map(([tag, count]) => ({ tag, count }))
        .sort((a, b) => b.count - a.count)
        .slice(0, topN);
}

/**
 * Get the full tag profiles (for Blind Spots analysis).
 * @returns {Object} { "tag": count }
 */
export function getTagProfile() {
    return store.read().tagCounts;
}
