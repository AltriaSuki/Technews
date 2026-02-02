use std::sync::Arc;
use techpulse_domain::article::Article;
use techpulse_domain::error::DomainError;
use techpulse_domain::repository::ArticleRepo;

pub struct GetChronologicalFeed {
    repo: Arc<dyn ArticleRepo>,
}

impl GetChronologicalFeed {
    pub fn new(repo: Arc<dyn ArticleRepo>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, limit: usize) -> Result<Vec<Article>, DomainError> {
        self.repo.find_latest(limit).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::mock;
    use techpulse_domain::article::{Article, ArticleId, Source};
    
    mock! {
        pub ArticleRepo {}
        #[async_trait]
        impl ArticleRepo for ArticleRepo {
            async fn save(&self, article: &Article) -> Result<(), DomainError>;
            async fn find_by_id(&self, id: &ArticleId) -> Result<Option<Article>, DomainError>;
            async fn find_latest(&self, limit: usize) -> Result<Vec<Article>, DomainError>;
        }
    }

    #[tokio::test]
    async fn test_get_feed() {
        let mut mock_repo = MockArticleRepo::new();
        
        let article = Article::new(Source::HackerNews, "1", "T1".into(), "".into(), 100).unwrap();
        let expected = vec![article];
        let return_val = expected.clone();
        
        mock_repo.expect_find_latest()
            .with(mockall::predicate::eq(10))
            .times(1)
            .returning(move |_| Ok(return_val.clone()));
            
        let usecase = GetChronologicalFeed::new(Arc::new(mock_repo));
        let result = usecase.execute(10).await.unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "T1");
    }
}
