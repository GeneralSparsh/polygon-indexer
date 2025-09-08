use axum::{routing::get, Router};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read HOST and PORT from env (from your .env when using dotenv loaders in other bins)
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000);

    let app = Router::new().route("/", get(|| async { "Polygon Indexer server is running." }));

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = TcpListener::bind(addr).await?;
    println!("HTTP server listening on http://{host}:{port}");
    axum::serve(listener, app).await?;
    Ok(())
}
