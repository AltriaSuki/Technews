// Domain entities for Articles
use serde::{Deserialize, Serialize}; // Keep for potential future use or serialization needs
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArticleId(String);

impl ArticleId {
    pub fn new(source: &Source, native_id: &str) -> Self {
        Self(format!("{}-{}", source.as_str(), native_id))
    }
}

impl fmt::Display for ArticleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    HackerNews,
    GitHub,
    Reddit(String), // subreddit
    ProductHunt,
    ArXiv,
    Custom(String),
}

impl Source {
    pub fn as_str(&self) -> String {
        match self {
            Source::HackerNews => "hn".to_string(),
            Source::GitHub => "gh".to_string(),
            Source::Reddit(sub) => format!("rd-{}", sub),
            Source::ProductHunt => "ph".to_string(),
            Source::ArXiv => "arxiv".to_string(),
            Source::Custom(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: ArticleId,
    pub title: String,
    pub url: String,
    pub source: Source,
    pub score: f64, // Normalized 0-100
    pub author: String,
    pub timestamp: i64, // Unix timestamp
    pub tags: HashSet<String>,
    pub comment_count: u32,
    pub is_hot_on_source: bool,
}

impl Article {
    pub fn new(
        source: Source,
        native_id: &str,
        title: String,
        url: String,
        timestamp: i64,
    ) -> Self {
        Self {
            id: ArticleId::new(&source, native_id),
            title,
            url,
            source,
            score: 0.0,
            author: "unknown".to_string(),
            timestamp,
            tags: HashSet::new(),
            comment_count: 0,
            is_hot_on_source: false,
        }
    }


    // Simple decaying score calculation example
    pub fn calculate_score(&self, now: i64) -> f64 {
        let age_hours = (now - self.timestamp) as f64 / 3600.0;
        let decay = 1.0 / (age_hours + 2.0).powf(1.8);
        self.score * decay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_id_generation() {
        let id = ArticleId::new(&Source::HackerNews, "12345");
        assert_eq!(id.0, "hn-12345");
    }

    #[test]
    fn test_score_decay() {
        let now = 1700000000;
        let mut article = Article::new(
            Source::HackerNews,
            "1",
            "Test".into(),
            "http://example.com".into(),
            now,
        );
        article.score = 100.0;

        let score_now = article.calculate_score(now);
        let score_1h_later = article.calculate_score(now + 3600);
        let score_24h_later = article.calculate_score(now + 86400);

        // Immediate score should only be affected by the +2.0 offset in formula
        // decay = 1 / (0 + 2)^1.8 = 1 / 3.48 = ~0.28
        // So 100 * 0.28 = 28.7
        assert!(score_now > 0.0);
        
        // Decay should reduce score over time
        assert!(score_1h_later < score_now);
        assert!(score_24h_later < score_1h_later);
    }
}
