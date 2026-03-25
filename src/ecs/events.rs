use bevy_ecs::{entity::Entity, event::Event};
use tokio_tungstenite::tungstenite::Message;

#[derive(Event, bevy_ecs::message::Message)]
pub struct MessageReceived {
    pub entity: Entity,
    pub client_id: u64,
    pub msg: Message,
}