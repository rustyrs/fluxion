use bevy_ecs::prelude::*;
use std::collections::HashMap;

// Entityを一発で引くための内部リソース
#[derive(Resource, Default)]
pub struct ConnectionMap(pub HashMap<u64, Entity>);