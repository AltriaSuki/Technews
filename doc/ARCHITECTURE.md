# Tech Pulse — Architecture Specification

This document defines the structural contracts that govern all implementation.
It must be read before writing any code and updated when boundaries change.

---

## 1. Module Responsibility Table

Each module has an **owner scope** (what it alone is responsible for) and a **forbidden list** (what it must never do). Violations of the forbidden list are architectural defects, not style issues.

### Stability Classification

- **Frozen**: Interface changes require updating this document and all dependents. Extremely rare.
- **Stable**: Interface changes are rare but possible. Internals may change freely.
- **Volatile**: Expected to change frequently. Must be isolated so changes don't propagate.

---

### `models/story.js` — **Frozen**

| | |
|---|---|
| **Owns** | The `Story` shape definition, the `createStory()` factory, the `SOURCES` enum, the `isStory()` validator |
| **Forbidden** | Importing any other project module. Performing I/O. Containing business logic. Mutating returned objects. |
| **Rationale** | Story is the central data contract. Every module downstream depends on this shape. If it imports anything, it creates a circular risk. If it mutates, downstream assumptions break. |

### `infrastructure/storage.js` — **Frozen**

| | |
|---|---|
| **Owns** | The `createStore()` factory that wraps localStorage with versioning and migration |
| **Forbidden** | Knowing about any specific data schema (keywords, milestones, etc.). Importing from services, adapters, views, or components. Defining migration functions (those belong to the store owner). |
| **Rationale** | This is the persistence foundation. It must be agnostic to what is stored. If it knows about specific schemas, replacing localStorage with IndexedDB later will drag in unrelated changes. |

### `infrastructure/keyword_engine.js` — **Stable**

| | |
|---|---|
| **Owns** | Tokenization, stopword filtering, alias/canonical mapping, `extractKeywords()`, `normalizeKeyword()` |
| **Forbidden** | Performing I/O. Accessing storage. Knowing about Story, trends, or knowledge states. |
| **Rationale** | This is shared infrastructure used by adapters (for Story.tags), trend_engine (for frequency tracking), and knowledge_store (for topic identification). If it depends on any consumer, changes become circular. |

### `adapters/hn.js`, `github.js`, `reddit.js`, `producthunt.js`, `arxiv.js` — **Volatile**

| | |
|---|---|
| **Owns** | Fetching from its specific external API. Converting raw API response into `Story[]` via `createStory()`. All source-specific field mapping, URL construction, and error handling. |
| **Forbidden** | Returning anything other than `Story[]` (or empty array on failure). Accessing localStorage or any store. Importing other adapters. Importing view or component modules. Leaking raw API fields beyond its own boundary. |
| **Rationale** | External APIs are the most volatile dependency. When an API changes, only its adapter file should need modification. The rest of the system sees only `Story[]`. |

### `services/aggregator.js` — **Stable**

| | |
|---|---|
| **Owns** | Adapter registry (which adapters are active). Calling adapters in parallel. Merging results. Deduplicating by URL. Sorting by score/timestamp. |
| **Forbidden** | Containing source-specific logic (that belongs in adapters). Accessing localStorage directly. Knowing about trends, knowledge, or milestones. Importing view or component modules. |
| **Rationale** | The aggregator is the single entry point for "get all stories." Its interface is simple: `fetchAll() → Story[]`. Adding a source means adding an adapter and registering it here — no structural change. |

### `services/trend_engine.js` — **Stable interface, volatile internals**

| | |
|---|---|
| **Owns** | The `techpulse_keyword_history` localStorage key (via storage.js). Recording keyword frequencies from Story[]. Spike detection algorithm. |
| **Forbidden** | Implementing its own tokenization (must use keyword_engine). Accessing any other localStorage key. Importing view or component modules. Modifying Story objects. |
| **Rationale** | The trending algorithm will be tuned repeatedly (window sizes, thresholds, spike formulas). The interface (`recordStories()`, `getTrending()`) must remain stable so view controllers don't break during tuning. |

### `services/knowledge_store.js` — **Stable interface, volatile internals**

| | |
|---|---|
| **Owns** | The `techpulse_knowledge_map` localStorage key (via storage.js). CRUD for topic familiarity states. Blind spot computation (topics with status `never_seen` or `heard_of`, sorted by frequency). |
| **Forbidden** | Implementing its own tokenization (must use keyword_engine). Accessing any other localStorage key. Importing view or component modules. |
| **Rationale** | Knowledge states and blind spot logic may evolve. The interface (`getStatus()`, `setStatus()`, `getBlindSpots()`) must remain stable. |

