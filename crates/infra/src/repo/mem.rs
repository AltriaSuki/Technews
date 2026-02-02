use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use techpulse_domain::article::{Article, ArticleId};
use techpulse_domain::error::DomainError;
use techpulse_domain::repository::{ArticleRepo, TimelineRepo, TrendRepo, UserRepo};
use techpulse_domain::trend::{TimelineEvent, TimelineEventId, TrendReport};
use techpulse_domain::user::{UserId, UserProfile};

// Helper macro or function for lock mapping could go here, 
// but direct mapping is also fine if cleaned up. 
// Let's use a private trait extension or method if desirable, 
// but idiomatic simple map_err is often preferred. 
// We will stick to simple map_err but ensure lines are cleaner where possible.

// --- Article Repository ---
#[derive(Debug, Clone, Default)]
pub struct InMemoryArticleRepo {
    store: Arc<RwLock<HashMap<ArticleId, Article>>>,
}

impl InMemoryArticleRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ArticleRepo for InMemoryArticleRepo {
    async fn save(&self, article: &Article) -> Result<(), DomainError> {
        let mut store = self.store.write().map_err(|e| DomainError::Repository(e.to_string()))?;
        store.insert(article.id.clone(), article.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &ArticleId) -> Result<Option<Article>, DomainError> {
        let store = self.store.read().map_err(|e| DomainError::Repository(e.to_string()))?;
        Ok(store.get(id).cloned())
    }

    async fn find_latest(&self, limit: usize) -> Result<Vec<Article>, DomainError> {
        let store = self.store.read().map_err(|e| DomainError::Repository(e.to_string()))?;
        let mut articles: Vec<Article> = store.values().cloned().collect();
        articles.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        articles.truncate(limit);
        Ok(articles)
    }
}

// --- User Repository ---
#[derive(Debug, Clone, Default)]
pub struct InMemoryUserRepo {
    store: Arc<RwLock<HashMap<UserId, UserProfile>>>,
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl UserRepo for InMemoryUserRepo {
    async fn save(&self, user: &UserProfile) -> Result<(), DomainError> {
        let mut store = self.store.write().map_err(|e| DomainError::Repository(e.to_string()))?;
        store.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<UserProfile>, DomainError> {
        let store = self.store.read().map_err(|e| DomainError::Repository(e.to_string()))?;
        Ok(store.get(id).cloned())
    }
}

// --- Trend Repository ---
#[derive(Debug, Clone, Default)]
pub struct InMemoryTrendRepo {
    reports: Arc<RwLock<Vec<TrendReport>>>,
}

impl InMemoryTrendRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl TrendRepo for InMemoryTrendRepo {
    async fn save_report(&self, report: &TrendReport) -> Result<(), DomainError> {
        let mut reports = self.reports.write().map_err(|e| DomainError::Repository(e.to_string()))?;
        reports.push(report.clone());
        Ok(())
    }

    async fn find_latest_report(&self) -> Result<Option<TrendReport>, DomainError> {
        let reports = self.reports.read().map_err(|e| DomainError::Repository(e.to_string()))?;
        // Explicit sort by timestamp to ensure correctness regardless of insertion order
        Ok(reports.iter().max_by_key(|r| r.timestamp).cloned())
    }
}

// --- Timeline Repository ---
#[derive(Debug, Clone, Default)]
pub struct InMemoryTimelineRepo {
    // Key by ID to support upserts
    events: Arc<RwLock<HashMap<TimelineEventId, TimelineEvent>>>,
}

impl InMemoryTimelineRepo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl TimelineRepo for InMemoryTimelineRepo {
    async fn save_event(&self, event: &TimelineEvent) -> Result<(), DomainError> {
        let mut events = self.events.write().map_err(|e| DomainError::Repository(e.to_string()))?;
        events.insert(event.id.clone(), event.clone());
        Ok(())
    }

    async fn list_events(&self) -> Result<Vec<TimelineEvent>, DomainError> {
        let events = self.events.read().map_err(|e| DomainError::Repository(e.to_string()))?;
        let mut list: Vec<TimelineEvent> = events.values().cloned().collect();
        // Sort by date descending (newest first)
        list.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use techpulse_domain::article::Source;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_article_repo() {
        let repo = InMemoryArticleRepo::new();
        let article = Article::new(
            Source::HackerNews,
            "123",
            "Test Title".into(),
            "http://test.com".into(),
            1000,
        ).unwrap();

        repo.save(&article).await.unwrap();

        let found = repo.find_by_id(&article.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Test Title");

        let missing = repo.find_by_id(&ArticleId::new(&Source::GitHub, "999").unwrap()).await.unwrap();
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn test_find_latest_sorting_and_limit() {
        let repo = InMemoryArticleRepo::new();
        let a1 = Article::new(Source::HackerNews, "1", "Old".into(), "".into(), 100).unwrap();
        let a2 = Article::new(Source::HackerNews, "2", "New".into(), "".into(), 200).unwrap();
        let a3 = Article::new(Source::HackerNews, "3", "Mid".into(), "".into(), 150).unwrap();
        
        repo.save(&a1).await.unwrap();
        repo.save(&a2).await.unwrap();
        repo.save(&a3).await.unwrap();

        // Test sorting
        let latest = repo.find_latest(3).await.unwrap();
        assert_eq!(latest[0].title, "New");
        assert_eq!(latest[1].title, "Mid");
        assert_eq!(latest[2].title, "Old");
        
        // Test limit
        let limited = repo.find_latest(2).await.unwrap();
        assert_eq!(limited.len(), 2);
        assert_eq!(limited[0].title, "New");
    }
    
    #[tokio::test]
    async fn test_article_upsert() {
        let repo = InMemoryArticleRepo::new();
        let mut article = Article::new(Source::HackerNews, "1", "V1".into(), "".into(), 100).unwrap();
        repo.save(&article).await.unwrap();
        
        article.title = "V2".into();
        repo.save(&article).await.unwrap();
        
        let found = repo.find_by_id(&article.id).await.unwrap().unwrap();
        assert_eq!(found.title, "V2");
    }
    
    #[tokio::test]
    async fn test_user_repo() {
        let repo = InMemoryUserRepo::new();
        let user = UserProfile::new(UserId::new("u1"));
        
        repo.save(&user).await.unwrap();
        let found = repo.find_by_id(&user.id).await.unwrap();
        assert_eq!(found.unwrap().id, user.id);
    }
    
    #[tokio::test]
    async fn test_timeline_repo_ordering_and_upsert() {
        let repo = InMemoryTimelineRepo::new();
        let date1 = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        
        let e1 = TimelineEvent {
            id: TimelineEventId::from("e1"),
            title: "E1".into(),
            date: date1,
            description: "".into(),
            category: "".into(),
            importance_score: 1.0,
        };
        let e2 = TimelineEvent {
            id: TimelineEventId::from("e2"),
            title: "E2".into(),
            date: date2, // Newer
            description: "".into(),
            category: "".into(),
            importance_score: 1.0,
        };
        
        repo.save_event(&e1).await.unwrap();
        repo.save_event(&e2).await.unwrap();
        
        let events = repo.list_events().await.unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].id, e2.id); // Newer first
        
        // Upsert check
        let mut e1_updated = e1.clone();
        e1_updated.title = "E1 Updated".into();
        repo.save_event(&e1_updated).await.unwrap();
        
        let events_after = repo.list_events().await.unwrap();
        assert_eq!(events_after.len(), 2); // Count same
        let found_e1 = events_after.iter().find(|e| e.id == e1.id).unwrap();
        assert_eq!(found_e1.title, "E1 Updated");
    }
    
    #[tokio::test]
    async fn test_trend_repo_find_latest() {
        let repo = InMemoryTrendRepo::new();
        let r1 = TrendReport { timestamp: 100, ..Default::default() };
        let r2 = TrendReport { timestamp: 200, ..Default::default() };
        
        // Save out of order
        repo.save_report(&r2).await.unwrap();
        repo.save_report(&r1).await.unwrap();
        
        let latest = repo.find_latest_report().await.unwrap();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().timestamp, 200);
    }
}
