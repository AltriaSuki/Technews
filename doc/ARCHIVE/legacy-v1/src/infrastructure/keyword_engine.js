/**
 * Shared keyword extraction and normalization engine.
 *
 * This is the ONLY module that performs tokenization, stopword filtering,
 * and alias resolution. All consumers (adapters, trend_engine, knowledge_store)
 * must call these functions instead of implementing their own.
 *
 * This module is STABLE: it must not import any other project module.
 */

const STOPWORDS = new Set([
  // Determiners, pronouns, prepositions, conjunctions
  'the', 'a', 'an', 'is', 'are', 'was', 'were', 'be', 'been', 'being',
  'have', 'has', 'had', 'do', 'does', 'did', 'will', 'would', 'could',
  'should', 'may', 'might', 'can', 'shall', 'to', 'of', 'in', 'for',
  'on', 'with', 'at', 'by', 'from', 'as', 'into', 'through', 'during',
  'before', 'after', 'above', 'below', 'between', 'out', 'off', 'up',
  'down', 'about', 'or', 'and', 'but', 'not', 'no', 'nor', 'so', 'yet',
  'both', 'either', 'neither', 'each', 'every', 'all', 'any', 'few',
  'more', 'most', 'other', 'some', 'such', 'than', 'too', 'very',
  'just', 'because', 'if', 'when', 'while', 'how', 'what', 'which',
  'who', 'whom', 'this', 'that', 'these', 'those', 'it', 'its',
  'i', 'me', 'my', 'we', 'our', 'you', 'your', 'he', 'him', 'his',
  'she', 'her', 'they', 'them', 'their',
  // Common low-signal words in tech headlines
  'new', 'now', 'get', 'got', 'make', 'made', 'way', 'back',
  'show', 'ask', 'tell', 'use', 'using', 'used',
  'why', 'via', 'vs', 'like', 'one', 'two', 'first',
  'also', 'even', 'still', 'already', 'here', 'there',
  'says', 'said', 'lets', 'let', 'see', 'look',
  'need', 'want', 'think', 'know', 'work', 'working',
  'really', 'much', 'many', 'well', 'only', 'over',
  'year', 'years', 'day', 'days', 'time', 'long',
  'part', 'things', 'thing', 'goes', 'going', 'come',
  'better', 'best', 'big', 'small', 'old', 'next',
  'open', 'source', 'free', 'built', 'build', 'building',
  'people', 'world', 'today', 'never', 'keep', 'take',
]);

/**
 * Canonical alias map.
 * Maps variant forms to a single canonical keyword so that trending
 * detection and knowledge tracking treat them as the same topic.
 */
const ALIASES = Object.freeze({
  // AI & ML
  'gpt4': 'gpt-4',
  'gpt-4o': 'gpt-4',
  'gpt4o': 'gpt-4',
  'gpt5': 'gpt-5',
  'llms': 'llm',
  'genai': 'generative-ai',
  'gen-ai': 'generative-ai',
  // Languages & frameworks
  'js': 'javascript',
  'ts': 'typescript',
  'reactjs': 'react',
  'react.js': 'react',
  'vuejs': 'vue',
  'vue.js': 'vue',
  'nodejs': 'node',
  'node.js': 'node',
  'nextjs': 'next.js',
  'next.js': 'next.js',
  'golang': 'go',
  'rustlang': 'rust',
  'py': 'python',
  'cpp': 'c++',
  // Platforms & tools
  'gh': 'github',
  'k8s': 'kubernetes',
  'tf': 'terraform',
  'postgres': 'postgresql',
});

/**
 * Extract meaningful keywords from a text string.
 * Returns a deduplicated array of canonical keywords.
 *
 * @param {string} text
 * @returns {string[]}
 */
export function extractKeywords(text) {
  if (typeof text !== 'string' || text.length === 0) return [];

  // Lowercase, keep only alphanumeric, hyphens, dots, and spaces
  const normalized = text
    .toLowerCase()
    .replace(/[^a-z0-9\s\-\.]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();

  const tokens = normalized.split(' ');
  const keywords = [];

  for (const token of tokens) {
    if (token.length <= 1) continue;
    if (STOPWORDS.has(token)) continue;

    // Strip leading/trailing punctuation artifacts
    const cleaned = token.replace(/^[\-\.]+|[\-\.]+$/g, '');
    if (cleaned.length <= 1) continue;

    const canonical = ALIASES[cleaned] || cleaned;
    keywords.push(canonical);
  }

  return [...new Set(keywords)];
}

/**
 * Normalize a single keyword to its canonical form.
 *
 * @param {string} keyword
 * @returns {string}
 */
export function normalizeKeyword(keyword) {
  if (typeof keyword !== 'string') return '';
  const lower = keyword.toLowerCase().trim();
  return ALIASES[lower] || lower;
}
