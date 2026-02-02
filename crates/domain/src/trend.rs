// Domain entities for Trends
use crate::article::ArticleId;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimelineEventId(String);

impl From<&str> for TimelineEventId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub keyword: String,
    pub score: f64,
    pub volume: u32,
    pub velocity: f64, // Change in volume/score over time
    pub related_articles: Vec<ArticleId>, // Article IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: TimelineEventId,
    pub title: String,
    pub date: NaiveDate, 
    pub description: String,
    pub category: String,
    pub importance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrendReport {
    pub timestamp: i64,
    pub trends: Vec<Trend>,
    pub metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_trend_serialization() {
        let trend = Trend {
            keyword: "rust".to_string(),
            score: 1.5,
            volume: 100,
            velocity: 0.5,
            related_articles: vec![],
        };
        
        let json = serde_json::to_string(&trend).unwrap();
        let decoded: Trend = serde_json::from_str(&json).unwrap();
        
        assert_eq!(decoded.keyword, "rust");
        assert_eq!(decoded.volume, 100);
    }

    #[test]
    fn test_timeline_event_creation() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let event = TimelineEvent {
            id: TimelineEventId::from("event-1"),
            title: "Launch".into(),
            date,
            description: "Big launch".into(),
            category: "Tech".into(),
            importance_score: 10.0,
        };
        
        assert_eq!(event.date.year(), 2023);
    }
}
