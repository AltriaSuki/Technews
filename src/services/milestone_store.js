/**
 * Milestone Store Service
 *
 * Manages a list of "Tech Milestones" - significant events to track on a timeline.
 *
 * Data Model:
 * {
 *   id: string (uuid),
 *   title: string,
 *   date: string (ISO date),
 *   description: string,
 *   type: 'release' | 'announcement' | 'personal',
 *   tags: string[]
 * }
 */

import { createStore } from '../infrastructure/storage.js';

const STORAGE_KEY = 'milestones';

const store = createStore(STORAGE_KEY, {
    version: 1,
    defaultValue: () => ({
        items: [
            // seed some example data
            {
                id: 'seed-1',
                title: 'GPT-4 Release',
                date: '2023-03-14',
                description: 'OpenAI released GPT-4.',
                type: 'release',
                tags: ['ai', 'gpt-4']
            },
            {
                id: 'seed-2',
                title: 'React 19 Announcement',
                date: '2024-04-25',
                description: 'React team announced roadmap for React 19.',
                type: 'announcement',
                tags: ['web', 'react']
            }
        ]
    }),
});

export function getMilestones() {
    const data = store.read();
    // Return sorted by date descending
    return [...data.items].sort((a, b) => new Date(b.date) - new Date(a.date));
}

export function addMilestone(milestone) {
    const data = store.read();
    const newId = crypto.randomUUID();
    data.items.push({
        ...milestone,
        id: newId
    });
    store.write(data);
    return newId;
}

export function deleteMilestone(id) {
    const data = store.read();
    data.items = data.items.filter(i => i.id !== id);
    store.write(data);
}
