// Infrastructure for External Gateways (APIs)
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use techpulse_domain::article::{Article, Source};
use techpulse_domain::error::DomainError;

#[async_trait]
pub trait ArticleGateway: Send + Sync {
    async fn fetch_top_articles(&self, limit: usize) -> Result<Vec<Article>, DomainError>;
}

#[derive(Debug, Clone)]
pub struct HackerNewsGateway {
    client: Client,
}

impl HackerNewsGateway {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for HackerNewsGateway {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct HnItem {
    id: u64,
    title: Option<String>,
    url: Option<String>,
    time: Option<i64>,
}

#[async_trait]
impl ArticleGateway for HackerNewsGateway {
    async fn fetch_top_articles(&self, limit: usize) -> Result<Vec<Article>, DomainError> {
        // 1. Fetch top story IDs
        let top_ids: Vec<u64> = self
            .client
            .get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .send()
            .await
            .map_err(|e| DomainError::Repository(format!("HN API error: {}", e)))?
            .json()
            .await
            .map_err(|e| DomainError::Repository(format!("HN parse error: {}", e)))?;

        // 2. Fetch each item (limit to avoid too many requests)
        let mut articles = Vec::new();
        for id in top_ids.into_iter().take(limit) {
            let item_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            let item: HnItem = match self.client.get(&item_url).send().await {
                Ok(resp) => match resp.json().await {
                    Ok(item) => item,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            // Skip items without title or url
            let title = match item.title {
                Some(t) => t,
                None => continue,
            };
            let url = item.url.unwrap_or_else(|| {
                format!("https://news.ycombinator.com/item?id={}", item.id)
            });
            let timestamp = item.time.unwrap_or(0);

            if let Ok(article) = Article::new(
                Source::HackerNews,
                &item.id.to_string(),
                title,
                url,
                timestamp,
            ) {
                articles.push(article);
            }
        }

        Ok(articles)
    }
}
