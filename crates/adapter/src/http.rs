use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use techpulse_domain::article::Article;
use techpulse_domain::trend::TrendReport;
use techpulse_infra::repo::mem::{InMemoryArticleRepo, InMemoryTrendRepo};
use techpulse_usecase::feed::GetChronologicalFeed;
use techpulse_usecase::trends::CalculateTrends;

#[derive(Clone)]
pub struct AppState {
    pub feed: Arc<GetChronologicalFeed>,
    pub trends: Arc<CalculateTrends>,
}

impl AppState {
    pub fn new() -> Self {
        let article_repo = Arc::new(InMemoryArticleRepo::new());
        let trend_repo = Arc::new(InMemoryTrendRepo::new());
        
        Self {
            feed: Arc::new(GetChronologicalFeed::new(article_repo.clone())),
            trends: Arc::new(CalculateTrends::new(article_repo, trend_repo)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn routes() -> Router {
    let state = AppState::new();
    
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/feed", get(get_feed))
        .route("/api/trends/calculate", post(calculate_trends))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct FeedQuery {
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    20
}

#[derive(Serialize)]
pub struct FeedResponse {
    articles: Vec<ArticleDto>,
}

#[derive(Serialize)]
pub struct ArticleDto {
    id: String,
    title: String,
    url: String,
    source: String,
    timestamp: i64,
}

impl From<Article> for ArticleDto {
    fn from(a: Article) -> Self {
        Self {
            id: a.id.to_string(),
            title: a.title,
            url: a.url,
            source: format!("{:?}", a.source),
            timestamp: a.timestamp,
        }
    }
}

async fn get_feed(
    State(state): State<AppState>,
    Query(params): Query<FeedQuery>,
) -> Json<FeedResponse> {
    let articles = state.feed.execute(params.limit).await.unwrap_or_default();
    Json(FeedResponse {
        articles: articles.into_iter().map(ArticleDto::from).collect(),
    })
}

#[derive(Serialize)]
pub struct TrendsResponse {
    timestamp: i64,
    trends: Vec<TrendDto>,
}

#[derive(Serialize)]
pub struct TrendDto {
    keyword: String,
    score: f64,
    volume: u32,
}

async fn calculate_trends(State(state): State<AppState>) -> Json<TrendsResponse> {
    let keywords = vec![
        "Rust".to_string(),
        "AI".to_string(),
        "Cloud".to_string(),
        "Crypto".to_string(),
    ];
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
        
    let report: TrendReport = state.trends.execute(&keywords, now).await.unwrap_or_default();
    
    Json(TrendsResponse {
        timestamp: report.timestamp,
        trends: report.trends.into_iter().map(|t| TrendDto {
            keyword: t.keyword,
            score: t.score,
            volume: t.volume,
        }).collect(),
    })
}
