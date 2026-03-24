use tokio::net::TcpListener;
use tokio::sync::mpsc;
use crate::network::{connection, channels::NetworkEvent};

pub async fn run(addr: &str, ecs_tx: mpsc::Sender<NetworkEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on ws://{addr}");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {addr}");
        // txをクローンして各コネクションタスクへ渡す
        tokio::spawn(connection::handle_connection(stream, addr, ecs_tx.clone()));
    }
    Ok(())
}