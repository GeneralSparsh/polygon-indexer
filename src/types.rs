use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use ethers::types::{Address, H256, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub block_number: i64,
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub value: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub is_binance_related: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetFlowData {
    pub address: String,
    pub net_flow: BigDecimal,
    pub inflow: BigDecimal,
    pub outflow: BigDecimal,
    pub transfer_count: i64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_transfers: i64,
    pub binance_transfers: i64,
    pub total_volume: BigDecimal,
    pub current_block: i64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub number: U256,
    pub hash: H256,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub block_number: u64,
    pub transaction_hash: H256,
    pub timestamp: DateTime<Utc>,
}
