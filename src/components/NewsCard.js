/**
 * NewsCard component — renders a single Story as a card element.
 *
 * This component receives a Story object and optional callbacks.
 * It must NOT import from services/, adapters/, infrastructure/storage.js, or views/.
 */

import './NewsCard.css';

/**
 * Creates a news card DOM element from a Story object.
 *
 * @param {import('../models/story.js').Story} story
 * @param {{ onPin?: (story: object) => void }} [callbacks]
 * @returns {HTMLElement}
 */
export function createNewsCard(story, callbacks = {}) {
  const card = document.createElement('article');
  card.className = 'news-card';

  const domain = story.url ? extractDomain(story.url) : story.source;
  const timeAgo = getTimeAgo(story.timestamp);
  const storyUrl = story.url || story.discussionUrl;

  card.innerHTML = `
    <div class="card-content">
      <div class="card-meta-top">
        <span class="source-badge">${esc(domain)}</span>
        <span class="score">&#9650; ${story.score}</span>
      </div>
      <h2 class="card-title">
        <a href="${esc(storyUrl)}" target="_blank" rel="noopener noreferrer">
          ${esc(story.title)}
        </a>
      </h2>
      <div class="card-meta-bottom">
        <span class="author">by ${esc(story.author)}</span>
        <span class="time">${esc(timeAgo)}</span>
      </div>
    </div>
    <div class="card-actions">
      ${story.discussionUrl
        ? `<a href="${esc(story.discussionUrl)}" target="_blank" class="comments-link">
            ${story.comments} comments
          </a>`
        : `<span class="comments-link">${story.comments} comments</span>`
      }
    </div>
  `;

  return card;
}

/**
 * Escape HTML to prevent XSS from untrusted content.
 * @param {string} str
 * @returns {string}
 */
function esc(str) {
  if (typeof str !== 'string') return '';
  const el = document.createElement('span');
  el.textContent = str;
  return el.innerHTML;
}

/**
 * Extract domain from URL, stripping "www." prefix.
 * @param {string} url
 * @returns {string}
 */
function extractDomain(url) {
  try {
    return new URL(url).hostname.replace('www.', '');
  } catch {
    return 'unknown';
  }
}

/**
 * Convert a unix timestamp to a human-readable relative time string.
 * @param {number} timestamp - Unix timestamp in seconds
 * @returns {string}
 */
function getTimeAgo(timestamp) {
  const seconds = Math.floor(Date.now() / 1000 - timestamp);
  const intervals = [
    ['year', 31536000],
    ['month', 2592000],
    ['week', 604800],
    ['day', 86400],
    ['hour', 3600],
    ['minute', 60],
  ];

  for (const [unit, secs] of intervals) {
    const interval = Math.floor(seconds / secs);
    if (interval >= 1) {
      return `${interval} ${unit}${interval > 1 ? 's' : ''} ago`;
    }
  }
  return 'just now';
}
