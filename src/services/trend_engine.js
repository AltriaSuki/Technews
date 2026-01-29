/**
 * Trend engine — keyword frequency tracking and spike detection.
 *
 * Owns: techpulse_keyword_history localStorage key.
 * No other module may read or write this key.
 *
 * Uses keyword_engine for all keyword processing.
 * Must not implement its own tokenization or normalization.
 */

import { createStore } from '../infrastructure/storage.js';

const RETENTION_DAYS = 30;

const store = createStore('keyword_history', {
  version: 1,
  migrations: {},
  defaultValue: () => ({ _version: 1, days: {} }),
});

/**
 * Get today's date key in YYYY-MM-DD format.
 * @private
 */
function todayKey() {
  return new Date().toISOString().slice(0, 10);
}

/**
 * Record keyword occurrences from a batch of stories.
 * Call this each time new stories are fetched.
 *
 * Keywords are read from story.tags (already extracted by adapters
 * via keyword_engine). This function does NOT re-tokenize titles.
 *
 * @param {import('../models/story.js').Story[]} stories
 */
export function recordStories(stories) {
  const data = store.read();
  const today = todayKey();

  if (!data.days[today]) {
    data.days[today] = {};
  }

  for (const story of stories) {
    if (!Array.isArray(story.tags)) continue;
    for (const keyword of story.tags) {
      data.days[today][keyword] = (data.days[today][keyword] || 0) + 1;
    }
  }

  // Prune days older than RETENTION_DAYS to prevent unbounded growth
  const cutoff = new Date();
  cutoff.setDate(cutoff.getDate() - RETENTION_DAYS);
  const cutoffKey = cutoff.toISOString().slice(0, 10);

  for (const day of Object.keys(data.days)) {
    if (day < cutoffKey) {
      delete data.days[day];
    }
  }

  store.write(data);
}

/**
 * Detect currently trending keywords by comparing today's frequency
 * against the rolling average of the past N days.
 *
 * A keyword is "trending" if today's count >= spikeThreshold × average.
 *
 * @param {{ windowDays?: number, spikeThreshold?: number }} [options]
 * @returns {{ keyword: string, count: number, changePercent: number }[]}
 *   Sorted by changePercent descending.
 */
export function getTrending({ windowDays = 7, spikeThreshold = 2 } = {}) {
  const data = store.read();
  const today = todayKey();
  const todayCounts = data.days[today];

  if (!todayCounts) return [];

  const pastDays = Object.keys(data.days)
    .filter(d => d !== today)
    .sort()
    .slice(-windowDays);

  // Need at least 1 day of history to detect a spike
  if (pastDays.length === 0) return [];

  const trending = [];

  for (const [keyword, todayCount] of Object.entries(todayCounts)) {
    const pastCounts = pastDays.map(d => (data.days[d] || {})[keyword] || 0);
    const avg = pastCounts.reduce((a, b) => a + b, 0) / pastCounts.length;

    if (avg > 0 && todayCount >= avg * spikeThreshold) {
      const changePercent = Math.round(((todayCount - avg) / avg) * 100);
      trending.push({ keyword, count: todayCount, changePercent });
    }
  }

  return trending.sort((a, b) => b.changePercent - a.changePercent);
}

/**
 * Get historical frequency data for a specific keyword.
 *
 * @param {string} keyword
 * @returns {{ date: string, count: number }[]} Sorted by date ascending.
 */
export function getHistory(keyword) {
  const data = store.read();
  return Object.keys(data.days)
    .sort()
    .map(date => ({
      date,
      count: (data.days[date] || {})[keyword] || 0,
    }));
}
