-- This file should undo anything in up.sql
"@

Write-Host "✅ Created initial setup migration"


Set-Content -Path "migrations\2023090801_create_tables\up.sql" -Encoding UTF8 -Value @"
-- Create transfers table
CREATE TABLE transfers (
    id TEXT PRIMARY KEY,
    block_number BIGINT NOT NULL,
    transaction_hash TEXT NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    value TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    is_binance_related BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create net_flows table
CREATE TABLE net_flows (
    address TEXT PRIMARY KEY,
    net_flow TEXT NOT NULL DEFAULT '0',
    inflow TEXT NOT NULL DEFAULT '0',
    outflow TEXT NOT NULL DEFAULT '0',
    transfer_count BIGINT NOT NULL DEFAULT 0,
    last_updated DATETIME NOT NULL
);

-- Create system_stats table
CREATE TABLE system_stats (
    id INTEGER PRIMARY KEY,
    total_transfers BIGINT NOT NULL DEFAULT 0,
    binance_transfers BIGINT NOT NULL DEFAULT 0,
    total_volume TEXT NOT NULL DEFAULT '0',
    current_block BIGINT NOT NULL DEFAULT 0,
    last_updated DATETIME NOT NULL
);

-- Create indexes for better performance
CREATE INDEX idx_transfers_block_number ON transfers(block_number);
CREATE INDEX idx_transfers_from_address ON transfers(from_address);
CREATE INDEX idx_transfers_to_address ON transfers(to_address);
CREATE INDEX idx_transfers_is_binance ON transfers(is_binance_related);
CREATE INDEX idx_transfers_timestamp ON transfers(timestamp);
CREATE INDEX idx_net_flows_last_updated ON net_flows(last_updated);
