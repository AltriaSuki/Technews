// Trait definitions for data access (ports)
use crate::article::{Article, ArticleId};
use crate::trend::TrendReport;
use crate::user::UserProfile;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait ArticleRepo: Send + Sync {
    async fn save(&self, article: &Article) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn find_by_id(
        &self,
        id: &ArticleId,
    ) -> Result<Option<Article>, Box<dyn Error + Send + Sync>>;
    async fn find_latest(
        &self,
        limit: usize,
    ) -> Result<Vec<Article>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait TrendRepo: Send + Sync {
    async fn save_report(
        &self,
        report: &TrendReport,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn find_latest_report(
        &self,
    ) -> Result<Option<TrendReport>, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn save(&self, user: &UserProfile) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<UserProfile>, Box<dyn Error + Send + Sync>>;
}