### `services/milestone_store.js` — **Stable interface, volatile internals**

| | |
|---|---|
| **Owns** | The `techpulse_milestones` localStorage key (via storage.js). CRUD for milestone entries. Auto-detection threshold logic. |
| **Forbidden** | Accessing any other localStorage key. Importing view or component modules. |
| **Rationale** | Same principle. Threshold for auto-detection will change. The interface stays fixed. |

### `services/settings_store.js` — **Stable**

| | |
|---|---|
| **Owns** | The `techpulse_settings` localStorage key (via storage.js). User preferences: enabled sources, trending thresholds, UI preferences. |
| **Forbidden** | Implementing business logic. Accessing any other localStorage key. |
| **Rationale** | Settings are read by other services (aggregator checks which sources are enabled, trend_engine reads threshold config). Settings store only persists and retrieves — it does not interpret. |

### `views/feed.js`, `blindspots.js`, `timeline.js` — **Volatile**

| | |
|---|---|
| **Owns** | Coordinating services and components for its specific view. Fetching data from aggregator/services. Passing processed data to components. Handling user events from components and translating them into service calls. |
| **Forbidden** | Accessing localStorage or storage.js directly. Implementing trend/keyword/milestone algorithms. Importing other view controllers. Constructing raw DOM beyond view-level containers. |
| **Rationale** | View controllers are the glue layer. They change when UI requirements change but should never change when algorithms or storage schemas change. |

### `components/*` — **Volatile**

| | |
|---|---|
| **Owns** | Rendering DOM from provided data. Defining the visual structure and CSS of its element. Exposing callback hooks for user interactions. |
| **Forbidden** | Importing from `services/`, `adapters/`, `infrastructure/storage.js`, or `views/`. Reading or writing localStorage. Performing fetch calls. Computing trends, scores, or keyword logic. Holding state beyond what's needed for rendering (e.g., hover state is fine; keyword frequency is not). |
| **Rationale** | Components must be pure rendering functions. If a component imports a service, changing that service forces re-testing the component, and vice versa. |

### `main.js` — **Stable**

| | |
|---|---|
| **Owns** | Application initialization. View routing (which view is active). Global event listeners (navigation tabs). |
| **Forbidden** | Business logic. Direct DOM manipulation of view contents. Importing adapters or infrastructure modules. Accessing localStorage. Growing beyond ~50 lines of routing logic. |
| **Rationale** | main.js is the entry point. If it accumulates logic, it becomes untestable and every change risks the entire app. It delegates everything to view controllers. |

---

## 2. Data Flow Diagram

