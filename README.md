# Tech Pulse ⚡️

**Tech Pulse** is a modern, real-time tech news dashboard designed to keep you on the bleeding edge of technology. It aggregates content from multiple high-signal sources, identifies trending topics, tracks your reading habits to reveal knowledge blind spots, and visualizes major tech milestones.

![Tech Pulse Screenshot](https://via.placeholder.com/1200x600?text=Tech+Pulse+Dashboard)

## ✨ Features

*   **🌐 Multi-Source Aggregation**: Real-time news from **Hacker News**, **GitHub** (Trending Repos), **Reddit** (r/programming, r/machinelearning, etc.), **Product Hunt**, and **ArXiv**.
*   **🔥 Trend Engine**: Automatically detects spiking keywords and topics (e.g., "DeepSeek", "React 19") using a 7-day moving average algorithm.
*   **🧠 Personal Knowledge Tracker**:
    *   Tracks your reading history locally.
    *   **Blind Spots**: Visualizes which tech sectors you are neglecting (e.g., "You read a lot of AI, but usually miss Web Dev news").
    *   **Topic Badges**: Visual indicators of story topics (e.g., `#ai`, `#web`).
*   **📝 Quick Summaries**: Instant preview of content (ArXiv abstracts, GitHub descriptions) without leaving the feed.
*   **⏳ Tech Timeline**: A chronological view of major tech events, user-extensible.

## 🚀 Quick Start

1.  **Clone the repository**
    ```bash
    git clone https://github.com/AltriaSuki/Technews.git
    cd Technews
    ```

2.  **Install dependencies**
    ```bash
    npm install
    ```

3.  **Run locally**
    ```bash
    npm run dev
    ```
    Open `http://localhost:5173` in your browser.

## 🛠 Architecture

Tech Pulse is built with **Vanilla JavaScript** and **Vite**, focusing on performance and simplicity. It uses **Glassmorphism** design principles for a premium UI.

*   `src/adapters/`: Source-specific logic (e.g., `github.js`, `arxiv.js`) to fetch and normalize data into a unified `Story` model.
*   `src/services/`: Core systems:
    *   `aggregator.js`: Merges, deduplicates, and caches news.
    *   `trend_engine.js`: Analyzes story keywords for spikes.
    *   `knowledge_store.js`: Manages user profile and history (persisted in `localStorage`).
*   `src/views/`: UI Controllers for different tabs (`feed.js`, `blindspots.js`, `timeline.js`).
*   `src/components/`: Reusable UI elements (`NewsCard.js`, `TrendingBar.js`).

## 🔒 Privacy

All data (reading history, settings) is stored **locally in your browser** (`localStorage`). No tracking data is sent to any server.

## 📄 License

MIT License.
