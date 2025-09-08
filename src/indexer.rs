use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::{info, warn, error, debug};
use bigdecimal::BigDecimal;

use crate::{
    Config, Result, IndexerError,
    database::DbPool,
    models::{Transfer, NetFlow},
    utils::{is_binance_address, format_address, wei_to_ether, generate_transfer_id, current_timestamp, 
            bigdecimal_to_string, add_bigdecimal_strings, subtract_bigdecimal_strings},
    schema::transfers,
};
use diesel::prelude::*;

pub struct PolygonIndexer {
    config: Config,
    pool: DbPool,
    provider: Arc<Provider<Ws>>,
    current_block: Arc<RwLock<u64>>,
    is_running: Arc<RwLock<bool>>,
}

impl PolygonIndexer {
    pub async fn new(config: Config, pool: DbPool) -> Result<Self> {
        info!("🔗 Connecting to Polygon WebSocket: {}", config.polygon_ws_url);
        
        let provider = Provider::<Ws>::connect(&config.polygon_ws_url)
            .await
            .map_err(|e| IndexerError::Web3(format!("Failed to connect to WebSocket: {}", e)))?;
        
        let provider = Arc::new(provider);
        
        // Get current block number
        let current_block = provider
            .get_block_number()
            .await
            .map_err(IndexerError::Ethereum)?
            .as_u64();
        
        info!("📦 Current block: {}", current_block);
        
        Ok(Self {
            config,
            pool,
            provider,
            current_block: Arc::new(RwLock::new(current_block)),
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn start(&self) -> Result<()> {
        {
            let mut running = self.is_running.write().await;
            *running = true;
        }
        
        info!("⚡ Starting Polygon POL Token Indexer...");
        
        // Start monitoring new blocks
        let provider = self.provider.clone();
        let pool = self.pool.clone();
        let current_block = self.current_block.clone();
        let is_running = self.is_running.clone();
        let pol_contract = self.config.pol_contract.clone();
        
        let mut stream = provider.subscribe_blocks().await.map_err(IndexerError::Ethereum)?;
        
        info!("🔄 Subscribed to new blocks");
        
        while *is_running.read().await {
            tokio::select! {
                Some(block) = stream.next() => {
                    if let Err(e) = self.process_block(&block, &pool, &pol_contract).await {
                        error!("❌ Error processing block {}: {}", block.number.unwrap_or_default(), e);
                    }
                    
                    *current_block.write().await = block.number.unwrap_or_default().as_u64();
                }
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    if !*is_running.read().await {
                        break;
                    }
                }
            }
        }
        
        info!("🛑 Indexer stopped");
        Ok(())
    }

    async fn process_block(&self, block: &Block<H256>, pool: &DbPool, pol_contract: &str) -> Result<()> {
        let block_number = block.number.unwrap_or_default().as_u64();
        debug!("🔍 Processing block: {}", block_number);
        
        // Get all transactions in this block
        let block_with_txs = self.provider
            .get_block_with_txs(block.hash.unwrap())
            .await
            .map_err(IndexerError::Ethereum)?;
            
        if let Some(block_with_txs) = block_with_txs {
            for tx in block_with_txs.transactions {
                // Check if this transaction interacted with POL token
                if let Some(to) = tx.to {
                    if format_address(&to) == pol_contract.to_lowercase() {
                        if let Err(e) = self.process_pol_transaction(&tx, block_number, pool).await {
                            warn!("⚠️ Error processing POL transaction {}: {}", tx.hash, e);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    async fn process_pol_transaction(&self, tx: &Transaction, block_number: u64, pool: &DbPool) -> Result<()> {
        // Get transaction receipt to access logs
        let receipt = self.provider
            .get_transaction_receipt(tx.hash)
            .await
            .map_err(IndexerError::Ethereum)?;
            
        if let Some(receipt) = receipt {
            for (log_index, log) in receipt.logs.iter().enumerate() {
                // Check if this is a Transfer event (topic0 = Transfer event signature)
                if log.topics.len() >= 3 && 
                   log.topics[0] == H256::from_slice(&keccak256("Transfer(address,address,uint256)")) {
                    
                    let from = Address::from(log.topics[1]);
                    let to = Address::from(log.topics[2]);
                    let value = U256::from_big_endian(&log.data);
                    
                    let from_str = format_address(&from);
                    let to_str = format_address(&to);
                    
                    // Check if this involves Binance
                    let is_binance_related = is_binance_address(&from_str) || is_binance_address(&to_str);
                    
                    if is_binance_related {
                        info!("💰 Binance-related POL transfer detected: {} -> {} ({})", 
                              from_str, to_str, wei_to_ether(value));
                    }
                    
                    // Store the transfer
                    let transfer = Transfer {
                        id: generate_transfer_id(&format!("{:#x}", tx.hash), log_index),
                        block_number: block_number as i64,
                        transaction_hash: format!("{:#x}", tx.hash),
                        from_address: from_str.clone(),
                        to_address: to_str.clone(),
                        value: bigdecimal_to_string(&wei_to_ether(value)),
                        timestamp: current_timestamp(),
                        is_binance_related,
                    };
                    
                    self.store_transfer(transfer, pool).await?;
                    
                    // Update net flows for Binance addresses
                    if is_binance_related {
                        self.update_net_flows(&from_str, &to_str, &wei_to_ether(value), pool).await?;
                    }
                }
            }
        }
        
        Ok(())
    }

    async fn store_transfer(&self, transfer: Transfer, pool: &DbPool) -> Result<()> {
        let mut conn = pool.get()?;
        
        diesel::insert_into(transfers::table)
            .values(&transfer)
            .execute(&mut conn)?;
            
        Ok(())
    }

    async fn update_net_flows(&self, from: &str, to: &str, value: &BigDecimal, pool: &DbPool) -> Result<()> {
        let mut conn = pool.get()?;
        
        // Update sender (outflow)
        if is_binance_address(from) {
            self.update_address_flow(from, value, true, &mut conn).await?;
        }
        
        // Update receiver (inflow) 
        if is_binance_address(to) {
            self.update_address_flow(to, value, false, &mut conn).await?;
        }
        
        Ok(())
    }

    async fn update_address_flow(&self, addr: &str, value: &BigDecimal, is_outflow: bool, conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>) -> Result<()> {
        use crate::schema::net_flows::dsl::*;
        
        // Try to get existing record
        let existing: Option<NetFlow> = net_flows
            .filter(address.eq(addr))
            .first(conn)
            .optional()?;
            
        let zero = BigDecimal::from(0);
        
        if let Some(mut flow) = existing {
            // Update existing record using string arithmetic
            if is_outflow {
                flow.outflow = add_bigdecimal_strings(&flow.outflow, value);
                flow.net_flow = subtract_bigdecimal_strings(&flow.net_flow, value);
            } else {
                flow.inflow = add_bigdecimal_strings(&flow.inflow, value);
                flow.net_flow = add_bigdecimal_strings(&flow.net_flow, value);
            }
            flow.transfer_count += 1;
            flow.last_updated = current_timestamp();
            
            diesel::update(net_flows.filter(address.eq(addr)))
                .set((
                    net_flow.eq(&flow.net_flow),
                    inflow.eq(&flow.inflow),
                    outflow.eq(&flow.outflow),
                    transfer_count.eq(flow.transfer_count),
                    last_updated.eq(flow.last_updated),
                ))
                .execute(conn)?;
        } else {
            // Create new record
            let (new_inflow, new_outflow, new_net_flow) = if is_outflow {
                (bigdecimal_to_string(&zero), bigdecimal_to_string(value), bigdecimal_to_string(&(-value)))
            } else {
                (bigdecimal_to_string(value), bigdecimal_to_string(&zero), bigdecimal_to_string(value))
            };
            
            let new_flow = NetFlow {
                address: addr.to_string(),
                net_flow: new_net_flow,
                inflow: new_inflow,
                outflow: new_outflow,
                transfer_count: 1,
                last_updated: current_timestamp(),
            };
            
            diesel::insert_into(net_flows)
                .values(&new_flow)
                .execute(conn)?;
        }
        
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut running = self.is_running.write().await;
        *running = false;
        info!("🛑 Stopping indexer...");
        Ok(())
    }

    pub async fn get_current_block(&self) -> u64 {
        *self.current_block.read().await
    }
}

fn keccak256(data: &str) -> [u8; 32] {
    use ethers::utils::keccak256 as k256;
    k256(data.as_bytes())
}
