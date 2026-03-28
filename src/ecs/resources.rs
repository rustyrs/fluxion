use bevy_ecs::prelude::*;
use std::collections::{HashMap, HashSet};

// Entityを一発で引くための内部リソース
#[derive(Resource, Default)]
pub struct ConnectionMap(pub HashMap<u64, Entity>);

// サーバーのTickレート
#[derive(Resource)]
pub struct ServerTickRate(pub f64);
impl Default for ServerTickRate {
    fn default() -> Self {
        Self::NORMAL
    }
}
impl ServerTickRate {
    pub const ECO: Self = Self(10.0);
    pub const NORMAL: Self = Self(30.0);
    pub const HIGH: Self = Self(60.0);
    pub const VERYHIGH: Self = Self(90.0);
    pub const REALTIME: Self = Self(120.0);
}

// ルーム名からそこに所属するエンティティ一覧をO(1)で引くため
#[derive(Resource, Default)]
pub struct RoomMap(pub HashMap<String, HashSet<Entity>>);