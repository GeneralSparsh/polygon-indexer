use axum::{
extract::{Query, State, WebSocketUpgrade, ws::{WebSocket, Message}},
response::{Html, IntoResponse, Response},
routing::{get, get_service},
Json, Router,
};
use tower::ServiceBuilder;
use tower_http::{
cors::CorsLayer,
trace::TraceLayer,
services::ServeDir,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::{sync::broadcast, time::{interval, Duration}};
use tracing::info;
use diesel::prelude::*;
use futures_util::{SinkExt, StreamExt};

use crate::{
Config, IndexerError,
database::DbPool,
indexer::PolygonIndexer,
models::{Transfer, NetFlow},
types::{NetFlowData, SystemStats},
utils::{string_to_bigdecimal, current_utc_timestamp},
};
// NOTE: Do NOT import crate::server::{...}. The handlers are defined in this same file,
// so they are already in scope and can be referenced directly in the Router.


#[derive(Clone)]
pub struct ServerState {
    pub config: Config,
    pub pool: DbPool,
    pub indexer: Arc<PolygonIndexer>,
    pub broadcast: broadcast::Sender<String>,
}

pub struct Server {
    state: ServerState,
}

#[derive(Deserialize)]
pub struct TransferQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    binance_only: Option<bool>,
}

impl Server {
    pub fn new(config: Config, pool: DbPool, indexer: Arc<PolygonIndexer>) -> Self {
        let (broadcast, _) = broadcast::channel(1000);

        let state = ServerState {
            config,
            pool,
            indexer,
            broadcast,
        };

        Self { state }
    }

    pub async fn start(self) -> crate::Result<()> {
        let addr = format!("{}:{}", self.state.config.host, self.state.config.port);
        info!("🌐 Starting web server on {}", addr);

        // Serve ./ui as site root
        let static_files = get_service(ServeDir::new("ui/dist").append_index_html_on_directories(true));

        // Build router
        let app = Router::new()
            .route("/ws", get(websocket_handler))
            .route("/api/health", get(health_check))
            .route("/api/transfers", get(get_transfers))
            .route("/api/netflow", get(get_net_flow))
            .route("/api/stats", get(get_stats))
            .nest_service("/", static_files)
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::very_permissive())
            )
            .with_state(self.state.clone());

        // WS broadcast task
        let broadcast_state = self.state.clone();
        tokio::spawn(async move {
            websocket_broadcast_task(broadcast_state).await;
        });

        // Start server
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        info!("🚀 Server listening on http://{}", addr);

        axum::serve(listener, app)
            .await
            .map_err(|e| IndexerError::Generic(format!("Server error: {}", e)))?;

        Ok(())
    }
}

async fn serve_index() -> Html<&'static str> {
    let html = r#"<!doctype html><html><body>Static UI is served from ./ui. Replace or remove serve_index.</body></html>"#;
    Html(html)
}

async fn websocket_broadcast_task(state: ServerState) {
    // broadcast updates every 5 seconds; adjust as desired
    let mut ticker = interval(Duration::from_secs(5));

    loop {
        ticker.tick().await;

        // Gather current stats and broadcast to all subscribers
        match get_current_stats(&state).await {
            Ok(stats) => {
                if let Ok(payload) = serde_json::to_string(&serde_json::json!({
                    "type": "stats_update",
                    "data": stats
                })) {
                    // Ignore send errors (no active subscribers)
                    let _ = state.broadcast.send(payload);
                }
            }
            Err(err) => {
                // Silently continue on errors; optionally log if you have tracing set up
                tracing::warn!("websocket_broadcast_task: stats error: {err}");
            }
        }
    }
}
async fn get_current_stats(state: &ServerState) -> crate::Result<SystemStats> {
    use crate::schema::transfers::dsl::*;

    // DB connection
    let mut conn = state.pool.get()?;

    // Total transfers
    let total_transfers: i64 = transfers.count().get_result(&mut conn)?;

    // Binance-related transfers
    let binance_transfers: i64 = transfers
        .filter(is_binance_related.eq(true))
        .count()
        .get_result(&mut conn)?;

    // Current block from indexer
    let current_block = state.indexer.get_current_block().await;

    // Build stats; adjust total_volume/uptime as needed
    Ok(SystemStats {
        total_transfers,
        binance_transfers,
        total_volume: bigdecimal::BigDecimal::from(0),
        current_block: current_block as i64,
        uptime_seconds: 0,
    })
}

