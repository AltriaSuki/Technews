use std::sync::Arc;
use techpulse_adapter::http::{routes, AppState};
use techpulse_infra::repo::mem::{InMemoryArticleRepo, InMemoryTrendRepo};
use techpulse_usecase::feed::GetChronologicalFeed;
use techpulse_usecase::trends::CalculateTrends;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Composition root: construct repositories and use cases
    let article_repo = Arc::new(InMemoryArticleRepo::new());
    let trend_repo = Arc::new(InMemoryTrendRepo::new());

    let state = AppState {
        feed: Arc::new(GetChronologicalFeed::new(article_repo.clone())),
        trends: Arc::new(CalculateTrends::new(article_repo, trend_repo)),
    };

    // Initialize routes with state
    let app = routes(state);

    // Start server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
