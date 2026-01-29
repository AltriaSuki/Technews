/**
 * Trend Engine Service
 *
 * Tracks keyword frequency over time to identify what's "trending".
 *
 * Responsibilities:
 * - Record daily keyword counts from newly fetched stories.
 * - Detect spikes by comparing Today's Count vs. 7-Day Average.
 * - Persist history to localStorage (lightweight).
 */

import { createStore } from '../infrastructure/storage.js';
import { settingsStore } from './settings_store.js';

// Configuration
const STORAGE_KEY = 'keyword_history';

// Store definition
const store = createStore(STORAGE_KEY, {
  version: 1,
  defaultValue: () => ({
    history: {}, // { "dateStr": { "keyword": count } }
    lastProcessed: 0, // Timestamp of last run
  }),
});

/**
 * Record keywords from a batch of stories.
 * Call this after fetching stories for the feed.
 *
 * @param {import('../models/story.js').Story[]} stories
 */
export function recordStories(stories) {
  if (!stories || stories.length === 0) return;

  const data = store.read();
  const today = new Date().toISOString().split('T')[0];

  // Initialize today's bucket if needed
  if (!data.history[today]) {
    data.history[today] = {};
    // Cleanup old history (keep last 30 days)
    cleanupHistory(data);
  }

  // Tally keywords
  stories.forEach(story => {
    if (!story.tags) return;
    story.tags.forEach(tag => {
      const normalized = tag.toLowerCase();
      data.history[today][normalized] = (data.history[today][normalized] || 0) + 1;
    });
  });

  data.lastProcessed = Date.now();
  store.write(data);
}

/**
 * cleanupHistory - remove entries older than 30 days to save space
 */
function cleanupHistory(data) {
  const dates = Object.keys(data.history).sort();
  if (dates.length > 30) {
    const toRemove = dates.slice(0, dates.length - 30);
    toRemove.forEach(date => delete data.history[date]);
  }
}

/**
 * Get the list of currently trending topics.
 *
 * @returns {Array<{ keyword: string, growth: number, count: number }>}
 */
export function getTrending() {
  const settings = settingsStore.getAll();
  const threshold = settings.trendingSpikeThreshold || 2; // e.g. 2x growth
  // Use a simpler approach: Minimum occurrences to even consider (noise filter)
  const MIN_COUNT = 3;

  const data = store.read();
  const today = new Date().toISOString().split('T')[0];
  const todayCounts = data.history[today] || {};

  const dates = Object.keys(data.history).filter(d => d !== today).sort().reverse().slice(0, 7);

  // Calculate 7-day average for each keyword found today
  const trends = [];

  Object.entries(todayCounts).forEach(([keyword, count]) => {
    if (count < MIN_COUNT) return;

    let historicalSum = 0;
    let daysPresent = 0;

    dates.forEach(date => {
      if (data.history[date] && data.history[date][keyword]) {
        historicalSum += data.history[date][keyword];
        daysPresent++;
      }
    });

    // Avoid divide by zero. If never seen before, average is small (0.5) to enable "new" spikes
    const average = daysPresent > 0 ? historicalSum / daysPresent : 0.5;

    const growth = count / average;

    if (growth >= threshold) {
      trends.push({
        keyword,
        growth: parseFloat(growth.toFixed(1)),
        count
      });
    }
  });

  // Sort by growth (explosiveness) then by raw count
  return trends.sort((a, b) => b.growth - a.growth || b.count - a.count).slice(0, 10);
}