```
 ┌──────────────────────────────────────────────────────────────────┐
 │                      EXTERNAL BOUNDARY                          │
 │  HN Firebase API  ·  GitHub  ·  Reddit  ·  ProductHunt  ·  ArXiv│
 └──────────┬───────────────────────────────────────────────────────┘
            │ raw JSON (source-specific, volatile formats)
            ▼
 ┌──────────────────────────────────────────────────────────────────┐
 │                         ADAPTERS                                │
 │  hn.js  ·  github.js  ·  reddit.js  ·  producthunt.js  · arxiv.js │
 │                                                                  │
 │  Each adapter:                                                   │
 │    1. Fetches from its API                                       │
 │    2. Maps raw fields → createStory() using keyword_engine       │
 │    3. Returns Story[] (or [] on failure)                         │
 │                                                                  │
 │  ⛔ Raw API data MUST NOT pass this boundary                     │
 └──────────┬───────────────────────────────────────────────────────┘
            │ Story[] (per source)
            ▼
 ┌──────────────────────────────────────────────────────────────────┐
 │                        AGGREGATOR                               │
 │  aggregator.js                                                   │
 │                                                                  │
 │    1. Reads settings_store for enabled sources                   │
 │    2. Calls enabled adapters in parallel                         │
 │    3. Merges all Story[] into one array                          │
 │    4. Deduplicates by URL                                        │
 │    5. Sorts by score (descending) then timestamp (descending)    │
 │    6. Returns unified Story[]                                    │
 └──────────┬───────────────────────────────────────────────────────┘
            │ Story[] (unified, deduplicated, sorted)
            ▼
 ┌──────────────────────────────────────────────────────────────────┐
 │                     VIEW CONTROLLERS                            │
 │  feed.js  ·  blindspots.js  ·  timeline.js                      │
 │                                                                  │
 │  Each view controller:                                           │
 │    1. Receives Story[] from aggregator                           │
 │    2. Calls services for analysis:                               │
 │       - trend_engine.recordStories(stories)                      │
 │       - trend_engine.getTrending() → TrendItem[]                 │
 │       - knowledge_store.getBlindSpots() → BlindSpot[]            │
 │       - milestone_store.getMilestones() → Milestone[]            │
 │    3. Passes PROCESSED data to components                        │
 │    4. Receives user intent events from components                │
 │    5. Translates events into service calls                       │
 │                                                                  │
 │  ⛔ Components never call services directly                      │
 │  ⛔ View controllers never call each other                       │
 └────────┬─────────────────────────┬───────────────────────────────┘
          │ processed data          ▲ user events (callbacks)
          ▼                         │
 ┌──────────────────────────────────────────────────────────────────┐
 │                       COMPONENTS                                │
 │  NewsCard · TrendingBar · BlindSpots · Timeline · MilestoneForm │
 │                                                                  │
 │    - Receive data as function arguments                          │
 │    - Return DOM elements                                         │
 │    - Call provided callbacks on user interaction                  │
 │    - No imports from services/adapters/storage                   │
 └──────────────────────────────────────────────────────────────────┘


 PERSISTENCE (side channel — never accessed by components or adapters):

 ┌──────────────────────────────────────────────────────────────────┐
 │                     SERVICES → STORAGE                          │
 │                                                                  │
 │  trend_engine ──→ storage.js ──→ localStorage:keyword_history    │
 │  knowledge_store → storage.js ──→ localStorage:knowledge_map     │
 │  milestone_store → storage.js ──→ localStorage:milestones        │
 │  settings_store ─→ storage.js ──→ localStorage:settings          │
 │                                                                  │
 │  storage.js handles versioning + migration for ALL stores        │
 │  Each store declares its own version and migration functions     │
 └──────────────────────────────────────────────────────────────────┘


 SHARED INFRASTRUCTURE (imported by adapters and services, never by components):

 ┌──────────────────────────────────────────────────────────────────┐
 │  keyword_engine.js                                               │
 │    extractKeywords(text) → string[]                              │
 │    normalizeKeyword(keyword) → string                            │
 │                                                                  │
 │  Used by: adapters (to populate Story.tags)                      │
 │           trend_engine (to track frequencies)                    │
 │           knowledge_store (to identify topics)                   │
 │                                                                  │
 │  ⛔ keyword_engine does not import from any project module       │
 │     except models/story.js constants if needed                   │
 └──────────────────────────────────────────────────────────────────┘
```

---

## 3. Non-Negotiable Invariants

These must hold at all times. Violation of any invariant is a defect regardless of whether the code "works."

### INV-1: Story is the only cross-boundary data shape for news content

Every module that produces, consumes, or transforms news content uses `Story` objects created through `createStory()`. No raw API response object, partial story, or ad-hoc `{ title, url }` object may be passed between modules.

**Test**: grep for raw API field names (e.g., `item.descendants`, `data.ups`) outside adapter files. Zero matches expected.

### INV-2: Each localStorage key has exactly one owning module

| Key | Owner | All other modules |
|---|---|---|
| `techpulse_keyword_history` | `trend_engine.js` | Must not read or write |
| `techpulse_knowledge_map` | `knowledge_store.js` | Must not read or write |
| `techpulse_milestones` | `milestone_store.js` | Must not read or write |
| `techpulse_settings` | `settings_store.js` | Must not read or write |
| `techpulse_meta` | `storage.js` | Must not read or write |

**Test**: grep for `localStorage` outside `infrastructure/storage.js`. Zero matches expected.

### INV-3: Components have no upward dependencies

Files in `components/` may import:
- Other component files (CSS)
- `models/story.js` (for type reference only)

Files in `components/` must never import from:
- `services/*`
- `adapters/*`
- `infrastructure/storage.js`
- `views/*`

**Test**: grep for `from '../services/` or `from '../adapters/` or `from '../infrastructure/storage` in `components/` directory. Zero matches expected.

### INV-4: All persisted data is versioned

