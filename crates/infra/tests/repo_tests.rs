use sqlx::sqlite::SqlitePoolOptions;
use techpulse_domain::article::{Article, ArticleId, Source};
use techpulse_domain::repository::{ArticleRepo, TrendRepo};
use techpulse_domain::trend::{Trend, TrendReport};
use techpulse_infra::repo::db::{SqliteArticleRepo, SqliteTrendRepo};

#[tokio::test]
async fn test_sqlite_article_roundtrip() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // Run migrations dynamically on the in-memory DB
    // We need to point to the migrations directory relative to where this test runs.
    // In workspace, this usually works from crate root.
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .unwrap();

    let repo = SqliteArticleRepo::new(pool);
    let article = Article::new(
        Source::HackerNews,
        "1",
        "Test Title".into(),
        "http://example.com".into(),
        100,
    )
    .unwrap();

    repo.save(&article).await.unwrap();

    let found = repo.find_by_id(&article.id).await.unwrap().unwrap();
    assert_eq!(found.title, "Test Title");
    assert_eq!(found.source, Source::HackerNews);
    assert_eq!(found.url, "http://example.com");

    let latest = repo.find_latest(10).await.unwrap();
    assert_eq!(latest.len(), 1);
    assert_eq!(latest[0].id, article.id);
}

#[tokio::test]
async fn test_save_all_sources() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();

    let repo = SqliteArticleRepo::new(pool);

    let sources = vec![
        Source::HackerNews,
        Source::GitHub,
        Source::ProductHunt,
        Source::ArXiv,
        Source::Reddit("rust".into()),
        Source::Custom("myblog".into()),
    ];

    for (i, source) in sources.into_iter().enumerate() {
        let article = Article::new(
            source.clone(),
            &i.to_string(),
            format!("Title {}", i),
            "http://x".into(),
            100,
        )
        .unwrap();
        
        repo.save(&article).await.unwrap();
        let found = repo.find_by_id(&article.id).await.unwrap().unwrap();
        assert_eq!(found.source, source, "Failed to match source {:?}", source);
    }
}

#[tokio::test]
async fn test_sqlite_trend_roundtrip() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();

    let repo = SqliteTrendRepo::new(pool);
    
    let trend = Trend {
        keyword: "Runes".into(),
        score: 1.0,
        volume: 10,
        velocity: 0.1,
        related_articles: vec![ArticleId::from_persisted("hn-123".into())],
    };

    let report = TrendReport {
        timestamp: 1000,
        trends: vec![trend],
        metadata: Default::default(),
    };

    repo.save_report(&report).await.unwrap();

    let found = repo.find_latest_report().await.unwrap().unwrap();
    assert_eq!(found.timestamp, 1000);
    assert_eq!(found.trends.len(), 1);
    assert_eq!(found.trends[0].keyword, "Runes");
    
    // Test overwrite prevention (new timestamp = new row due to unique constraint logic if PK was timestamp, 
    // but now PK is ID, so it just inserts new row)
    let report2 = TrendReport {
        timestamp: 2000,
        trends: vec![],
        metadata: Default::default(),
    };
    repo.save_report(&report2).await.unwrap();
    
    let latest = repo.find_latest_report().await.unwrap().unwrap();
    assert_eq!(latest.timestamp, 2000);
}
