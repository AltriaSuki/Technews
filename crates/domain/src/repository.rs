// Trait definitions for data access (ports)
use crate::article::{Article, ArticleId};
use crate::error::DomainError;
use crate::trend::{TimelineEvent, TrendReport};
use crate::user::{UserId, UserProfile};
use async_trait::async_trait;

#[async_trait]
pub trait ArticleRepo: Send + Sync {
    async fn save(&self, article: &Article) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &ArticleId) -> Result<Option<Article>, DomainError>;
    async fn find_latest(&self, limit: usize) -> Result<Vec<Article>, DomainError>;
}

#[async_trait]
pub trait TrendRepo: Send + Sync {
    async fn save_report(&self, report: &TrendReport) -> Result<(), DomainError>;
    async fn find_latest_report(&self) -> Result<Option<TrendReport>, DomainError>;
}

#[async_trait]
pub trait TimelineRepo: Send + Sync {
    async fn save_event(&self, event: &TimelineEvent) -> Result<(), DomainError>;
    async fn list_events(&self) -> Result<Vec<TimelineEvent>, DomainError>;
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn save(&self, user: &UserProfile) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<UserProfile>, DomainError>;
}
