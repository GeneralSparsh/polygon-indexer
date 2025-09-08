use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    
    #[error("R2D2 pool error: {0}")]
    Pool(#[from] r2d2::Error),
    
    #[error("Ethereum client error: {0}")]
    Ethereum(#[from] ethers::providers::ProviderError),
    
    #[error("Web3 error: {0}")]
    Web3(String),
    
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, IndexerError>;

impl From<anyhow::Error> for IndexerError {
    fn from(err: anyhow::Error) -> Self {
        IndexerError::Generic(err.to_string())
    }
}
