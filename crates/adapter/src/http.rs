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
use techpulse_domain::error::DomainError;
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

#[derive(Serialize, Deserialize)]
pub struct FeedResponse {
    pub articles: Vec<ArticleDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleDto {
    pub id: String,
    pub title: String,
    pub url: String,
    pub source: String,
    pub timestamp: i64,
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

pub struct ApiError(DomainError);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // Log actual error for debugging
        tracing::error!("API error: {:?}", self.0);
        
        let status = match &self.0 {
            DomainError::NotFound(_) => StatusCode::NOT_FOUND,
            DomainError::Validation(_) => StatusCode::BAD_REQUEST,
            DomainError::AlreadyExists(_) => StatusCode::CONFLICT,
            DomainError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        // Return generic message, don't leak internals
        let message = match status {
            StatusCode::NOT_FOUND => "Not found",
            StatusCode::BAD_REQUEST => "Bad request",
            StatusCode::CONFLICT => "Conflict",
            _ => "Internal server error",
        };
        
        (status, message).into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self {
        ApiError(e)
    }
}

async fn get_feed(
    State(state): State<AppState>,
    Query(params): Query<FeedQuery>,
) -> Result<Json<FeedResponse>, ApiError> {
    let limit = params.limit.min(100).max(1);
    let articles = state.feed.execute(limit).await?;
    Ok(Json(FeedResponse {
        articles: articles.into_iter().map(ArticleDto::from).collect(),
    }))
}

#[derive(Deserialize)]
pub struct TrendsRequest {
    #[serde(default = "default_keywords")]
    pub keywords: Vec<String>,
}

impl Default for TrendsRequest {
    fn default() -> Self {
        Self { keywords: default_keywords() }
    }
}

fn default_keywords() -> Vec<String> {
    vec![
        "Rust".to_string(),
        "AI".to_string(),
        "Cloud".to_string(),
        "Crypto".to_string(),
    ]
}

#[derive(Serialize, Deserialize)]
pub struct TrendsResponse {
    pub timestamp: i64,
    pub trends: Vec<TrendDto>,
}

#[derive(Serialize, Deserialize)]
pub struct TrendDto {
    pub keyword: String,
    pub score: f64,
    pub volume: u32,
}

async fn calculate_trends(
    State(state): State<AppState>,
    body: Option<Json<TrendsRequest>>,
) -> Result<Json<TrendsResponse>, ApiError> {
    let request = body.map(|b| b.0).unwrap_or_default();
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let report: TrendReport = state.trends.execute(&request.keywords, now).await?;

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
    use http_body_util::BodyExt;
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
        
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let feed: FeedResponse = serde_json::from_slice(&body).unwrap();
        assert!(feed.articles.is_empty()); // Empty in-memory repo
    }

    #[tokio::test]
    async fn test_trends_endpoint_with_body() {
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
        
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let trends: TrendsResponse = serde_json::from_slice(&body).unwrap();
        assert!(trends.trends.is_empty()); // No articles to match
    }
    
    #[tokio::test]
    async fn test_trends_endpoint_without_body() {
        let app = routes(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/trends/calculate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
