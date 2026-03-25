use crate::network::channels::NetworkEvent;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_connection(
    stream: TcpStream,
    conn_id: u64,
    ecs_tx: mpsc::Sender<NetworkEvent>,
) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Error during websocket handshake for ID {conn_id}: {e}");
            return;
        }
    };

    println!("WebSocket connection established for ID {conn_id}");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let (client_tx, mut client_rx) = mpsc::channel::<Message>(100);
    
    // Connected に conn_id を乗せて送る
    if ecs_tx
        .send(NetworkEvent::Connected {
            id: conn_id,
            sender: client_tx,
        })
        .await
        .is_err()
    {
        return; 
    }

    let write_task = tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let ecs_tx_clone = ecs_tx.clone();
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if msg.is_close() {
                break;
            }
            let _ = ecs_tx_clone
                .send(NetworkEvent::Message { id: conn_id, msg })
                .await;
        }
        let _ = ecs_tx_clone
            .send(NetworkEvent::Disconnected { id: conn_id })
            .await;
    });

    let _ = tokio::join!(read_task, write_task);
    println!("Connection closed for ID {conn_id}");
}