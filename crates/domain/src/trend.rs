// Domain entities for Trends
use crate::article::ArticleId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub id: String,
    pub title: String,
    pub date: String, // ISO8601 YYYY-MM-DD
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
