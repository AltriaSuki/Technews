/**
 * App shell — initialization and view routing only.
 *
 * This module must NOT import from services/, adapters/, or infrastructure/.
 * It delegates all logic to view controllers.
 */

import './style.css';
import { FeedView } from './views/feed.js';
import { BlindSpotsView } from './views/blindspots.js';
import { TimelineView } from './views/timeline.js';

const VIEWS = {
  feed: FeedView,
  blindspots: BlindSpotsView,
  timeline: TimelineView,
};

let activeView = null;
let activeViewName = null;

function navigateTo(viewName) {
  if (viewName === activeViewName) return;

  const viewContainer = document.getElementById('view-container');
  if (!viewContainer) return;

  if (activeView) {
    activeView.destroy();
  }

  viewContainer.innerHTML = '';
  activeViewName = viewName;
  activeView = VIEWS[viewName];

  if (activeView) {
    activeView.init(viewContainer);
  }

  // Update nav tab active state
  document.querySelectorAll('.nav-tab').forEach(tab => {
    tab.classList.toggle('active', tab.dataset.view === viewName);
  });

  // Clear search when switching views
  const searchInput = document.getElementById('search-input');
  if (searchInput) {
    searchInput.value = '';
  }
}

document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('.nav-tab').forEach(tab => {
    tab.addEventListener('click', () => {
      navigateTo(tab.dataset.view);
    });
  });

  navigateTo('feed');
});
