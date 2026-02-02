/**
 * Story — the central data contract for Tech Pulse.
 *
 * Every module that produces, consumes, or transforms news content
 * uses Story objects created through createStory(). No raw API response
 * or ad-hoc object may cross module boundaries.
 *
 * This module is FROZEN: it must not import any other project module.
 */

export const SOURCES = Object.freeze({
  HACKERNEWS: 'hackernews',
  GITHUB: 'github',
  REDDIT: 'reddit',
  PRODUCTHUNT: 'producthunt',
  ARXIV: 'arxiv',
});

const VALID_SOURCES = new Set(Object.values(SOURCES));

/**
 * Creates an immutable Story object.
 *
 * @param {Object} fields
 * @param {string} fields.id            - Source-prefixed unique ID (e.g., "hn:12345")
 * @param {string} fields.title         - Display title
 * @param {string} [fields.url]         - Link to original content
 * @param {string} fields.source        - One of SOURCES values
 * @param {number} [fields.score]       - Normalized score 0–100
 * @param {number} [fields.comments]    - Comment/discussion count
 * @param {string} [fields.author]      - Author name or handle
 * @param {number} [fields.timestamp]   - Unix timestamp (seconds)
 * @param {string[]} [fields.tags]      - Extracted keywords (via keyword_engine)
 * @param {string} [fields.discussionUrl] - Link to discussion page
 * @returns {Readonly<Object>}
 */
export function createStory({ id, title, url, source, score, comments, author, timestamp, tags, discussionUrl, summary }) {
  if (typeof id !== 'string' || id.length === 0) {
    throw new Error(`Story: id must be a non-empty string, got "${id}"`);
  }
  if (typeof title !== 'string' || title.length === 0) {
    throw new Error(`Story: title must be a non-empty string, got "${title}"`);
  }
  if (!VALID_SOURCES.has(source)) {
    throw new Error(`Story: source must be one of [${[...VALID_SOURCES].join(', ')}], got "${source}"`);
  }

  return Object.freeze({
    id,
    title,
    url: typeof url === 'string' ? url : '',
    source,
    score: clampInt(score, 0, 100),
    comments: clampInt(comments, 0, Infinity),
    author: typeof author === 'string' && author.length > 0 ? author : 'unknown',
    timestamp: typeof timestamp === 'number' ? timestamp : Math.floor(Date.now() / 1000),
    tags: Object.freeze(Array.isArray(tags) ? [...tags] : []),
    discussionUrl: typeof discussionUrl === 'string' ? discussionUrl : '',
    summary: typeof summary === 'string' ? summary : '',
  });
}

/**
 * Validates that an object conforms to the Story shape.
 * @param {any} obj
 * @returns {boolean}
 */
export function isStory(obj) {
  return (
    obj !== null &&
    typeof obj === 'object' &&
    typeof obj.id === 'string' &&
    typeof obj.title === 'string' &&
    VALID_SOURCES.has(obj.source) &&
    typeof obj.score === 'number' &&
    Array.isArray(obj.tags)
  );
}

function clampInt(value, min, max) {
  if (typeof value !== 'number' || isNaN(value)) return min;
  return Math.max(min, Math.min(max, Math.round(value)));
}