Every object written to localStorage via `storage.js` contains a `_version` integer field. Every store created via `createStore()` declares its current version and a migrations map. Data without `_version` is treated as version 0.

**Test**: read any localStorage key → parse → assert `_version` exists and is a positive integer.

### INV-5: All keyword processing goes through keyword_engine

No module implements its own:
- Tokenization (splitting text into words)
- Stopword filtering
- Alias resolution
- Keyword normalization

All such operations call `extractKeywords()` or `normalizeKeyword()` from `keyword_engine.js`.

**Test**: grep for `.split(/\s+/)` or `.toLowerCase()` used for keyword purposes outside `keyword_engine.js`. Zero matches expected in service/adapter files.

### INV-6: Adapters are the only source-specific code

No module outside `adapters/` contains:
- API endpoint URLs for external services
- Raw field name mappings (e.g., `item.descendants` → `comments`)
- Source-specific error handling or retry logic

**Test**: grep for external API base URLs outside `adapters/`. Zero matches expected.

### INV-7: View controllers are independent

No view controller imports another view controller. They share data only through services (which they both call independently).

**Test**: grep for `from './feed` in `blindspots.js` or `timeline.js`, etc. Zero matches expected.

### INV-8: main.js contains no business logic

main.js may call view controller `init()` / `destroy()` methods and listen for navigation events. It must not:
- Call adapter or service methods directly
- Manipulate DOM content within views
- Contain conditional logic about stories, trends, or knowledge

**Test**: main.js should have no imports from `services/` or `adapters/`.

---

## 4. High-Risk Evolution Points

These are areas **expected to change** over the project lifetime. Each is isolated behind a defined interface so that changes remain local.

| Risk Area | What will change | Isolation boundary | Stable interface |
|---|---|---|---|
| **API formats** | External APIs change endpoints, field names, rate limits, auth requirements | Individual adapter file | `fetchStories({ limit }) → Story[]` |
| **Trending algorithm** | Spike threshold (2x → 3x?), window size (7d → 14d?), weighting formula | `trend_engine.js` internals | `recordStories(Story[])`, `getTrending() → TrendItem[]` |
| **Keyword rules** | New stopwords, new aliases (e.g., "DeepSeek-R2" → "deepseek"), plural handling | `keyword_engine.js` internals | `extractKeywords(text) → string[]`, `normalizeKeyword(kw) → string` |
| **Score normalization** | How to equate HN score 500 vs GitHub 10k stars vs Reddit 2k upvotes | Each adapter's mapping logic | `Story.score` is always 0-100 |
| **Storage backend** | localStorage → IndexedDB (if data grows beyond 5MB) | `storage.js` | `createStore()` returns `{ read(), write(), clear() }` |
| **Knowledge states** | May add new states beyond know/heard/never/want | `knowledge_store.js` internals + migration | `getStatus() → string`, `setStatus(topic, status)` |
| **UI layout** | Card designs, view layouts, color schemes, responsiveness | Component files only | Components accept data + callbacks, return DOM |
| **Data sources** | New sources added, old sources removed | New adapter file + aggregator registry | `aggregator.fetchAll() → Story[]` |

### Change impact verification

For each high-risk change, the expected file-change footprint:

| Change | Files modified |
|---|---|
| HN API changes endpoint format | `adapters/hn.js` only |
| Add new data source (e.g., Lobste.rs) | New `adapters/lobsters.js` + register in `aggregator.js` |
| Change trending spike threshold from 2x to 3x | `trend_engine.js` only |
| Add alias "gpt-5" → "gpt" | `keyword_engine.js` only |
| Redesign NewsCard layout | `components/NewsCard.js` + `NewsCard.css` only |
| Migrate from localStorage to IndexedDB | `infrastructure/storage.js` only |
| Add new knowledge state "mastered" | `knowledge_store.js` (logic + migration) + `components/BlindSpots.js` (display) |

If a change requires modifying files not listed in its row, the architecture has a boundary violation that must be fixed before proceeding.

---

## 5. Data Schema Versioning & Migration Strategy

### Principle

Every persisted dataset will evolve. Silent schema drift in a knowledge system causes data loss or incorrect analysis that may not be noticed for weeks. Therefore:

1. All data is versioned from the first write.
2. All stores declare migrations eagerly (at definition time), run lazily (on first read).
3. No migration is ever deleted — old migrations remain in the chain to support data from any historical version.

### Implementation: `infrastructure/storage.js`

