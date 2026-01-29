/**
 * Blind Spots view controller — stub.
 *
 * Will be implemented in Phase 3 with knowledge_store integration.
 * For now, renders a placeholder explaining the feature.
 */

export const BlindSpotsView = {
  async init(el) {
    el.innerHTML = `
      <div class="view-placeholder">
        <h2>Blind Spots</h2>
        <p>Personal knowledge tracking &mdash; coming in Phase 3.</p>
        <p>This view will show topics you haven't explored yet, sorted by frequency,
           so you can actively close knowledge gaps.</p>
      </div>
    `;
  },

  destroy() {},
};
