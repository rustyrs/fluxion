use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::net::SocketAddr;

// SocketAddrからEntityを一発で引くための内部リソース
#[derive(Resource, Default)]
pub struct ConnectionMap(pub HashMap<SocketAddr, Entity>);