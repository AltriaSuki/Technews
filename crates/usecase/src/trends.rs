use std::collections::HashMap;
use std::sync::Arc;
use techpulse_domain::error::DomainError;
use techpulse_domain::repository::{ArticleRepo, TrendRepo};
use techpulse_domain::trend::{Trend, TrendReport};

pub struct CalculateTrends {
    article_repo: Arc<dyn ArticleRepo + Send + Sync>,
    trend_repo: Arc<dyn TrendRepo + Send + Sync>,
}

impl CalculateTrends {
    pub fn new(
        article_repo: Arc<dyn ArticleRepo + Send + Sync>,
        trend_repo: Arc<dyn TrendRepo + Send + Sync>,
    ) -> Self {
        Self {
            article_repo,
            trend_repo,
        }
    }

    pub async fn execute(&self) -> Result<TrendReport, DomainError> {
        // 1. Fetch recent articles (limit 100 for now)
        let articles = self.article_repo.find_latest(100).await?;
        
        // 2. Simple analysis: Group by keywords in title
        let mut topic_counts: HashMap<String, (u32, Vec<String>)> = HashMap::new();
        
        let keywords = vec!["Rust", "AI", "Cloud", "Crypto", "Apple", "Linux"];
        
        for article in &articles {
            for &kw in &keywords {
                if article.title.contains(kw) {
                    let entry = topic_counts.entry(kw.to_string()).or_insert((0, Vec::new()));
                    entry.0 += 1;
                    // Store article ID (converted to string for now since ArticleId doesn't have clone helper in trait easily available without full import)
                    // Actually we have ArticleId type.
                    // entry.1.push(article.id.to_string());
                }
            }
        }
        
        // 3. Build Trend objects
        let mut trends = Vec::new();
        for (kw, (count, _ids)) in topic_counts {
            if count > 0 {
                trends.push(Trend {
                    keyword: kw,
                    score: count as f64 * 10.0, // dummy score
                    volume: count,
                    velocity: 0.0,
                    related_articles: vec![], // Populating this requires ArticleId clones, skipping for "Basic" MVP simplicity unless trivial
                });
            }
        }
        
        // 4. Create and Save Report
        let report = TrendReport {
            timestamp: ::std::time::SystemTime::now()
                .duration_since(::std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
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
        
        let a1 = Article::new(Source::HackerNews, "1", "Rust is great".into(), "".into(), 100).unwrap();
        let a2 = Article::new(Source::HackerNews, "2", "AI is future".into(), "".into(), 100).unwrap();
        let a3 = Article::new(Source::HackerNews, "3", "Rust 1.0".into(), "".into(), 100).unwrap();
        
        let articles = vec![a1, a2, a3];
        
        mock_article_repo.expect_find_latest()
            .returning(move |_| Ok(articles.clone()));
            
        mock_trend_repo.expect_save_report()
            .times(1)
            .returning(|_| Ok(()));
            
        let usecase = CalculateTrends::new(
            Arc::new(mock_article_repo),
            Arc::new(mock_trend_repo)
        );
        
        let report = usecase.execute().await.unwrap();
        
        assert_eq!(report.trends.len(), 2); // Rust and AI
        
        let rust_trend = report.trends.iter().find(|t| t.keyword == "Rust").unwrap();
        assert_eq!(rust_trend.volume, 2);
    }
}