```js
const STORAGE_PREFIX = 'techpulse_';

/**
 * Creates a versioned persistent store backed by localStorage.
 *
 * @param {string} key - Storage key (without prefix)
 * @param {Object} options
 * @param {number} options.version - Current schema version (positive integer)
 * @param {Object} options.migrations - Map of version number → migration function
 *   Each migration receives data at version N-1 and returns data at version N.
 * @param {Function} options.defaultValue - Factory returning the default empty state
 * @returns {{ read(): any, write(data: any): void, clear(): void }}
 */
export function createStore(key, { version, migrations = {}, defaultValue }) {
  const fullKey = STORAGE_PREFIX + key;

  function read() {
    const raw = localStorage.getItem(fullKey);
    if (raw === null) return defaultValue();

    let data;
    try {
      data = JSON.parse(raw);
    } catch {
      console.warn(`[storage] Corrupted data for "${key}", resetting to default.`);
      return defaultValue();
    }

    return applyMigrations(data, version, migrations);
  }

  function write(data) {
    const versioned = { ...data, _version: version };
    localStorage.setItem(fullKey, JSON.stringify(versioned));
  }

  function clear() {
    localStorage.removeItem(fullKey);
  }

  return Object.freeze({ read, write, clear });
}

function applyMigrations(data, targetVersion, migrations) {
  let current = data._version || 0;

  while (current < targetVersion) {
    current++;
    const migrator = migrations[current];
    if (migrator) {
      data = migrator(data);
    }
    data._version = current;
  }

  return data;
}
```

### Usage pattern in stores

```js
// Example: services/trend_engine.js
import { createStore } from '../infrastructure/storage.js';

const store = createStore('keyword_history', {
  version: 1,
  migrations: {
    // Version 1 is the initial schema — no migration needed from 0→1
    // because defaultValue handles the "no data" case.
    //
    // Future example:
    // 2: (data) => {
    //   // v1 stored counts as integers; v2 stores as { count, sources }
    //   for (const [day, keywords] of Object.entries(data.days)) {
    //     for (const [kw, count] of Object.entries(keywords)) {
    //       data.days[day][kw] = { count, sources: [] };
    //     }
    //   }
    //   return data;
    // }
  },
  defaultValue: () => ({ _version: 1, days: {} }),
});
```

### Migration rules

1. Migrations are **pure functions**: `(oldData) → newData`. No I/O, no side effects.
2. Migrations are **never deleted**. A store at version 5 must be able to migrate data from version 1 by chaining 1→2→3→4→5.
3. Migrations are **tested** with snapshot data from each version (keep test fixtures).
4. If a migration cannot preserve data (breaking change), it must log a warning and fall back to `defaultValue()` rather than corrupt silently.

---

## 6. Module Interfaces & File Structure

### Final directory layout

```
src/
├── main.js                          [Stable]  App shell, view routing
├── style.css                        [Volatile] Global styles
│
├── models/
│   └── story.js                     [Frozen]  Story shape, factory, validator
│
├── infrastructure/
│   ├── storage.js                   [Frozen]  Versioned localStorage wrapper
│   └── keyword_engine.js            [Stable]  Tokenization, stopwords, aliases
│
├── adapters/
│   ├── hn.js                        [Volatile] Hacker News → Story[]
│   ├── github.js                    [Volatile] GitHub Trending → Story[]
│   ├── reddit.js                    [Volatile] Reddit → Story[]
│   ├── producthunt.js               [Volatile] Product Hunt → Story[]
│   └── arxiv.js                     [Volatile] ArXiv CS → Story[]
│
├── services/
│   ├── aggregator.js                [Stable]  Adapter orchestration, merge, dedup
│   ├── trend_engine.js              [Stable]  Keyword frequency, spike detection
│   ├── knowledge_store.js           [Stable]  Personal knowledge map
│   ├── milestone_store.js           [Stable]  Timeline milestones
│   └── settings_store.js            [Stable]  User preferences
│
├── views/
│   ├── feed.js                      [Volatile] Feed view controller
│   ├── blindspots.js                [Volatile] Blind Spots view controller
│   └── timeline.js                  [Volatile] Timeline view controller
│
└── components/
    ├── NewsCard.js + .css           [Volatile] Story card rendering
    ├── TrendingBar.js + .css        [Volatile] Trending chips rendering
    ├── BlindSpots.js + .css         [Volatile] Blind spots panel rendering
    ├── TopicBadge.js + .css         [Volatile] Knowledge status dot
    ├── Timeline.js + .css           [Volatile] Vertical timeline rendering
    └── MilestoneForm.js + .css      [Volatile] Manual milestone entry form
```

