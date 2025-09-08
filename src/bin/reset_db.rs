use anyhow::Result;
use dotenvy::dotenv;
use polygon_indexer::Config;
use std::fs;
use tracing::{info, warn};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    tracing_subscriber::fmt::init();
    
    info!("🗑️  Resetting Polygon Indexer database...");
    
    let config = Config::from_env()?;
    
    if std::path::Path::new(&config.database_url).exists() {
        info!("🔥 Removing existing database: {}", config.database_url);
        fs::remove_file(&config.database_url)?;
        info!("✅ Database removed successfully!");
    } else {
        warn!("⚠️  Database file not found: {}", config.database_url);
    }
    
    info!("🔄 Run 'cargo run --bin setup_db' to recreate the database");
    
    Ok(())
}
