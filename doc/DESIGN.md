# Tech Pulse v2 — Design Overview

This document captures the product-level design intent for the Rust-based v2. It complements:
- Architecture: `doc/ARCHITECTURE_V2.md`
- Legacy v1 design: `doc/ARCHIVE/legacy-v1/design.md`

---

## 1. Product Intent

Tech Pulse helps users discover what they are *missing* in tech news, not just what they already follow. The system highlights blind spots and reinforces a broader knowledge map over time.

---

## 2. Core User Value

1. **Blind-spot discovery**: Reveal high-activity domains the user rarely reads.
2. **Concise awareness**: A short daily briefing that can be consumed in minutes.
3. **Trustable relevance**: Ranking that balances recency, trend momentum, and diversity.

---

## 3. Primary Views (v2 MVP)

1. **Today (Briefing)**
   - Top stories (limited count)
   - Top trends (limited count)
   - 1-2 blind-spot nudges

2. **Explore (Smart Feed)**
   - Ranked feed with trend and blind-spot weight
   - Filters: source, topic, recency
   - “New since last visit” divider

Notes:
- The **Blind Spots** and **Timeline** views are deferred until interaction data is available.

---

## 4. Personalization Signals (MVP)

- Click / open
- Read duration (simple threshold)
- Explicit “already know / want to learn” actions

---

## 5. Ranking Principles (MVP)

- Normalize across sources (percentile-style normalization)
- Balance factors (weights are tunable):
  - recency
  - trend momentum
  - blind-spot boost
  - source diversity

---

## 6. SEO and Distribution

- Public content must be SSR-rendered and indexable.
- PWA is the primary mobile “app” experience for v2.
- App Store distribution is optional and can follow after web maturity.

---

## 7. Risks & Mitigations

- **Cold start**: Provide default “explore” experience and light onboarding.
- **Trust**: Add “why this story” explanations for ranked items.
- **Data quality**: Keep a curated source list and observable ingestion failures.

---

## 8. Success Metrics (MVP)

- Day-1 activation: first session length and depth
- Return visits within 7 days
- Blind-spot card engagement rate

---

## 9. Scope Control

In v2 MVP:
- Do not attempt full timeline or deep knowledge map visualizations.
- Keep model complexity low; focus on experience and data integrity.

