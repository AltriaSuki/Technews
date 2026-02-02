use async_trait::async_trait;
use crate::article::Article;
use crate::error::DomainError;

#[async_trait]
pub trait ArticleGateway: Send + Sync {
    async fn fetch_top_articles(&self, limit: usize) -> Result<Vec<Article>, DomainError>;
}
