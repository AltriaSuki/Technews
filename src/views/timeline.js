/**
 * Timeline view controller — stub.
 *
 * Will be implemented in Phase 4 with milestone_store integration.
 * For now, renders a placeholder explaining the feature.
 */

export const TimelineView = {
  async init(el) {
    el.innerHTML = `
      <div class="view-placeholder">
        <h2>Timeline</h2>
        <p>Tech milestone timeline &mdash; coming in Phase 4.</p>
        <p>This view will visualize major tech developments over time,
           letting you see what happened and when.</p>
      </div>
    `;
  },

  destroy() {},
};
