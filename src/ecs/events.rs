use bevy_ecs::{event::Event};
use tokio_tungstenite::tungstenite::Message;
use std::net::SocketAddr;

#[derive(Event, bevy_ecs::message::Message)]
pub struct MessageReceived {
    pub client_id: SocketAddr,
    pub msg: Message,
}