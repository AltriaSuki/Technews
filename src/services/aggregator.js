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
import { settingsStore } from './settings_store.js';

/**
 * Registry of all available adapters.
 * Key must match a SOURCES value from models/story.js.
 * @type {Object<string, { fetchStories: Function }>}
 */
const ALL_ADAPTERS = {
  hackernews: hn,
  // github: github,       // Phase 1
  // reddit: reddit,       // Phase 1
  // producthunt: ph,      // Phase 1
  // arxiv: arxiv,         // Phase 1
};

/**
 * Fetch stories from all enabled sources, merge, deduplicate, and sort.
 *
 * @param {{ limit?: number }} [options] - Per-source story limit
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
export async function fetchAll({ limit } = {}) {
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

  return deduplicateAndSort(allStories);
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
