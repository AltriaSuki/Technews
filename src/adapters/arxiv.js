import { createStory, SOURCES } from '../models/story.js';
import { extractKeywords } from '../infrastructure/keyword_engine.js';

const API_BASE = 'http://export.arxiv.org/api/query';

/**
 * Fetch recent CS papers from ArXiv.
 * Categories: AI, SE, LG (Learning), CV (Computer Vision)
 *
 * @param {Object} options
 * @param {number} [options.limit=30]
 * @returns {Promise<import('../models/story.js').Story[]>}
 */
export async function fetchStories({ limit = 30 } = {}) {
    try {
        const query = 'cat:cs.AI OR cat:cs.SE OR cat:cs.LG OR cat:cs.CV';
        // Use CORS proxy to bypass localhost restrictions
        // WARNING: Dependent on third-party proxy (corsproxy.io).
        const PROXY = 'https://corsproxy.io/?';
        const targetUrl = encodeURIComponent(`${API_BASE}?search_query=${encodeURIComponent(query)}&start=0&max_results=${limit}&sortBy=submittedDate&sortOrder=descending`);
        const response = await fetch(`${PROXY}${targetUrl}`);
        if (!response.ok) throw new Error(`ArXiv API error: ${response.status}`);

        const xmlText = await response.text();
        const parser = new DOMParser();
        const xmlDoc = parser.parseFromString(xmlText, 'text/xml');

        const entries = Array.from(xmlDoc.querySelectorAll('entry'));
        return entries.map(mapEntryToStory);
    } catch (error) {
        console.error('[arxiv] Fetch failed:', error);
        return [];
    }
}

function mapEntryToStory(entry) {
    const idUrl = entry.querySelector('id').textContent;
    const id = idUrl.split('/').pop(); // Extract ID from http://arxiv.org/abs/2101.00001
    const title = entry.querySelector('title').textContent.replace(/\n/g, ' ').trim();
    const summary = entry.querySelector('summary').textContent.replace(/\n/g, ' ').trim();
    const published = entry.querySelector('published').textContent;
    const authorNode = entry.querySelector('author name');
    const author = authorNode ? authorNode.textContent : 'ArXiv';
    const link = entry.querySelector('link[title="pdf"]')
        ? entry.querySelector('link[title="pdf"]').getAttribute('href')
        : idUrl;

    return createStory({
        id: `arxiv-${id}`,
        title: `[Paper] ${title}`,
        url: link,
        source: SOURCES.ARXIV,
        score: 60, // No logic for importance yet, standard base score
        comments: 0,
        author: author,
        timestamp: new Date(published).getTime() / 1000,
        tags: extractKeywords(`${title} ${summary}`),
        discussionUrl: idUrl, // Abstract page
    });
}
