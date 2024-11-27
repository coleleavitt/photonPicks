use crate::models::websocket::ConnectionManager;
use crate::models::token::TokenProcessor;
use tokio::net::TcpStream;
use crate::errors::Result;

pub async fn handle_connection(
    stream: TcpStream,
    conn_manager: &ConnectionManager,
    token_processor: &TokenProcessor,
) -> Result<()> {
    tracing::info!("New connection established");
    conn_manager.handle_connection(stream, token_processor).await
}
