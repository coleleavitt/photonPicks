use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(String),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Token validation error: {0}")]
    ValidationError(String),

    #[error("WebSocket protocol error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
}

pub type Result<T> = std::result::Result<T, WebSocketError>;
