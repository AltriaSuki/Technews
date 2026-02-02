use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    NotFound(String),
    Validation(String),
    Repository(String),
    AlreadyExists(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound(msg) => write!(f, "Entity not found: {}", msg),
            DomainError::Validation(msg) => write!(f, "Validation error: {}", msg),
            DomainError::Repository(msg) => write!(f, "Repository error: {}", msg),
            DomainError::AlreadyExists(msg) => write!(f, "Entity already exists: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}
