// @generated automatically by Diesel CLI.

diesel::table! {
    transfers (id) {
        id -> Text,
        block_number -> BigInt,
        transaction_hash -> Text,
        from_address -> Text,
        to_address -> Text,
        value -> Text,
        timestamp -> Timestamp,
        is_binance_related -> Bool,
    }
}

diesel::table! {
    net_flows (address) {
        address -> Text,
        net_flow -> Text,
        inflow -> Text,
        outflow -> Text,
        transfer_count -> BigInt,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    system_stats (id) {
        id -> Integer,
        total_transfers -> BigInt,
        binance_transfers -> BigInt,
        total_volume -> Text,
        current_block -> BigInt,
        last_updated -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    transfers,
    net_flows,
    system_stats,
);
