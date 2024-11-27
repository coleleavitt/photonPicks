mod errors;
mod handlers;
mod models;
mod config;
mod metrics;

use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> errors::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let settings = config::Settings::new().map_err(|e| errors::WebSocketError::ConnectionError(e.to_string()))?;
    let metrics = metrics::Metrics::new();
    let conn_manager = models::websocket::ConnectionManager::new();
    let token_processor = models::token::TokenProcessor::new();

    let addr = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("WebSocket server listening on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        metrics.record_connection();
        let cm = conn_manager.clone();
        let tp = token_processor.clone();

        tokio::spawn(async move {
            if let Err(e) = handlers::handle_connection(stream, &cm, &tp).await {
                tracing::error!("Connection error: {}", e);
            }
        });
    }

    Ok(())
}