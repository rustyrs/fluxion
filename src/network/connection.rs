use crate::{
    ecs,
    network::channels::{ClientMessage, NetworkEvent},
};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::{Message, client};

pub async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    ecs_tx: mpsc::Sender<NetworkEvent>,
) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Error during websocket handshake for {addr}: {e}");
            return;
        }
    };

    println!("WebSocket connection established for {addr}");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // このクライアント専用ポスト
    let (client_tx, mut client_rx) = mpsc::channel::<Message>(100);
    if ecs_tx
        .send(NetworkEvent::Connected {
            id: addr,
            sender: client_tx,
        })
        .await
        .is_err()
    {
        return; // ECSが死んでたら終了
    }

    // writeタスク
    let write_task = tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // readタスク
    let ecs_tx_clone = ecs_tx.clone();
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if msg.is_close() {
                break;
            }
            let _ = ecs_tx_clone
                .send(NetworkEvent::Message { id: addr, msg })
                .await;
        }
        let _ = ecs_tx_clone
            .send(NetworkEvent::Disconnected { id: addr })
            .await;
    });

    let _ = tokio::join!(read_task, write_task);
    println!("Connection closed");
}
