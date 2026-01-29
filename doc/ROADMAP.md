# Tech Pulse — Project Roadmap

## Mission

Turn Tech Pulse from a passive HN reader into an **active tech-awareness system** that prevents you from missing important developments like you missed AI agents from 2023–2025.

---

## Current State

- Vanilla JS + Vite, zero dependencies
- Single data source: Hacker News top 60 stories
- Keyword-based category filtering (AI, Web, Hardware, Science)
- Search by title
- No persistence, no history, no intelligence

**The core problem:** This app shows you what's popular *right now* but doesn't tell you **what's gaining momentum**, **what's new that you haven't seen**, or **what you're missing**.

---

## Architecture Evolution

```
Current:
  HN API → stories[] → keyword filter → render cards

Target:
  Multiple APIs → normalized stories[] → trend engine → render views
                                        → knowledge tracker (localStorage)
                                        → timeline builder
```

**Key decisions:**
- Stay vanilla JS — no framework needed for this scope
- Use `localStorage` for persistence (knowledge state, seen topics, history)
- All processing client-side — no backend required
- New API services follow the same pattern as `hn_api.js`

---

## Phase 1: Multi-Source Aggregation

**Goal:** Reduce blind spots by pulling from multiple tech communities, not just HN.

### New data sources

| Source | API | What it adds |
|---|---|---|
| GitHub Trending | Scrape via unofficial API or use `gh-trending-api` | Emerging tools/repos you'd miss on HN |
| Reddit (r/programming, r/MachineLearning) | Reddit JSON API (append `.json` to any subreddit URL) | Broader community discussions |
| Product Hunt | Product Hunt API | New product launches |
| ArXiv (CS) | ArXiv API (Atom feed) | Research papers, bleeding edge |

### Implementation

1. **Create unified story interface:**
   ```js
   // src/models/Story.js
   {
     id: string,          // source-prefixed: "hn-123", "gh-repo-name"
     title: string,
     url: string,
     source: 'hackernews' | 'github' | 'reddit' | 'producthunt' | 'arxiv',
     score: number,        // normalized 0-100
     commentCount: number,
     author: string,
     timestamp: number,    // unix
     tags: string[],       // extracted keywords
   }
   ```

2. **New service files:**
   ```
   src/services/
   ├── hn_api.js          (exists)
   ├── github_api.js      (new — GitHub trending)
   ├── reddit_api.js      (new — subreddit feeds)
   ├── producthunt_api.js (new — PH daily)
   ├── arxiv_api.js       (new — CS papers)
   └── aggregator.js      (new — merges all sources, normalizes scores)
   ```

3. **Add source filter chips** alongside category chips:
   - "All Sources" | "HN" | "GitHub" | "Reddit" | "PH" | "ArXiv"

4. **Update NewsCard** to show source icon/color per platform.

### Files to change
- `src/services/` — add 5 new files
- `src/models/Story.js` — new, defines normalized shape
- `src/components/NewsCard.js` — add source indicator
- `src/components/NewsCard.css` — source badge colors
- `index.html` — add source filter row
- `src/main.js` — use aggregator instead of `hnApi` directly

---

## Phase 2: Trending Detection

**Goal:** Automatically surface topics that are suddenly appearing more often — the exact thing that would have alerted you to "agents" in 2023–2024.

### How it works

1. **Keyword extraction:** Extract meaningful terms from every story title using simple NLP (split by space, remove stopwords, normalize).

2. **Frequency tracking:** Store keyword frequency per day in `localStorage`:
   ```js
   // localStorage: "techpulse_keyword_history"
   {
     "2026-01-29": { "agent": 12, "mcp": 5, "rust": 8, ... },
     "2026-01-28": { "agent": 4, "mcp": 2, "rust": 9, ... },
     ...
   }
   ```

3. **Spike detection:** Compare today's frequency vs. 7-day rolling average. If a keyword appears 2x+ more than average → it's **trending**.

4. **Trending bar UI:** A horizontal bar above the news grid showing trending keywords as clickable chips:
   ```
   🔥 Trending: [agent +200%] [mcp +150%] [deepseek +300%]
   ```
   Clicking a chip filters the feed to stories containing that keyword.

### Implementation

1. **New module:**
   ```
   src/services/
   └── trend_engine.js    (keyword extraction, frequency storage, spike detection)
   ```

