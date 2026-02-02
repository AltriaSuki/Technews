use std::sync::Arc;
use techpulse_domain::error::DomainError;
use techpulse_domain::gateway::ArticleGateway;
use techpulse_domain::repository::ArticleRepo;

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

#[cfg(test)]
mod tests {
    use super::*;
    use techpulse_domain::article::{Article, ArticleId, Source};
    use mockall::predicate::*;
    use mockall::mock;
    use async_trait::async_trait;

    mock! {
        Gateway {}
        #[async_trait]
        impl ArticleGateway for Gateway {
            async fn fetch_top_articles(&self, limit: usize) -> Result<Vec<Article>, DomainError>;
        }
    }

    mock! {
        Repo {}
        #[async_trait]
        impl ArticleRepo for Repo {
            async fn save(&self, article: &Article) -> Result<(), DomainError>;
            async fn find_by_id(&self, id: &ArticleId) -> Result<Option<Article>, DomainError>;
            async fn find_latest(&self, limit: usize) -> Result<Vec<Article>, DomainError>;
        }
    }

    #[tokio::test]
    async fn test_ingest_articles() {
        let mut mock_gateway = MockGateway::new();
        let mut mock_repo = MockRepo::new();

        let articles = vec![
            Article::new(Source::HackerNews, "1", "Title 1".to_string(), "http://url1".to_string(), 100).unwrap(),
            Article::new(Source::HackerNews, "2", "Title 2".to_string(), "http://url2".to_string(), 200).unwrap(),
        ];

        mock_gateway
            .expect_fetch_top_articles()
            .with(eq(2))
            .times(1)
            .return_once(|_| Ok(articles));

        mock_repo
            .expect_save()
            .times(2)
            .returning(|_| Ok(()));

        let use_case = IngestArticles::new(Arc::new(mock_gateway), Arc::new(mock_repo));
        let result = use_case.execute(2).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }
}
