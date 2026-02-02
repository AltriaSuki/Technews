use std::sync::Arc;
use techpulse_domain::error::DomainError;
use techpulse_domain::repository::ArticleRepo;
use techpulse_infra::gateway::ArticleGateway;

pub struct IngestArticles {
    gateway: Arc<dyn ArticleGateway>,
    repo: Arc<dyn ArticleRepo>,
}

impl IngestArticles {
    pub fn new(gateway: Arc<dyn ArticleGateway>, repo: Arc<dyn ArticleRepo>) -> Self {
        Self { gateway, repo }
    }

    pub async fn execute(&self, limit: usize) -> Result<usize, DomainError> {
        let articles = self.gateway.fetch_top_articles(limit).await?;
        let count = articles.len();
        
        for article in articles {
            self.repo.save(&article).await?;
        }
        
        Ok(count)
    }
}
