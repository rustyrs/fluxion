use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use std::net::SocketAddr;

// ECSに送るデータ型
pub struct ClientMessage {
    pub client_id: SocketAddr,
    pub msg: Message,
}

// ネットワーク層からECSへ送るイベント
pub enum NetworkEvent {
    Connected {
        id: u64,
        sender: mpsc::Sender<Message>,
    },
    Message {
        id: u64,
        msg: Message,
    },
    Disconnected {
        id: u64,
    }
}