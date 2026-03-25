use bevy_ecs::prelude::*;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use std::net::SocketAddr;

/// クライアントを一意に識別するためのIDコンポーネント
/// 接続時にインクリメントされるカウンターやUUID
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub u64);

// エンティティに持たせる送信用のポスト
#[derive(Component)]
pub struct ClientSender(pub mpsc::Sender<Message>);

// エンティティに持たせるクライアントID
#[derive(Component)]
pub struct ClientId(pub SocketAddr);

// クライアントが所属しているルームを表す
#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Room(pub String);

