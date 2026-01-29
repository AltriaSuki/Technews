/**
 * Aggregator — the single entry point for fetching stories.
 *
 * Owns: adapter registry, parallel fetching, deduplication, sorting.
 * This module imports adapters directly (the only service allowed to do so).
 *
 * Adding a new source: import the adapter, add it to ALL_ADAPTERS.
 * No other file changes required.
 */

import * as hn from '../adapters/hn.js';
import * as github from '../adapters/github.js';
import * as reddit from '../adapters/reddit.js';
import * as ph from '../adapters/producthunt.js';
import * as arxiv from '../adapters/arxiv.js';
import { settingsStore } from './settings_store.js';

/**
 * Registry of all available adapters.
 * Key must match a SOURCES value from models/story.js.
 * @type {Object<string, { fetchStories: Function }>}
 */
const ALL_ADAPTERS = {
  hackernews: hn,
  github: github,
  reddit: reddit,
  producthunt: ph,
  arxiv: arxiv,
};

/**
 * Fetch stories from all enabled sources, merge, deduplicate, and sort.
 *
 * @param {{ limit?: number }} [options] - Per-source story limit
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
// Cache state
let cache = {
  data: [],
  timestamp: 0,
  ttl: 5 * 60 * 1000 // 5 minutes
};

/**
 * Fetch stories from all enabled sources, merge, deduplicate, and sort.
 * Uses a simple time-based cache.
 *
 * @param {{ limit?: number, forceRefresh?: boolean }} [options]
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
export async function fetchAll({ limit, forceRefresh = false } = {}) {
  // Check cache (unless forcing refresh)
  const now = Date.now();
  if (!forceRefresh && cache.data.length > 0 && (now - cache.timestamp < cache.ttl)) {
    return [...cache.data]; // Return copy to be safe
  }

  const settings = settingsStore.getAll();
  const enabledSources = settings.enabledSources || [];
  const perSourceLimit = limit || settings.storiesPerSource || 30;

  const enabledAdapters = Object.entries(ALL_ADAPTERS)
    .filter(([name]) => enabledSources.includes(name));

  if (enabledAdapters.length === 0) {
    console.warn('[aggregator] No enabled adapters found.');
    return [];
  }

  const results = await Promise.allSettled(
    enabledAdapters.map(([, adapter]) =>
      adapter.fetchStories({ limit: perSourceLimit })
    )
  );

  const allStories = [];
  for (const result of results) {
    if (result.status === 'fulfilled') {
      allStories.push(...result.value);
    } else {
      console.warn('[aggregator] Adapter failed:', result.reason);
    }
  }

  const finalData = deduplicateAndSort(allStories);

  // Update cache
  cache = {
    data: finalData,
    timestamp: now,
    ttl: cache.ttl
  };

  return finalData;
}

/**
 * Remove duplicate stories (by URL) keeping the highest-scored version,
 * then sort by score descending, then by timestamp descending.
 * @private
 */
function deduplicateAndSort(stories) {
  const seen = new Map();

  for (const story of stories) {
    // Use URL as dedup key when available, otherwise fall back to ID.
    // Stories without URLs (e.g., "Ask HN") are never deduped against each other.
    const key = story.url || story.id;
    const existing = seen.get(key);
    if (!existing || existing.score < story.score) {
      seen.set(key, story);
    }
  }

  return [...seen.values()].sort(
    (a, b) => b.score - a.score || b.timestamp - a.timestamp
  );
}
