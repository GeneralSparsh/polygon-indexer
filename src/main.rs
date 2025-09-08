use anyhow::Result;
use dotenvy::dotenv;
use polygon_indexer::{config::Config, database::create_pool, indexer::PolygonIndexer, server::Server};
use std::sync::Arc;
use tokio::signal;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "polygon_indexer=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🌌 Starting Polygon POL Token Indexer");

    let config = Config::from_env()?;
    let pool = create_pool(&config.database_url)?;

    polygon_indexer::database::run_migrations(&mut pool.get()?)?;

    let indexer = Arc::new(PolygonIndexer::new(config.clone(), pool.clone()).await?);
    let server = Server::new(config.clone(), pool.clone(), indexer.clone());

    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            warn!("❌ Server error: {}", e);
        }
    });

    let indexer_handle = {
        let indexer = indexer.clone();
        tokio::spawn(async move {
            if let Err(e) = indexer.start().await {
                warn!("❌ Indexer error: {}", e);
            }
        })
    };

    info!("🚀 Polygon Indexer running at http://{}:{}", config.host, config.port);

    match signal::ctrl_c().await {
        Ok(()) => info!("🛑 Shutdown signal received"),
        Err(err) => warn!("❌ Unable to listen for shutdown signal: {}", err),
    }

    indexer.stop().await?;
    server_handle.abort();
    indexer_handle.abort();

    info!("👋 Polygon Indexer stopped gracefully");
    Ok(())
}
