use bevy_ecs::prelude::{Entity, Message};
use ecson_core::plugin::Plugin;
use ecson_core::prelude::{EcsonApp, Update};
use crate::prelude::{receive_network_messages_system, Component, Resource};

pub mod systems;

/// 超過時の動作
#[derive(Clone, Debug)]
enum RateLimitAction {
    /// 無視
    Drop,
    /// 一時停止
    Throttle,
    /// 切断
    Disconnect,
}

/// Token Limitingの設定
#[derive(Resource)]
pub struct RateLimitConfig {
    /// 最大容量
    pub capacity: f32,
    /// 1秒あたりのトークン補充量
    pub refill_rate: f32,
    /// 超過時の動作
    pub action: RateLimitAction,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            capacity: 10.0,
            refill_rate: 5.0,
            action: RateLimitAction::Drop,
        }
    }
}

/// レート制限の状態
#[derive(Component)]
pub struct RateLimitState {
    /// 現在のトークン残量
    pub tokens: f32,
    /// 最後に補充を行った時刻
    pub last_refill: std::time::Instant,
    /// Throttle中の場合、解除時刻
    pub throttled_until: Option<std::time::Instant>,
}

impl RateLimitState {
    pub fn new(capacity: f32) -> Self {
        Self {
            tokens: capacity, // 初期値は満タン
            last_refill: std::time::Instant::now(),
            throttled_until: None,
        }
    }
}

/// レート制限を超過したときに発火
#[derive(Message)]
pub struct RateLimitExceededEvent {
    pub entity: Entity,
    pub client_id: u64,
}

/// プラグイン
pub struct RateLimitPlugin {
    config: RateLimitConfig,
}

impl Default for RateLimitPlugin {
    fn default() -> Self {
        Self { config: RateLimitConfig::default() }
    }
}

impl RateLimitPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn capacity(mut self, capacity: f32) -> Self {
        self.config.capacity = capacity;
        self
    }

    pub fn refill_rate(mut self, refill_rate: f32) -> Self {
        self.config.refill_rate = refill_rate;
        self
    }

    pub fn on_exceed(mut self, action: RateLimitAction) -> Self {
        self.config.action = action;
        self
    }
}

impl Plugin for RateLimitPlugin {
    fn build(&mut self, app: &mut EcsonApp) {
        app.insert_resource(RateLimitConfig {
            capacity: self.config.capacity,
            refill_rate: self.config.refill_rate,
            action: self.config.action.clone(),
        });
        app.add_event::<RateLimitExceededEvent>();
        app.add_systems(
            Update,
            (setup_rate_limit_system, check_rate_limit_system)
                .chain()
                .after(receive_network_messages_system)
        );
    }
}
