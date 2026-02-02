use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use techpulse_domain::article::Article;
use techpulse_domain::trend::TrendReport;
use techpulse_usecase::feed::GetChronologicalFeed;
use techpulse_usecase::trends::CalculateTrends;

#[derive(Clone)]
pub struct AppState {
    pub feed: Arc<GetChronologicalFeed>,
    pub trends: Arc<CalculateTrends>,
}

pub fn routes(state: AppState) -> Router {
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
            source: a.source.to_string(),
            timestamp: a.timestamp,
        }
    }
}

pub struct ApiError(String);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

async fn get_feed(
    State(state): State<AppState>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<FeedResponse>, ApiError> {
    let limit = params.limit.min(100).max(1);
    let articles = state
        .feed
        .execute(limit)
        .await
        .map_err(|e| ApiError(e.to_string()))?;
    Ok(Json(FeedResponse {
        articles: articles.into_iter().map(ArticleDto::from).collect(),
    }))
}

#[derive(Deserialize)]
pub struct TrendsRequest {
    #[serde(default = "default_keywords")]
    keywords: Vec<String>,
}

fn default_keywords() -> Vec<String> {
    vec![
        "Rust".to_string(),
        "AI".to_string(),
        "Cloud".to_string(),
        "Crypto".to_string(),
    ]
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

async fn calculate_trends(
    State(state): State<AppState>,
    Json(body): Json<TrendsRequest>,
) -> Result<Json<TrendsResponse>, ApiError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let report: TrendReport = state
        .trends
        .execute(&body.keywords, now)
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    Ok(Json(TrendsResponse {
        timestamp: report.timestamp,
        trends: report
            .trends
            .into_iter()
            .map(|t| TrendDto {
                keyword: t.keyword,
                score: t.score,
                volume: t.volume,
            })
            .collect(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;
    use techpulse_infra::repo::mem::{InMemoryArticleRepo, InMemoryTrendRepo};

    fn test_state() -> AppState {
        let article_repo = Arc::new(InMemoryArticleRepo::new());
        let trend_repo = Arc::new(InMemoryTrendRepo::new());
        AppState {
            feed: Arc::new(GetChronologicalFeed::new(article_repo.clone())),
            trends: Arc::new(CalculateTrends::new(article_repo, trend_repo)),
        }
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = routes(test_state());
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_feed_endpoint() {
        let app = routes(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/feed?limit=5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_trends_endpoint() {
        let app = routes(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/trends/calculate")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"keywords":["Rust"]}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
