/**
 * Timeline View
 *
 * Displays a vertical timeline of tech milestones.
 * Allows adding new milestones via a form.
 */

import { getMilestones, addMilestone, deleteMilestone } from '../services/milestone_store.js';

export const TimelineView = {
  async init(el) {
    el.innerHTML = `
      <div class="timeline-layout" style="max-width: 800px; margin: 0 auto; padding: 2rem;">
        <div class="timeline-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;">
            <h2 style="font-family: var(--font-heading);">Tech Timeline</h2>
            <button id="btn-add-milestone" style="
                background: hsl(var(--accent-primary));
                color: white;
                border: none;
                padding: 8px 16px;
                border-radius: 100px;
                cursor: pointer;
                font-weight: 600;
            ">+ Add Event</button>
        </div>

        <div id="add-form-container" style="
            background: var(--glass-bg);
            border: 1px solid var(--glass-border);
            padding: 1.5rem;
            border-radius: 12px;
            margin-bottom: 2rem;
            display: none;
        ">
            <!-- Form injected here -->
        </div>

        <div class="timeline-feed" style="position: relative; border-left: 2px solid var(--glass-border); padding-left: 2rem;">
            <!-- Timeline items -->
        </div>
      </div>
    `;

    bindEvents(el);
    renderTimeline(el);
  },

  destroy() {
    // Cleanup
  },
};

function bindEvents(container) {
  const btnAdd = container.querySelector('#btn-add-milestone');
  const formContainer = container.querySelector('#add-form-container');

  btnAdd.addEventListener('click', () => {
    const isVisible = formContainer.style.display === 'block';
    formContainer.style.display = isVisible ? 'none' : 'block';
    if (!isVisible) renderForm(formContainer, container);
  });
}

function renderForm(formContainer, mainContainer) {
  formContainer.innerHTML = `
        <form id="milestone-form" style="display: grid; gap: 1rem;">
            <input type="text" name="title" placeholder="Event Title" required style="
                background: rgba(255,255,255,0.05); border: 1px solid var(--glass-border); padding: 10px; color: white; border-radius: 6px;
            ">
            <input type="date" name="date" required style="
                background: rgba(255,255,255,0.05); border: 1px solid var(--glass-border); padding: 10px; color: white; border-radius: 6px;
            ">
            <textarea name="description" placeholder="Description" style="
                background: rgba(255,255,255,0.05); border: 1px solid var(--glass-border); padding: 10px; color: white; border-radius: 6px; min-height: 80px;
            "></textarea>
            <div style="display: flex; gap: 1rem;">
                <button type="submit" style="
                    background: hsl(var(--accent-secondary)); color: black; border: none; padding: 8px 16px; border-radius: 6px; cursor: pointer; font-weight: 600;
                ">Save Event</button>
                <button type="button" id="btn-cancel" style="
                    background: transparent; border: 1px solid var(--glass-border); color: var(--text-secondary); padding: 8px 16px; border-radius: 6px; cursor: pointer;
                ">Cancel</button>
            </div>
        </form>
    `;

  const form = formContainer.querySelector('#milestone-form');
  form.addEventListener('submit', (e) => {
    e.preventDefault();
    const formData = new FormData(form);
    addMilestone({
      title: formData.get('title'),
      date: formData.get('date'),
      description: formData.get('description'),
      type: 'manual',
      tags: []
    });

    formContainer.style.display = 'none';
    renderTimeline(mainContainer);
  });

  formContainer.querySelector('#btn-cancel').addEventListener('click', () => {
    formContainer.style.display = 'none';
  });
}

function renderTimeline(container) {
  const list = container.querySelector('.timeline-feed');
  const items = getMilestones();

  if (items.length === 0) {
    list.innerHTML = `<div style="color: var(--text-secondary);">No events recorded.</div>`;
    return;
  }

  list.innerHTML = items.map(item => `
        <div class="timeline-item" style="position: relative; margin-bottom: 2rem;">
            <div class="timeline-dot" style="
                position: absolute;
                left: -2.4rem;
                top: 0.2rem;
                width: 12px;
                height: 12px;
                background: hsl(var(--accent-primary));
                border-radius: 50%;
                box-shadow: 0 0 10px hsl(var(--accent-primary));
            "></div>
            <div class="item-date" style="font-size: 0.85rem; color: var(--accent-secondary); margin-bottom: 0.25rem;">
                ${item.date}
            </div>
            <h3 style="font-size: 1.2rem; margin-bottom: 0.5rem; color: var(--text-primary);">${item.title}</h3>
            <p style="color: var(--text-secondary); line-height: 1.6;">${item.description}</p>
        </div>
    `).join('');
}
