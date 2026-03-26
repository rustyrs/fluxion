use bevy_ecs::{entity::Entity, event::Event};
use tokio_tungstenite::tungstenite::Message as WsMessage;
use bevy_ecs::message::Message;

#[derive(Event, Message)]
pub struct MessageReceived {
    pub entity: Entity,
    pub client_id: u64,
    pub msg: WsMessage,
}

// ユーザーが送信要素を出すためのイベント
#[derive(Message)]
pub struct SendWsMessage {
    pub target: Entity,
    pub msg: WsMessage,
}

// 上の全体に一斉送信版
#[derive(Message)]
pub struct BroadcastWsMessage {
    pub msg: WsMessage,
}