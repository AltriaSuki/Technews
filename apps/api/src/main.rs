use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use techpulse_adapter::http::{routes, AppState};
use techpulse_infra::gateway::HackerNewsGateway;
use techpulse_infra::repo::db::{SqliteArticleRepo, SqliteTrendRepo};
use techpulse_usecase::feed::GetChronologicalFeed;
use techpulse_usecase::ingest::IngestArticles;
use techpulse_usecase::trends::CalculateTrends;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Composition root: construct repositories, gateways, and use cases
    let article_repo = Arc::new(SqliteArticleRepo::new(pool.clone()));
    let trend_repo = Arc::new(SqliteTrendRepo::new(pool.clone()));
    let hn_gateway = Arc::new(HackerNewsGateway::new());

    let state = AppState {
        feed: Arc::new(GetChronologicalFeed::new(article_repo.clone())),
        trends: Arc::new(CalculateTrends::new(article_repo.clone(), trend_repo)),
        ingest: Arc::new(IngestArticles::new(hn_gateway, article_repo)),
    };

    // Initialize routes with state
    let app = routes(state);

    // Start server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
