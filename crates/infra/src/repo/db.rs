use async_trait::async_trait;
use sqlx::{Pool, Sqlite, Row};
use techpulse_domain::article::{Article, ArticleId, Source};
use techpulse_domain::error::DomainError;
use techpulse_domain::repository::{ArticleRepo, TrendRepo};
use techpulse_domain::trend::{TrendReport, Trend};
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone)]
pub struct SqliteArticleRepo {
    pool: Pool<Sqlite>,
}

impl SqliteArticleRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticleRepo for SqliteArticleRepo {
    async fn save(&self, article: &Article) -> Result<(), DomainError> {
        let tags_json = serde_json::to_string(&article.tags)
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO articles (id, title, url, source, score, author, timestamp, tags, comment_count, is_hot_on_source)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(article.id.to_string())
        .bind(&article.title)
        .bind(&article.url)
        .bind(article.source.to_string())
        .bind(article.score)
        .bind(&article.author)
        .bind(article.timestamp)
        .bind(tags_json)
        .bind(article.comment_count as i64)
        .bind(article.is_hot_on_source)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &ArticleId) -> Result<Option<Article>, DomainError> {
        let row = sqlx::query("SELECT * FROM articles WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        if let Some(row) = row {
            Ok(Some(map_row_to_article(&row)?))
        } else {
            Ok(None)
        }
    }

    async fn find_latest(&self, limit: usize) -> Result<Vec<Article>, DomainError> {
        let rows = sqlx::query("SELECT * FROM articles ORDER BY timestamp DESC LIMIT ?")
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        let mut articles = Vec::new();
        for row in rows {
            articles.push(map_row_to_article(&row)?);
        }
        Ok(articles)
    }
}

fn map_row_to_article(row: &sqlx::sqlite::SqliteRow) -> Result<Article, DomainError> {
    let source_str: String = row.try_get("source").unwrap_or_default();
    let source = match source_str.as_str() {
        "hn" => Source::HackerNews,
        _ => Source::Custom(source_str),
    };
    
    let tags_str: String = row.try_get("tags").unwrap_or_else(|_| "[]".to_string());
    let tags: HashSet<String> = serde_json::from_str(&tags_str)
        .unwrap_or_default();

    Ok(Article {
        id: ArticleId::from(row.try_get::<String, _>("id").unwrap_or_default()),
        title: row.try_get("title").unwrap_or_default(),
        url: row.try_get("url").unwrap_or_default(),
        source,
        score: row.try_get("score").unwrap_or_default(),
        author: row.try_get("author").unwrap_or_default(),
        timestamp: row.try_get("timestamp").unwrap_or_default(),
        tags,
        comment_count: row.try_get::<i64, _>("comment_count").unwrap_or_default() as u32,
        is_hot_on_source: row.try_get("is_hot_on_source").unwrap_or_default(),
    })
}

#[derive(Debug, Clone)]
pub struct SqliteTrendRepo {
    pool: Pool<Sqlite>,
}

impl SqliteTrendRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TrendRepo for SqliteTrendRepo {
    async fn save_report(&self, report: &TrendReport) -> Result<(), DomainError> {
        let data = serde_json::to_string(&report.trends)
            .map_err(|e| DomainError::Repository(format!("Serialization error: {}", e)))?;
        let metadata = serde_json::to_string(&report.metadata)
            .map_err(|e| DomainError::Repository(format!("Serialization error: {}", e)))?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO trends (timestamp, data, metadata)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(report.timestamp)
        .bind(data)
        .bind(metadata)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Repository(e.to_string()))?;

        Ok(())
    }

    async fn find_latest_report(&self) -> Result<Option<TrendReport>, DomainError> {
        let row = sqlx::query("SELECT * FROM trends ORDER BY timestamp DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        if let Some(row) = row {
            let timestamp: i64 = row.try_get("timestamp").unwrap_or_default();
            let data_str: String = row.try_get("data").unwrap_or_default();
            let trends: Vec<Trend> = serde_json::from_str(&data_str)
                .map_err(|e| DomainError::Repository(format!("Deserialization error: {}", e)))?;
            
            let metadata_str: String = row.try_get("metadata").unwrap_or_else(|_| "{}".to_string());
            let metadata: HashMap<String, String> = serde_json::from_str(&metadata_str)
                .unwrap_or_default();

            Ok(Some(TrendReport { timestamp, trends, metadata }))
        } else {
            Ok(None)
        }
    }
}
