#[derive(Debug, thiserror::Error)]
pub enum WebSocketError {
    #[error("Failed to parse token data: {0}")]
    TokenParse(String),
    #[error("WebSocket error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, WebSocketError>;