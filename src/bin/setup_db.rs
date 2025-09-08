use anyhow::Result;
use dotenvy::dotenv;
use polygon_indexer::{Config, database::{create_pool, run_migrations}};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    tracing_subscriber::fmt::init();
    
    info!("🗄️  Setting up Polygon Indexer database...");
    
    let config = Config::from_env()?;
    
    info!("📁 Creating database: {}", config.database_url);
    let pool = create_pool(&config.database_url)?;
    
    info!("🔄 Running migrations...");
    let mut conn = pool.get()?;
    run_migrations(&mut conn)?;
    
    info!("✅ Database setup completed successfully!");
    info!("🚀 You can now run: cargo run --release");
    
    Ok(())
}
