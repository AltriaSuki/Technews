// Domain entities for Trends
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub keyword: String,
    pub score: f64,
}