async fn get_stats(
    State(state): State<ServerState>,
) -> impl IntoResponse {
    use crate::schema::transfers::dsl::*;

    // Get a DB connection
    let mut conn = match state.pool.get() {
        Ok(c) => c,
        Err(e) => return Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    };

    // Aggregate totals
    let total_transfers: i64 = match transfers.count().get_result(&mut conn) {
        Ok(count) => count,
        Err(e) => return Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    };

    let binance_transfers: i64 = match transfers
        .filter(is_binance_related.eq(true))
        .count()
        .get_result(&mut conn)
    {
        Ok(count) => count,
        Err(e) => return Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    };

    // Current head from indexer
    let current_block = state.indexer.get_current_block().await;

    // Build response payload (adjust fields as your SystemStats requires)
    let stats = SystemStats {
        total_transfers,
        binance_transfers,
        total_volume: bigdecimal::BigDecimal::from(0),
        current_block: current_block as i64,
        uptime_seconds: 0,
    };

    Json(stats).into_response()
}
async fn get_net_flow(
    State(state): State<ServerState>,
) -> impl IntoResponse {
    use crate::schema::net_flows::dsl::*;

    // Acquire a DB connection
    let mut conn = match state.pool.get() {
        Ok(c) => c,
        Err(e) => return Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    };

    // Load all net flow rows ordered by last_updated (tweak as needed)
    let rows: Vec<NetFlow> = match net_flows
        .order(last_updated.desc())
        .load(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    };

    // Map DB rows to API response type
    let data: Vec<NetFlowData> = rows
        .into_iter()
        .map(|row| NetFlowData {
            address: row.address,
            net_flow: string_to_bigdecimal(&row.net_flow),
            inflow: string_to_bigdecimal(&row.inflow),
            outflow: string_to_bigdecimal(&row.outflow),
            transfer_count: row.transfer_count,
            last_updated: chrono::DateTime::from_naive_utc_and_offset(row.last_updated, chrono::Utc),
        })
        .collect();

    Json(data).into_response()
}
async fn get_transfers(
    Query(query): Query<TransferQuery>,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    use crate::schema::transfers::dsl::*;

    // Get a DB connection
    let mut conn = match state.pool.get() {
        Ok(c) => c,
        Err(e) => return Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    };

    // Pagination defaults and bounds
    let limit_val = query.limit.unwrap_or(100).min(1000);
    let offset_val = query.offset.unwrap_or(0);

    // Build query with optional Binance filter
    let mut q = transfers.into_boxed();
    if query.binance_only.unwrap_or(false) {
        q = q.filter(is_binance_related.eq(true));
    }

    // Execute
    let rows: Vec<Transfer> = match q
        .order(block_number.desc())
        .limit(limit_val)
        .offset(offset_val)
        .load(&mut conn)
    {
        Ok(v) => v,
        Err(e) => return Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    };

    Json(rows).into_response()
}
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": current_utc_timestamp()
    }))
}

// WebSocket upgrade handler for GET /ws
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

// Per-connection task: forward broadcast messages to this socket and
// drain client messages (optional) until close.
async fn websocket_connection(mut socket: WebSocket, state: ServerState) {
    // Subscribe to the broadcast channel
    let mut rx = state.broadcast.subscribe();

    // Send an initial message
    if let Ok(init) = serde_json::to_string(&serde_json::json!({
        "type": "hello",
        "data": "Connected to Polygon Indexer"
    })) {
        let _ = socket.send(Message::Text(init)).await;
    }

    // Split a copy of the socket sender for outgoing messages
    let (mut sender, mut receiver) = socket.split();

    // Task to forward server broadcasts to this client
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break; // client disconnected
            }
        }
    });

    // Task to read client messages and stop on close/error
    let recv_task = tokio::spawn(async move {
        use futures_util::StreamExt;
        while let Some(incoming) = receiver.next().await {
            match incoming {
                Ok(Message::Close(_)) => break,
                Ok(_other) => {
                    // Optionally handle ping/pong/text/binary from client here
                }
                Err(_) => break,
            }
        }
    });

    // End when either task finishes
    let _ = tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    };
}

// Note: health_check, get_transfers, get_net_flow, get_stats,
// websocket_handler, websocket_broadcast_task must either be defined below
// in this file or imported via the HANDLERS IMPORT section above.