2. **New component:**
   ```
   src/components/
   └── TrendingBar.js     (renders trending chips)
   └── TrendingBar.css
   ```

3. **Data flow:**
   - On each fetch, pass stories to `trend_engine.recordKeywords(stories)`
   - `trend_engine.getTrending()` returns `[{ keyword, changePercent, count }]`
   - `TrendingBar` renders the result
   - Clicking a trend chip sets `App.searchQuery` to that keyword

### Files to change
- `src/services/trend_engine.js` — new
- `src/components/TrendingBar.js` — new
- `src/components/TrendingBar.css` — new
- `src/main.js` — integrate trend engine into init/render cycle
- `index.html` — add trending bar container between header and grid

---

## Phase 3: Personal Knowledge Tracker

**Goal:** Mark topics by your familiarity level. The app highlights what you haven't explored so you can actively close knowledge gaps.

### How it works

1. **Topic extraction:** Reuse keyword extraction from Phase 2 to identify recurring topics.

2. **Knowledge states:**
   ```
   🟢 "Know it"     — you've used or deeply understand this
   🟡 "Heard of it" — you've seen the name but haven't explored
   🔴 "Never seen"  — first time appearing (default for new keywords)
   ⭐ "Want to learn"— bookmarked for future exploration
   ```

3. **Storage:** `localStorage: "techpulse_knowledge_map"`
   ```js
   {
     "mcp": { status: "never_seen", firstSeen: "2026-01-15", lastSeen: "2026-01-29" },
     "react": { status: "know_it", firstSeen: "2024-03-01", lastSeen: "2026-01-29" },
     "deepseek": { status: "heard_of", firstSeen: "2026-01-20", lastSeen: "2026-01-29" },
   }
   ```

4. **UI integration:**
   - Each NewsCard shows small colored dots for the topic tags it contains, indicating your knowledge level
   - A **"Blind Spots" panel** (sidebar or separate view) lists all `never_seen` and `heard_of` topics sorted by frequency — these are your knowledge gaps
   - Clicking a topic in the panel lets you change its status

5. **New view: Blind Spots Dashboard**
   ```
   ┌─────────────────────────────────────────┐
   │  Your Blind Spots (12 topics)           │
   ├─────────────────────────────────────────┤
   │  🔴 mcp          — 47 stories, trending │
   │  🔴 deepseek-r2  — 23 stories           │
   │  🟡 zig          — 15 stories            │
   │  🟡 bun          — 12 stories            │
   │  ⭐ webgpu       — 9 stories             │
   └─────────────────────────────────────────┘
   ```
   This view directly answers: **"What am I missing?"**

### Implementation

1. **New modules:**
   ```
   src/services/
   └── knowledge_store.js  (CRUD for knowledge map in localStorage)

   src/components/
   └── BlindSpots.js       (blind spots panel/view)
   └── BlindSpots.css
   └── TopicBadge.js       (colored dot for knowledge status on cards)
   └── TopicBadge.css
   ```

2. **Modify NewsCard** to include topic badges.

3. **Add navigation** — tabs or a toggle between "Feed" and "Blind Spots" views.

### Files to change
- `src/services/knowledge_store.js` — new
- `src/components/BlindSpots.js` — new
- `src/components/BlindSpots.css` — new
- `src/components/TopicBadge.js` — new
- `src/components/TopicBadge.css` — new
- `src/components/NewsCard.js` — add topic badges
- `src/main.js` — add view switching, integrate knowledge store
- `index.html` — add nav tabs, blind spots container

---

## Phase 4: Tech Milestone Timeline

**Goal:** Visualize the evolution of technology over time so you can see what happened and when, filling in the gaps you missed.

### How it works

1. **Milestone detection:** Stories that cross a high score threshold (e.g., top 1% of all stories seen) get saved as "milestones."

2. **Manual milestones:** You can also pin any story as a milestone.

3. **Storage:** `localStorage: "techpulse_milestones"`
   ```js
   [
     {
       id: "hn-38345678",
       title: "GPT-4 Released",
       url: "...",
       date: "2023-03-14",
       category: "AI",
       source: "hackernews",
       pinned: false,        // auto-detected
     },
     {
       id: "manual-1",
       title: "Claude Code launched",
       url: "...",
       date: "2025-05-01",
       category: "AI",
       pinned: true,          // user-added
     }
   ]
   ```

