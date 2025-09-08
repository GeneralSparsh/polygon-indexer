-- Drop indexes
DROP INDEX IF EXISTS idx_transfers_block_number;
DROP INDEX IF EXISTS idx_transfers_from_address;
DROP INDEX IF EXISTS idx_transfers_to_address;
DROP INDEX IF EXISTS idx_transfers_is_binance;
DROP INDEX IF EXISTS idx_transfers_timestamp;
DROP INDEX IF EXISTS idx_net_flows_last_updated;

-- Drop tables
DROP TABLE IF EXISTS system_stats;
DROP TABLE IF EXISTS net_flows;
DROP TABLE IF EXISTS transfers;
