/**
 * Blind Spots View
 *
 * Visualizes the user's knowledge profile vs available tech topics.
 * Highlights areas the user is neglecting (Blind Spots).
 */

import { getTagProfile } from '../services/knowledge_store.js';

export const BlindSpotsView = {
  async init(el) {
    el.innerHTML = `
      <div class="blind-spots-container" style="padding: 2rem; color: var(--text-secondary); text-align: center;">
        <h2 style="color: var(--text-primary); margin-bottom: 1rem;">Your Tech Blind Spots</h2>
        <div id="radar-chart" class="chart-container">
           <!-- Placeholder for visualization -->
           <div class="stat-card">
              <h3>Reading Profile</h3>
              <p>Start reading stories to build your profile!</p>
           </div>
        </div>
        <div id="blind-spot-list" style="margin-top: 2rem; text-align: left; max-width: 600px; margin-left: auto; margin-right: auto;">
        </div>
      </div>
    `;

    renderAnalysis(el);
  },

  destroy() {
    // Cleanup if needed
  },
};

function renderAnalysis(container) {
  const profile = getTagProfile();
  const totalReads = Object.values(profile).reduce((a, b) => a + b, 0);

  if (totalReads < 5) {
    return; // Not enough data
  }

  const list = container.querySelector('#blind-spot-list');
  const chart = container.querySelector('.stat-card');

  // Simple analysis: Check major categories (defined in FeedView, but duplicated here for simplicity or should be shared)
  // For now, let's just show top read tags vs potential missing ones.
  // We'll define some "Core Tech Pillars" to check against.
  const PILLARS = ['ai', 'web-development', 'crypto', 'mobile', 'devops', 'security', 'hardware'];

  const blindSpots = PILLARS.filter(p => !profile[p] || profile[p] < 2);

  chart.innerHTML = `
    <h3>Profile Strength</h3>
    <div style="font-size: 3rem; font-weight: 800; color: var(--accent-primary); margin: 1rem 0;">
        ${Math.min(100, totalReads * 2)}<span style="font-size: 1rem;"> XP</span>
    </div>
    <p>You have read <strong>${totalReads}</strong> stories.</p>
  `;

  if (blindSpots.length > 0) {
    list.innerHTML = `
        <h3 style="color: var(--accent-secondary); margin-bottom: 1rem;">⚠️ Detected Blind Spots</h3>
        <p>You are missing out on updates in these core areas:</p>
        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; margin-top: 1rem;">
            ${blindSpots.map(s => `
                <span style="
                    border: 1px solid var(--accent-secondary); 
                    color: var(--accent-secondary);
                    padding: 4px 12px;
                    border-radius: 100px;
                    font-size: 0.9rem;
                ">
                    ${s}
                </span>
            `).join('')}
        </div>
      `;
  } else {
    list.innerHTML = `
        <h3 style="color: #4ade80;">All Clear!</h3>
        <p>You have a well-rounded reading diet across core tech pillars.</p>
      `;
  }
}