4. **Timeline UI:** A vertical timeline view filterable by category:
   ```
   2026 ─── ● DeepSeek-R2 released (AI)
        │
        ├── ● MCP protocol gains adoption (AI)
        │
   2025 ─── ● Claude Code launched (AI)
        │
        ├── ● Vite 6.0 released (Web)
        │
   2024 ─── ● Llama 3 open-sourced (AI)
        │
        ├── ● GPT-4o multimodal (AI)
   ```

5. **Add milestone button** on each NewsCard — click to pin to timeline.

### Implementation

1. **New modules:**
   ```
   src/services/
   └── milestone_store.js  (CRUD for milestones in localStorage)

   src/components/
   └── Timeline.js         (vertical timeline view)
   └── Timeline.css
   └── MilestoneForm.js    (form for manually adding milestones)
   └── MilestoneForm.css
   ```

2. **Modify NewsCard** to add a "pin to timeline" button.

3. **Add "Timeline" as a third nav tab.**

### Files to change
- `src/services/milestone_store.js` — new
- `src/components/Timeline.js` — new
- `src/components/Timeline.css` — new
- `src/components/MilestoneForm.js` — new
- `src/components/MilestoneForm.css` — new
- `src/components/NewsCard.js` — add pin button
- `src/main.js` — add timeline view, integrate milestone store
- `index.html` — add timeline nav tab, timeline container

---

## File Structure After All Phases

```
src/
├── main.js                      (app controller, view routing)
├── style.css                    (global styles)
├── models/
│   └── Story.js                 (normalized story interface)
├── services/
│   ├── hn_api.js                (exists — Hacker News)
│   ├── github_api.js            (Phase 1 — GitHub trending)
│   ├── reddit_api.js            (Phase 1 — Reddit feeds)
│   ├── producthunt_api.js       (Phase 1 — Product Hunt)
│   ├── arxiv_api.js             (Phase 1 — ArXiv CS papers)
│   ├── aggregator.js            (Phase 1 — merges all sources)
│   ├── trend_engine.js          (Phase 2 — keyword tracking + spike detection)
│   ├── knowledge_store.js       (Phase 3 — personal knowledge map)
│   └── milestone_store.js       (Phase 4 — timeline milestones)
└── components/
    ├── NewsCard.js              (exists — enhanced with badges + pin)
    ├── NewsCard.css             (exists — updated)
    ├── TrendingBar.js           (Phase 2)
    ├── TrendingBar.css          (Phase 2)
    ├── BlindSpots.js            (Phase 3)
    ├── BlindSpots.css           (Phase 3)
    ├── TopicBadge.js            (Phase 3)
    ├── TopicBadge.css           (Phase 3)
    ├── Timeline.js              (Phase 4)
    ├── Timeline.css             (Phase 4)
    ├── MilestoneForm.js         (Phase 4)
    └── MilestoneForm.css        (Phase 4)
```

---

## Phase Dependencies

```
Phase 1 (Multi-Source) ──→ Phase 2 (Trending)
                      └──→ Phase 3 (Knowledge) ──→ Phase 4 (Timeline)
```

- Phase 1 is foundational — more sources = better trending data and broader coverage
- Phase 2 depends on Phase 1 for richer keyword data
- Phase 3 depends on Phase 2 for keyword extraction (reuse)
- Phase 4 can start partially in parallel with Phase 3 but benefits from the full pipeline

---

## localStorage Keys

| Key | Phase | Purpose |
|---|---|---|
| `techpulse_keyword_history` | 2 | Daily keyword frequency counts |
| `techpulse_knowledge_map` | 3 | Personal topic familiarity status |
| `techpulse_milestones` | 4 | Pinned/auto-detected milestones |
| `techpulse_settings` | All | User preferences (sources enabled, thresholds) |

---

## No Backend Required

Everything runs client-side:
- APIs are fetched directly (HN, Reddit `.json`, GitHub unofficial)
- Persistence is `localStorage`
- Trend analysis is simple math on small datasets
- If localStorage limits become an issue, migrate to IndexedDB later

---

## What This Solves

| Your problem | Which phase fixes it |
|---|---|
| "I only saw what I already looked for" | Phase 1 — multiple sources surface different things |
| "I didn't notice 'agents' was becoming a thing" | Phase 2 — trending detection flags spikes automatically |
| "I didn't know what I didn't know" | Phase 3 — blind spots panel shows your knowledge gaps |
| "I missed 2 years of evolution" | Phase 4 — timeline shows what happened and when |
