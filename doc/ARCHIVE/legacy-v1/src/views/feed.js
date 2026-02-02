/**
 * Feed view controller.
 *
 * Coordinates: aggregator (data), trend_engine (analysis),
 * NewsCard component (rendering).
 *
 * Owns the feed-specific UI: category filters, trending bar, story grid.
 * Must not import other view controllers.
 */

import { fetchAll } from '../services/aggregator.js';
import { recordStories, getTrending } from '../services/trend_engine.js';
import { settingsStore } from '../services/settings_store.js';
import { markAsRead } from '../services/knowledge_store.js';
import { createNewsCard } from '../components/NewsCard.js';
import { createTrendingBar } from '../components/TrendingBar.js';

// ... (Categories definition) ...
const CATEGORIES = {
  all: [],
  AI: [
    'ai', 'gpt', 'gpt-4', 'gpt-5', 'llm', 'chatgpt', 'claude',
    'gemini', 'model', 'neural', 'machine-learning', 'deep-learning',
    'generative-ai', 'openai', 'anthropic', 'deepseek', 'transformer',
    'training', 'inference', 'agent', 'rag',
  ],
  Web: [
    'css', 'html', 'javascript', 'typescript', 'react', 'vue', 'svelte',
    'web-development', 'browser', 'frontend', 'backend', 'node', 'next.js', 'api',
    'http', 'wasm', 'webassembly',
  ],
  Hardware: [
    'chip', 'apple', 'nvidia', 'intel', 'amd', 'processor', 'hardware',
    'device', 'phone', 'gpu', 'cpu', 'arm', 'risc-v',
  ],
  Science: [
    'space', 'physics', 'energy', 'quantum', 'science', 'math',
    'research', 'biology', 'chemistry', 'climate', 'nasa',
  ],
};

let state = {
  stories: [],
  searchQuery: '',
  currentCategory: 'all',
  trending: [],
};

let container = null;
let cleanupFns = [];

export const FeedView = {
  async init(el) {
    container = el;
    state = { stories: [], searchQuery: '', currentCategory: 'all', trending: [] };
    cleanupFns = [];

    renderShell();
    bindEvents();

    try {
      state.stories = await fetchAll();

      // 1. Process Trends from new stories
      recordStories(state.stories);

      // 2. Get latest trends
      // We read settings just in case threshold changed
      // const settings = settingsStore.getAll();
      state.trending = getTrending();

      // 3. Render full view
      render();
    } catch (error) {
      console.error('[view:feed] Init failed:', error);
      const grid = container.querySelector('.news-grid');
      if (grid) grid.innerHTML = '<div class="loading-state">Error loading content.</div>';
    }
  },

  destroy() {
    for (const fn of cleanupFns) fn();
    cleanupFns = [];
    state = { stories: [], searchQuery: '', currentCategory: 'all', trending: [] };
    container = null;
  },
};

/**
 * Render the static shell (filter chips, empty grid).
 * Called once during init before data is available.
 */
function renderShell() {
  container.innerHTML = `
    <div class="feed-controls">
      <div class="feed-filters">
        ${Object.keys(CATEGORIES).map(cat => `
          <button class="filter-chip${cat === 'all' ? ' active' : ''}" data-category="${cat}">
            ${cat === 'all' ? 'All' : cat === 'AI' ? 'AI &amp; ML' : cat === 'Web' ? 'Web Dev' : cat}
          </button>
        `).join('')}
      </div>
    </div>
    <div class="trending-bar-container"></div>
    <div class="news-grid">
      <div class="loading-state">Loading latest tech news...</div>
    </div>
  `;
}

/**
 * Bind event listeners using event delegation on the container.
 */
function bindEvents() {
  // Container-level click delegation for filter chips and trending chips
  const handleClick = (e) => {
    const filterChip = e.target.closest('.filter-chip');
    if (filterChip && container.contains(filterChip)) {
      container.querySelectorAll('.filter-chip').forEach(c => c.classList.remove('active'));
      filterChip.classList.add('active');
      state.currentCategory = filterChip.dataset.category;
      renderStories();
      return;
    }

    const trendChip = e.target.closest('.trending-chip');
    if (trendChip && container.contains(trendChip)) {
      const searchInput = document.getElementById('search-input');
      if (searchInput) {
        searchInput.value = trendChip.dataset.keyword;
        searchInput.dispatchEvent(new Event('input'));
      }
      return;
    }
  };

  container.addEventListener('click', handleClick);
  cleanupFns.push(() => container.removeEventListener('click', handleClick));

  // Global search input (lives in the header, shared across views)
  const searchInput = document.getElementById('search-input');
  if (searchInput) {
    const handleSearch = (e) => {
      state.searchQuery = e.target.value.toLowerCase();
      renderStories();
    };
    searchInput.addEventListener('input', handleSearch);
    cleanupFns.push(() => searchInput.removeEventListener('input', handleSearch));
  }
}

/**
 * Render both trending bar and story grid.
 */
function render() {
  renderTrending();
  renderStories();
}

/**
 * Render the trending bar. Shows nothing if no trends are detected.
 */
function renderTrending() {
  const barContainer = container.querySelector('.trending-bar-container');
  if (!barContainer) return;

  barContainer.innerHTML = '';

  if (state.trending.length === 0) {
    return;
  }

  const trendingBar = createTrendingBar({
    onSelect: (keyword) => {
      // Set search input
      const searchInput = document.getElementById('search-input');
      if (searchInput) {
        searchInput.value = keyword;
        searchInput.dispatchEvent(new Event('input'));
      }
    }
  });

  barContainer.appendChild(trendingBar);
}

/**
 * Filter and render stories into the grid.
 */
function renderStories() {
  const grid = container.querySelector('.news-grid');
  if (!grid) return;

  const filtered = filterStories(state.stories);

  if (filtered.length === 0) {
    grid.innerHTML = '<div class="loading-state">No stories found matching your criteria.</div>';
    return;
  }

  grid.innerHTML = '';
  for (const story of filtered) {
    grid.appendChild(createNewsCard(story, {
      onRead: (s) => markAsRead(s)
    }));
  }
}

/**
 * Apply search query and category filter to stories.
 * @param {object[]} stories
 * @returns {object[]}
 */
function filterStories(stories) {
  return stories.filter(story => {
    // Search: match against title text and tags
    const matchSearch = !state.searchQuery
      || story.title.toLowerCase().includes(state.searchQuery)
      || story.tags.some(t => t.includes(state.searchQuery));

    // Category: match against story tags
    let matchCategory = true;
    if (state.currentCategory !== 'all') {
      const categoryKeywords = CATEGORIES[state.currentCategory] || [];
      matchCategory = story.tags.some(t => categoryKeywords.includes(t));
    }

    return matchSearch && matchCategory;
  });
}
