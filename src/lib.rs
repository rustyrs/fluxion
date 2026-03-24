// `bevy_ecs` クレートを探しに行けるようにするための魔法です。
pub use bevy_ecs;

pub mod app;
pub mod plugin;
pub mod server;
pub mod ecs;
pub mod network;

pub mod prelude {
    pub use crate::bevy_ecs::prelude::*;
    
    pub use crate::bevy_ecs::event::Event;
    pub use crate::bevy_ecs::message::{Messages, MessageReader, MessageWriter};

    pub use crate::app::{FluxionApp, MainSchedule};
    pub use crate::network::channels::NetworkEvent;
    pub use crate::ecs::components::*;
    pub use crate::ecs::systems::*;
    pub use crate::ecs::events::MessageReceived;
}