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
    let source_str: String = row.try_get("source")
        .map_err(|e| DomainError::Repository(format!("Missing source: {}", e)))?;
        
    let source = match source_str.as_str() {
        "hn" => Source::HackerNews,
        "gh" => Source::GitHub,
        "ph" => Source::ProductHunt,
        "arxiv" => Source::ArXiv,
        s if s.starts_with("rd-") => Source::Reddit(s[3..].to_string()),
        _ => Source::Custom(source_str),
    };
    
    let tags_str: String = row.try_get("tags").unwrap_or_else(|_| "[]".to_string());
    let tags: HashSet<String> = serde_json::from_str(&tags_str)
        .unwrap_or_default();

    let id_str: String = row.try_get("id")
        .map_err(|e| DomainError::Repository(format!("Missing id: {}", e)))?;

    Ok(Article {
        id: ArticleId::from_persisted(id_str),
        title: row.try_get("title").map_err(|e| DomainError::Repository(format!("Missing title: {}", e)))?,
        url: row.try_get("url").unwrap_or_default(), // Can be empty per migration default
        source,
        score: row.try_get("score").unwrap_or_default(),
        author: row.try_get("author").unwrap_or_default(),
        timestamp: row.try_get("timestamp").map_err(|e| DomainError::Repository(format!("Missing timestamp: {}", e)))?,
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

        // Note: report.timestamp is just a value in the column, not PK anymore
        sqlx::query(
            r#"
            INSERT INTO trends (timestamp, data, metadata)
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
        // Use timestamp index to find latest
        let row = sqlx::query("SELECT * FROM trends ORDER BY timestamp DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Repository(e.to_string()))?;

        if let Some(row) = row {
            let timestamp: i64 = row.try_get("timestamp").map_err(|e| DomainError::Repository(format!("Missing timestamp: {}", e)))?;
            let data_str: String = row.try_get("data").map_err(|e| DomainError::Repository(format!("Missing data: {}", e)))?;
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
