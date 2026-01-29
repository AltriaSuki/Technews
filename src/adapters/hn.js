/**
 * Hacker News adapter.
 *
 * Fetches top stories from the HN Firebase API and returns Story[].
 * All HN-specific field mapping and error handling is contained here.
 *
 * This module is VOLATILE: HN API changes should only require
 * modifications to this file.
 */

import { createStory, SOURCES } from '../models/story.js';
import { extractKeywords } from '../infrastructure/keyword_engine.js';

const BASE_URL = 'https://hacker-news.firebaseio.com/v0';

/**
 * Fetch top stories from Hacker News.
 *
 * @param {{ limit?: number }} options
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
export async function fetchStories({ limit = 30 } = {}) {
  try {
    const response = await fetch(`${BASE_URL}/topstories.json`);
    if (!response.ok) throw new Error(`HN API error: ${response.status}`);

    const ids = await response.json();
    const items = await Promise.all(
      ids.slice(0, limit).map(id =>
        fetch(`${BASE_URL}/item/${id}.json`)
          .then(res => res.json())
          .catch(() => null)
      )
    );

    return items
      .filter(item => item && item.title)
      .map(item => createStory({
        id: `hn:${item.id}`,
        title: item.title,
        url: item.url || '',
        source: SOURCES.HACKERNEWS,
        score: normalizeScore(item.score || 0),
        comments: item.descendants || 0,
        author: item.by || 'unknown',
        timestamp: item.time || Math.floor(Date.now() / 1000),
        tags: extractKeywords(item.title),
        discussionUrl: `https://news.ycombinator.com/item?id=${item.id}`,
      }));
  } catch (error) {
    console.error('[adapter:hn] Failed to fetch stories:', error);
    return [];
  }
}

/**
 * Normalize HN score to 0–100 range.
 *
 * Uses log scale so the mapping handles the wide range of HN scores
 * (1 to 4000+) gracefully. This also makes cross-source comparison
 * meaningful when other adapters use the same approach.
 *
 * Approximate mapping:
 *   raw 1 → 0,  raw 10 → 22,  raw 50 → 38,
 *   raw 100 → 44, raw 500 → 60, raw 1000 → 66, raw 3000 → 77
 *
 * @param {number} raw - Raw HN score
 * @returns {number} Normalized score 0–100
 */
function normalizeScore(raw) {
  if (raw <= 0) return 0;
  return Math.min(100, Math.round((Math.log2(raw + 1) / 15) * 100));
}
