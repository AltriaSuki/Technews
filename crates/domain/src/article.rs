// Domain entities for Articles
use crate::error::DomainError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArticleId(String);

impl ArticleId {
    pub fn new(source: &Source, native_id: &str) -> Result<Self, DomainError> {
        if native_id.contains('-') {
            return Err(DomainError::Validation(
                "Native ID cannot contain '-' separator".to_string(),
            ));
        }
        
        // Prevent aliasing: Custom sources cannot look like standard ones
        match source {
            Source::Custom(s) => {
                if s.trim().is_empty() {
                     return Err(DomainError::Validation(
                        "Custom source name cannot be empty".to_string(),
                    ));
                }
                if s.contains('-') {
                    return Err(DomainError::Validation(
                        "Custom source name cannot contain '-'".to_string(),
                    ));
                }
                
                const RESERVED_PREFIXES: &[&str] = &["hn", "gh", "ph", "arxiv", "rd"];
                if RESERVED_PREFIXES.contains(&s.as_str()) {
                     return Err(DomainError::Validation(
                        format!("Custom source cannot use reserved prefix '{}'", s)
                    ));
                }
            },
            Source::Reddit(s) if s.contains('-') => {
                 return Err(DomainError::Validation(
                    "Subreddit name cannot contain '-'".to_string(),
                ));
            }
            _ => {}
        }
        
        Ok(Self(format!("{}-{}", source, native_id)))
    }
    /// Reconstruct from a previously-validated stored string.
    /// Only use for DB deserialization — not for creating new IDs.
    pub fn from_persisted(s: String) -> Self {
        Self(s)
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

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::HackerNews => f.write_str("hn"),
            Source::GitHub => f.write_str("gh"),
            Source::Reddit(sub) => write!(f, "rd-{}", sub),
            Source::ProductHunt => f.write_str("ph"),
            Source::ArXiv => f.write_str("arxiv"),
            Source::Custom(s) => f.write_str(s),
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
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: ArticleId::new(&source, native_id)?,
            title,
            url,
            source,
            score: 0.0,
            author: "unknown".to_string(),
            timestamp,
            tags: HashSet::new(),
            comment_count: 0,
            is_hot_on_source: false,
        })
    }

    // Simple decaying score calculation example
    pub fn calculate_score(&self, now: i64) -> f64 {
        // Clamp age to 0 to prevent future-dated articles from getting infinite/huge scores
        let age_hours = ((now - self.timestamp) as f64 / 3600.0).max(0.0);
        let decay = 1.0 / (age_hours + 2.0).powf(1.8);
        
        // Base score affected by comments (simple heuristic)
        // Logarithmic boost from comments to avoid massive skew
        let engagement_boost = (self.comment_count as f64 + 1.0).ln(); 
        
        let hot_multiplier = if self.is_hot_on_source { 1.2 } else { 1.0 };

        (self.score + engagement_boost) * decay * hot_multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_id_generation() {
        let id_result = ArticleId::new(&Source::HackerNews, "12345");
        assert!(id_result.is_ok());
        assert_eq!(id_result.unwrap().0, "hn-12345");
    }

    #[test]
    fn test_article_id_validation() {
        // Native ID cannot contain separator
        assert!(ArticleId::new(&Source::HackerNews, "12-34").is_err());
        
        // Custom source cannot contain separator
        assert!(ArticleId::new(&Source::Custom("my-source".into()), "123").is_err());

        // Custom source cannot use reserved prefix
        assert!(ArticleId::new(&Source::Custom("hn".into()), "123").is_err());
        assert!(ArticleId::new(&Source::Custom("rd".into()), "123").is_err());
        
        // Custom source cannot be empty
        assert!(ArticleId::new(&Source::Custom("".into()), "123").is_err());
        
        // Reddit subreddit cannot contain separator
        assert!(ArticleId::new(&Source::Reddit("sub-reddit".into()), "123").is_err());
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
        ).unwrap();
        article.score = 100.0;
        // Default comment_count=0, hot=false

        let score_now = article.calculate_score(now);
        let score_1h_later = article.calculate_score(now + 3600);
        
        assert!(score_now > 0.0);
        assert!(score_1h_later < score_now);
        
        // Future dated article (negative age) should be treated as age=0
        let score_at_birth = article.calculate_score(now);
        let score_pre_birth = article.calculate_score(now - 3600);
        
        assert!((score_at_birth - score_pre_birth).abs() < 0.001);
    }
    
    #[test]
    fn test_score_boost() {
        let now = 1700000000;
        let mut article = Article::new(
            Source::HackerNews,
            "1",
            "Test".into(),
            "http://example.com".into(),
            now,
        ).unwrap();
        article.score = 50.0;
        
        let base_score = article.calculate_score(now);
        
        article.comment_count = 100;
        let boosted_score = article.calculate_score(now);
        
        assert!(boosted_score > base_score);
        
        article.is_hot_on_source = true;
        let hot_score = article.calculate_score(now);
        
        assert!(hot_score > boosted_score);
    }
}