### Module interfaces (public API of each module)

```
models/story.js
  ├── createStory({ id, title, url, source, score, comments, author, timestamp, tags }) → Story
  ├── isStory(obj) → boolean
  └── SOURCES: { HACKERNEWS, GITHUB, REDDIT, PRODUCTHUNT, ARXIV }

infrastructure/storage.js
  └── createStore(key, { version, migrations, defaultValue }) → { read(), write(data), clear() }

infrastructure/keyword_engine.js
  ├── extractKeywords(text) → string[]
  └── normalizeKeyword(keyword) → string

adapters/*.js  (all adapters share this interface)
  └── fetchStories({ limit }) → Promise<Story[]>

services/aggregator.js
  ├── fetchAll({ limit }) → Promise<Story[]>
  └── registerAdapter(name, adapterModule) → void

services/trend_engine.js
  ├── recordStories(stories: Story[]) → void
  ├── getTrending() → TrendItem[]        // { keyword, count, changePercent }
  └── getHistory(keyword) → DayCount[]   // { date, count }

services/knowledge_store.js
  ├── getStatus(topic) → string          // 'know_it'|'heard_of'|'never_seen'|'want_to_learn'
  ├── setStatus(topic, status) → void
  ├── getBlindSpots() → BlindSpot[]      // { topic, status, storyCount, firstSeen, lastSeen }
  └── recordTopics(keywords: string[]) → void  // updates firstSeen/lastSeen

services/milestone_store.js
  ├── addMilestone(story: Story, pinned: boolean) → void
  ├── removeMilestone(id: string) → void
  ├── getMilestones({ category?, source? }) → Milestone[]
  └── isMilestone(storyId: string) → boolean

services/settings_store.js
  ├── get(key) → any
  ├── set(key, value) → void
  └── getAll() → Settings

views/*.js  (all view controllers share this interface)
  ├── init(container: HTMLElement, { services }) → void
  └── destroy() → void

components/*.js  (all components share this pattern)
  └── createXxx(data, { onEvent, onOtherEvent }) → HTMLElement
```

### Dependency graph (who may import whom)

```
main.js ──→ views/*
views/* ──→ services/*, components/*, models/story.js
services/* ──→ infrastructure/*, models/story.js
adapters/* ──→ infrastructure/keyword_engine.js, models/story.js
components/* ──→ models/story.js (type reference only), own .css files
infrastructure/* ──→ (nothing — leaf modules)
models/* ──→ (nothing — leaf modules)
```

**Forbidden edges (these imports must never exist):**
```
components/* ──✖──→ services/*
components/* ──✖──→ adapters/*
components/* ──✖──→ infrastructure/storage.js
components/* ──✖──→ views/*
adapters/*   ──✖──→ services/*
adapters/*   ──✖──→ views/*
adapters/*   ──✖──→ components/*
services/*   ──✖──→ views/*
services/*   ──✖──→ components/*
services/*   ──✖──→ adapters/*
main.js      ──✖──→ services/*
main.js      ──✖──→ adapters/*
main.js      ──✖──→ infrastructure/*
views/*      ──✖──→ views/* (no cross-view imports)
```

---

## Summary: Verification Checklist

Before merging any change, verify:

| # | Check | How to verify |
|---|---|---|
| 1 | No raw API fields outside adapters | `grep` for source-specific field names in `services/`, `views/`, `components/` |
| 2 | No `localStorage` calls outside `storage.js` | `grep -r "localStorage" src/ --include="*.js"` — only hits in `infrastructure/storage.js` |
| 3 | No service imports in components | `grep -r "from.*services" src/components/` — zero matches |
| 4 | All persisted data has `_version` | Read each localStorage key, parse, check `_version` field |
| 5 | No tokenization outside keyword_engine | `grep -r "\.split\b" src/services/ src/adapters/` used for keyword purposes — zero matches |
| 6 | No cross-view imports | `grep -r "from.*views/" src/views/` — each file only imports from services/components |
| 7 | main.js has no service imports | Check import statements in main.js |
| 8 | All adapters return Story[] | Each adapter's return passes `results.every(isStory)` |
