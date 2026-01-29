import { getTrending } from '../services/trend_engine.js';

/**
 * Creates the Trending Bar component.
 *
 * @param {Object} props
 * @param {Function} props.onSelect - Callback when a trend is clicked (keyword) => void
 * @returns {HTMLElement} The .trending-bar element
 */
export function createTrendingBar({ onSelect }) {
    const container = document.createElement('div');
    container.className = 'trending-bar';

    // Initial render
    render(container, onSelect);

    return container;
}

function render(container, onSelect) {
    const trends = getTrending();

    if (trends.length === 0) {
        container.innerHTML = '';
        container.style.display = 'none';
        return;
    }

    container.style.display = 'block';

    // Header
    const label = document.createElement('div');
    label.className = 'trending-label';
    label.innerHTML = `<span>🔥 Trending Now</span>`;

    // Content container
    const content = document.createElement('div');
    content.className = 'trending-content';

    trends.forEach(trend => {
        const chip = document.createElement('button');
        chip.className = 'trending-chip';
        chip.title = `${trend.count} stories today`;
        chip.dataset.keyword = trend.keyword;

        // Show flame icon if growth is explosive (>3x)
        const icon = trend.growth > 3 ? '⚡' : '';

        chip.innerHTML = `
            ${icon} #${trend.keyword}
            <span class="trending-pct">+${Math.round(trend.growth * 100)}%</span>
        `;

        chip.addEventListener('click', () => {
            // Visual feedback
            document.querySelectorAll('.trending-chip').forEach(c => c.classList.remove('active'));
            chip.classList.add('active');

            if (onSelect) onSelect(trend.keyword);
        });

        content.appendChild(chip);
    });

    container.innerHTML = '';
    container.appendChild(label);
    container.appendChild(content);
}
