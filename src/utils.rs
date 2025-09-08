use chrono::{DateTime, Utc, NaiveDateTime};
use ethers::types::{Address, U256};
use bigdecimal::BigDecimal;
use std::str::FromStr;

/// Known Binance addresses on Polygon
pub const BINANCE_ADDRESSES: &[&str] = &[
    "0xf977814e90da44bfa03b6295a0616a897441acec", // Binance 8
    "0x3c783c21a0383057d128bae431894a5c19f9cf06", // Binance 9
    "0x8894e0a0c962cb723c1976a4421c95949be2d4e3", // Binance Hot Wallet
    "0xd551234ae421e3bcba99a0da6d736074f22192ff", // Binance 12
    "0x28c6c06298d514db089934071355e5743bf21d60", // Binance 14
    "0x21a31ee1afc51d94c2efccaa2092ad1028285549", // Binance 15
    "0x564286362092d8e7936f0549571a803b203aaced", // Binance 16
    "0x0681d8db095565fe8a346fa0277bffde9c0edbbf", // Binance 17
    "0xfec6f679e32d45e22736ad09dfdf6e3368704e31", // Binance 18
    "0x4e9ce36e442e55ecd9025b9a6e0d88485d628a67", // Binance 19
];

pub fn is_binance_address(address: &str) -> bool {
    let normalized = address.to_lowercase();
    BINANCE_ADDRESSES.iter().any(|&addr| addr == normalized)
}

pub fn format_address(address: &Address) -> String {
    format!("{:#x}", address).to_lowercase()
}

pub fn u256_to_bigdecimal(value: U256) -> BigDecimal {
    BigDecimal::from_str(&value.to_string()).unwrap_or_default()
}

pub fn wei_to_ether(wei: U256) -> BigDecimal {
    let wei_bd = u256_to_bigdecimal(wei);
    let divisor = BigDecimal::from(10u64.pow(18));
    wei_bd / divisor
}

pub fn generate_transfer_id(tx_hash: &str, log_index: usize) -> String {
    format!("{}_{}", tx_hash, log_index)
}

pub fn current_timestamp() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn current_utc_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

// Helper functions for BigDecimal <-> String conversion
pub fn bigdecimal_to_string(value: &BigDecimal) -> String {
    value.to_string()
}

pub fn string_to_bigdecimal(value: &str) -> BigDecimal {
    BigDecimal::from_str(value).unwrap_or_default()
}

// Helper for arithmetic with string-stored BigDecimal values
pub fn add_bigdecimal_strings(a: &str, b: &BigDecimal) -> String {
    let a_bd = string_to_bigdecimal(a);
    let result = a_bd + b;
    bigdecimal_to_string(&result)
}

pub fn subtract_bigdecimal_strings(a: &str, b: &BigDecimal) -> String {
    let a_bd = string_to_bigdecimal(a);
    let result = a_bd - b;
    bigdecimal_to_string(&result)
}
