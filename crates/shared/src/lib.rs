// Shared utilities
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    Internal,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_port: u16,
}
