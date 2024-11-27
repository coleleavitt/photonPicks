use std::sync::atomic::{AtomicUsize, Ordering};
use std::net::SocketAddr;
use metrics::{counter, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;

pub struct Metrics {
    pub active_connections: AtomicUsize,
    pub messages_processed: AtomicUsize,
}

impl Metrics {
    pub fn new() -> Self {

        Self {
            active_connections: AtomicUsize::new(0),
            messages_processed: AtomicUsize::new(0),
        }
    }

    pub fn record_connection(&self) {
        let count = self.active_connections.fetch_add(1, Ordering::SeqCst);
        let g = gauge!("websocket_connections");
        g.set(count as f64);
    }

    pub fn record_message(&self) {
        let count = self.messages_processed.fetch_add(1, Ordering::SeqCst);
        let c = counter!("websocket_messages_processed");
        c.increment(count as u64);  // Changed from f64 to u64
    }
}