// Infrastructure for External Gateways (APIs)
use async_trait::async_trait;
use futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;
use techpulse_domain::article::{Article, Source};
use techpulse_domain::error::DomainError;
use techpulse_domain::gateway::ArticleGateway;

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
            .map_err(|e| DomainError::Gateway(format!("HN API error: {}", e)))?
            .json()
            .await
            .map_err(|e| DomainError::Gateway(format!("HN parse error: {}", e)))?;

        // 2. Fetch items in parallel
        // Use a limit on concurrency if needed, but for now strict limit is fine
        let fetch_futures = top_ids.into_iter().take(limit).map(|id| {
            let client = self.client.clone();
            async move {
                let item_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
                match client.get(&item_url).send().await {
                    Ok(resp) => match resp.json::<HnItem>().await {
                        Ok(item) => Some(item),
                        Err(e) => {
                            tracing::warn!("Failed to parse HN item {}: {}", id, e);
                            None
                        }
                    },
                    Err(e) => {
                        tracing::warn!("Failed to fetch HN item {}: {}", id, e);
                        None
                    }
                }
            }
        });

        // Parallel execution
        let results = join_all(fetch_futures).await;

        // 3. Convert to domain articles
        let articles: Vec<Article> = results
            .into_iter()
            .filter_map(|item| item)
            .filter_map(|item| {
                let title = item.title?;
                let url = item
                    .url
                    .unwrap_or_else(|| format!("https://news.ycombinator.com/item?id={}", item.id));
                let timestamp = item.time.unwrap_or(0);

                Article::new(
                    Source::HackerNews,
                    &item.id.to_string(),
                    title,
                    url,
                    timestamp,
                )
                .ok()
            })
            .collect();

        Ok(articles)
    }
}
