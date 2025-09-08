use serde::{Deserialize, Serialize};
use std::env;
use crate::{IndexerError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub polygon_rpc_url: String,
    pub polygon_ws_url: String,
    pub pol_contract: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "data/indexer.db".to_string()),
            polygon_rpc_url: env::var("POLYGON_RPC_URL")
                .unwrap_or_else(|_| "https://polygon-rpc.com/".to_string()),
            polygon_ws_url: env::var("POLYGON_WS_URL")
                .unwrap_or_else(|_| "wss://rpc-mainnet.matic.network".to_string()),
            pol_contract: env::var("POL_CONTRACT")
                .unwrap_or_else(|_| "0x0000000000000000000000000000000000001010".to_string())
                .to_lowercase(),
            host: env::var("HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|e| IndexerError::Config(format!("Invalid PORT: {}", e)))?,
        })
    }
}
