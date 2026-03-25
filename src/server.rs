use tokio::net::TcpListener;
use tokio::sync::mpsc;
use crate::network::{connection, channels::NetworkEvent};
use std::sync::atomic::{AtomicU64, Ordering};

// IDを生成するためのスレッドセーフなカウンター
static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

pub async fn run(addr: &str, ecs_tx: mpsc::Sender<NetworkEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on ws://{addr}");

    while let Ok((stream, addr)) = listener.accept().await {
        // 新しい接続が来るたびにIDを1進めて取得
        let conn_id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);

        println!("New connection from: {addr}");

        // txをクローンして各コネクションタスクへ渡す
        tokio::spawn(connection::handle_connection(stream, conn_id, ecs_tx.clone()));
    }
    Ok(())
}