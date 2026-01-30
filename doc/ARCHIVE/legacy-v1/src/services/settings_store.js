/**
 * Settings store — persists user preferences.
 *
 * Owns: techpulse_settings localStorage key.
 * No other module may read or write this key.
 *
 * Settings are read by other services (aggregator checks enabled sources,
 * trend_engine reads threshold config). This store only persists and
 * retrieves — it does not interpret.
 */

import { createStore } from '../infrastructure/storage.js';

const DEFAULT_SETTINGS = Object.freeze({
  enabledSources: ['hackernews', 'github', 'reddit', 'producthunt', 'arxiv'],
  trendingWindowDays: 7,
  trendingSpikeThreshold: 2,
  storiesPerSource: 30,
  translationEnabled: false,
  translationTargetLang: 'zh',
  translationProvider: 'libre',
  translationEndpoint: 'https://libretranslate.com',
  translationApiKey: '',
});

const store = createStore('settings', {
  version: 2,
  migrations: {
    2: (data) => {
      // Add new sources to enabledSources if they aren't there
      const newSources = ['github', 'reddit', 'producthunt', 'arxiv'];
      const current = new Set(data.enabledSources || []);
      newSources.forEach(s => current.add(s));
      data.enabledSources = [...current];
      return data;
    },
  },
  defaultValue: () => ({ _version: 2, ...DEFAULT_SETTINGS }),
});

export const settingsStore = Object.freeze({
  /**
   * Get a single setting value.
   * @param {string} key
   * @returns {any} The setting value, or the default if not set.
   */
  get(key) {
    const data = store.read();
    return key in data ? data[key] : DEFAULT_SETTINGS[key];
  },

  /**
   * Set a single setting value.
   * @param {string} key
   * @param {any} value
   */
  set(key, value) {
    const data = store.read();
    data[key] = value;
    store.write(data);
  },

  /**
   * Get all settings merged with defaults.
   * @returns {Object}
   */
  getAll() {
    const data = store.read();
    return { ...DEFAULT_SETTINGS, ...data };
  },
});
