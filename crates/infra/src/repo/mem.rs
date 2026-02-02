use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use techpulse_domain::article::{Article, ArticleId};
use techpulse_domain::error::DomainError;
use techpulse_domain::repository::{ArticleRepo, TimelineRepo, TrendRepo, UserRepo};
use techpulse_domain::trend::{TimelineEvent, TrendReport};
use techpulse_domain::user::{UserId, UserProfile};

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
        // Sort by timestamp descending
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
    // Storing reports by timestamp (simple valid use-case for now)
    // Or just storing the "latest" separately. 
    // For simplicity, let's keep a list of reports.
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
        // Assuming recently added are at the end, or we sort by timestamp
        // Let's just take the last one pushed for now, assuming append-only in time order
        Ok(reports.last().cloned())
    }
}

// --- Timeline Repository ---
#[derive(Debug, Clone, Default)]
pub struct InMemoryTimelineRepo {
    // For simplicity, just a list. Real impl might index by ID.
    events: Arc<RwLock<Vec<TimelineEvent>>>,
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
        // Check if exists to update? For now just push (append log style) or replace if we had an ID map.
        // Let's assume append for now or simple "upsert" if we scan.
        // Given it's basic, let's just push.
        events.push(event.clone());
        Ok(())
    }

    async fn list_events(&self) -> Result<Vec<TimelineEvent>, DomainError> {
        let events = self.events.read().map_err(|e| DomainError::Repository(e.to_string()))?;
        Ok(events.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use techpulse_domain::article::Source;

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
    async fn test_find_latest_sorting() {
        let repo = InMemoryArticleRepo::new();
        let a1 = Article::new(Source::HackerNews, "1", "Old".into(), "".into(), 100).unwrap();
        let a2 = Article::new(Source::HackerNews, "2", "New".into(), "".into(), 200).unwrap();
        
        repo.save(&a1).await.unwrap();
        repo.save(&a2).await.unwrap();

        let latest = repo.find_latest(10).await.unwrap();
        assert_eq!(latest.len(), 2);
        assert_eq!(latest[0].title, "New");
        assert_eq!(latest[1].title, "Old");
    }
    
    #[tokio::test]
    async fn test_user_repo() {
        let repo = InMemoryUserRepo::new();
        let user = UserProfile::new(UserId::new("u1"));
        
        repo.save(&user).await.unwrap();
        let found = repo.find_by_id(&user.id).await.unwrap();
        assert_eq!(found.unwrap().id, user.id);
    }
}
