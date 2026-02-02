use std::collections::HashMap;
use std::sync::Arc;
use techpulse_domain::error::DomainError;
use techpulse_domain::repository::{ArticleRepo, TrendRepo};
use techpulse_domain::trend::{Trend, TrendReport};

pub struct CalculateTrends {
    article_repo: Arc<dyn ArticleRepo>,
    trend_repo: Arc<dyn TrendRepo>,
}

impl CalculateTrends {
    pub fn new(
        article_repo: Arc<dyn ArticleRepo>,
        trend_repo: Arc<dyn TrendRepo>,
    ) -> Self {
        Self {
            article_repo,
            trend_repo,
        }
    }

    pub async fn execute(&self, keywords: &[String], now: i64) -> Result<TrendReport, DomainError> {
        // 1. Fetch recent articles
        let articles = self.article_repo.find_latest(100).await?;
        
        // 2. Simple analysis: Group by keywords in title
        // Map: keyword -> (count, list_of_article_ids)
        let mut topic_counts: HashMap<String, (u32, Vec<techpulse_domain::article::ArticleId>)> = HashMap::new();
        
        for article in &articles {
            let title_lower = article.title.to_lowercase();
            for kw in keywords {
                let kw_lower = kw.to_lowercase();
                // Naive word boundary check could be improved with regex, 
                // but for now simple containment of lowercased strings is the MVP step up.
                if title_lower.contains(&kw_lower) {
                    let entry = topic_counts.entry(kw.clone()).or_insert((0, Vec::new()));
                    entry.0 += 1;
                    entry.1.push(article.id.clone());
                }
            }
        }
        
        // 3. Build Trend objects
        let mut trends = Vec::new();
        for (kw, (count, ids)) in topic_counts {
            trends.push(Trend {
                keyword: kw,
                // Simple score metric: volume * 10
                score: count as f64 * 10.0, 
                volume: count,
                velocity: 0.0,
                related_articles: ids,
            });
        }
        
        // 4. Create and Save Report
        let report = TrendReport {
            timestamp: now,
            trends,
            metadata: HashMap::new(),
        };
        
        self.trend_repo.save_report(&report).await?;
        
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::mock;
    use techpulse_domain::article::{Article, ArticleId};
    use techpulse_domain::article::Source;

    mock! {
        pub ArticleRepo {}
        #[async_trait]
        impl ArticleRepo for ArticleRepo {
            async fn save(&self, article: &Article) -> Result<(), DomainError>;
            async fn find_by_id(&self, id: &ArticleId) -> Result<Option<Article>, DomainError>;
            async fn find_latest(&self, limit: usize) -> Result<Vec<Article>, DomainError>;
        }
    }

    mock! {
        pub TrendRepo {}
        #[async_trait]
        impl TrendRepo for TrendRepo {
            async fn save_report(&self, report: &TrendReport) -> Result<(), DomainError>;
            async fn find_latest_report(&self) -> Result<Option<TrendReport>, DomainError>;
        }
    }

    #[tokio::test]
    async fn test_calculate_trends() {
        let mut mock_article_repo = MockArticleRepo::new();
        let mut mock_trend_repo = MockTrendRepo::new();
        
        // Case insensitive matching test: "rust" in implementation should match "Rust" in title
        let a1 = Article::new(Source::HackerNews, "1", "Rust is great".into(), "".into(), 100).unwrap();
        // "AI" should match "AI"
        let a2 = Article::new(Source::HackerNews, "2", "AI is future".into(), "".into(), 100).unwrap();
        // "rust" lowercase in title should match "Rust" keyword
        let a3 = Article::new(Source::HackerNews, "3", "learning rust 1.0".into(), "".into(), 100).unwrap();
        
        let articles = vec![a1, a2, a3];
        
        mock_article_repo.expect_find_latest()
            .returning(move |_| Ok(articles.clone()));
            
        mock_trend_repo.expect_save_report()
            .times(1)
            .withf(|r| r.timestamp == 1234567890) // Verify timestamp passed through
            .returning(|_| Ok(()));
            
        let usecase = CalculateTrends::new(
            Arc::new(mock_article_repo),
            Arc::new(mock_trend_repo)
        );
        
        let keywords = vec!["Rust".to_string(), "AI".to_string()];
        let report = usecase.execute(&keywords, 1234567890).await.unwrap();
        
        assert_eq!(report.trends.len(), 2);
        
        let rust_trend = report.trends.iter().find(|t| t.keyword == "Rust").unwrap();
        assert_eq!(rust_trend.volume, 2);
        assert_eq!(rust_trend.related_articles.len(), 2);
    }

    #[tokio::test]
    async fn test_empty_articles() {
        let mut mock_article_repo = MockArticleRepo::new();
        let mut mock_trend_repo = MockTrendRepo::new();
        
        mock_article_repo.expect_find_latest()
            .returning(|_| Ok(vec![]));
            
        mock_trend_repo.expect_save_report()
            .times(1)
            .returning(|_| Ok(()));
            
        let usecase = CalculateTrends::new(
            Arc::new(mock_article_repo),
            Arc::new(mock_trend_repo)
        );
        
        let keywords = vec!["Rust".to_string()];
        let report = usecase.execute(&keywords, 100).await.unwrap();
        
        assert!(report.trends.is_empty());
    }
    
    #[tokio::test]
    async fn test_repo_error_propagation() {
        let mut mock_article_repo = MockArticleRepo::new();
        let mock_trend_repo = MockTrendRepo::new(); // No expectation needed if first call fails
        
        mock_article_repo.expect_find_latest()
            .returning(|_| Err(DomainError::Repository("DB Down".into())));
            
        let usecase = CalculateTrends::new(
            Arc::new(mock_article_repo),
            Arc::new(mock_trend_repo)
        );
        
        let keywords = vec!["Rust".to_string()];
        let result = usecase.execute(&keywords, 100).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::Repository(msg) => assert_eq!(msg, "DB Down"),
            _ => panic!("Wrong error type"),
        }
    }
}
