use std::net::SocketAddr;
use tokio::net::TcpStream;

#[tokio::test]
/// Tests if the server starts and listens on the specified address.
///
/// This test binds a `TcpListener` to the address `127.0.0.1:8081` and asserts
/// that the listener's local address matches the expected address.
async fn server_starts_and_listens_on_address() {
    let addr = "127.0.0.1:8081";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    assert_eq!(
        listener.local_addr().unwrap(),
        addr.parse::<SocketAddr>().unwrap()
    );
}

#[tokio::test]
/// Tests if the server accepts a connection.
///
/// This test binds a `TcpListener` to the address `127.0.0.1:8082`, spawns a task
/// to connect to the listener, and asserts that the listener accepts the connection
/// and the stream's peer address is valid.
async fn server_accepts_connection() {
    let addr = "127.0.0.1:8082";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tokio::spawn(async move {
        let _ = TcpStream::connect(addr).await.unwrap();
    });
    let (stream, _) = listener.accept().await.unwrap();
    assert!(stream.peer_addr().is_ok());
}

#[tokio::test]
/// Tests if the server handles multiple connections.
///
/// This test binds a `TcpListener` to the address `127.0.0.1:8083`, spawns multiple
/// tasks to connect to the listener, and asserts that the listener accepts each
/// connection and the stream's peer address is valid.
async fn server_handles_multiple_connections() {
    let addr = "127.0.0.1:8083";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let mut handles = vec![];

    for _ in 0..5 {
        let addr = addr.to_string();
        handles.push(tokio::spawn(async move {
            let _ = TcpStream::connect(&addr).await.unwrap();
        }));
    }

    for _ in 0..5 {
        let (stream, _) = listener.accept().await.unwrap();
        assert!(stream.peer_addr().is_ok());
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
