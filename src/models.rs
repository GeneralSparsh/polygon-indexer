use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = transfers)]
pub struct Transfer {
    pub id: String,
    pub block_number: i64,
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String, // Store as string for SQLite
    pub timestamp: chrono::NaiveDateTime, // Use NaiveDateTime for SQLite
    pub is_binance_related: bool,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = net_flows)]
pub struct NetFlow {
    pub address: String,
    pub net_flow: String, // Store as string for SQLite
    pub inflow: String,   // Store as string for SQLite
    pub outflow: String,  // Store as string for SQLite
    pub transfer_count: i64,
    pub last_updated: chrono::NaiveDateTime, // Use NaiveDateTime for SQLite
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = system_stats)]
pub struct SystemStat {
    pub id: i32,
    pub total_transfers: i64,
    pub binance_transfers: i64,
    pub total_volume: String, // Store as string for SQLite
    pub current_block: i64,
    pub last_updated: chrono::NaiveDateTime, // Use NaiveDateTime for SQLite
}

impl Transfer {
    pub fn new(
        id: String,
        block_number: i64,
        transaction_hash: String,
        from_address: String,
        to_address: String,
        value: String,
        timestamp: chrono::NaiveDateTime,
        is_binance_related: bool,
    ) -> Self {
        Self {
            id,
            block_number,
            transaction_hash,
            from_address,
            to_address,
            value,
            timestamp,
            is_binance_related,
        }
    }
}
