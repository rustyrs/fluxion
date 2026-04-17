use bevy_ecs::message::MessageReader;
use bevy_ecs::prelude::{Commands, Entity, MessageWriter, Query, Res};
use crate::plugins::rate_limit::{RateLimitAction, RateLimitConfig, RateLimitExceededEvent, RateLimitState};
use crate::prelude::{ClientId, MessageReceived, SendMessage, UserDisconnected};

pub fn check_rate_limit_system(
    mut commands: Commands,
    config: Res<RateLimitConfig>,
    mut query: Query<(Entity, &mut RateLimitState, &ClientId)>,
    mut ev_received: MessageReader<MessageReceived>,
    mut ev_exceeded: MessageWriter<RateLimitExceededEvent>,
    mut ev_send: MessageWriter<SendMessage>,
    mut ev_disconnect: MessageWriter<UserDisconnected>,
) {
    for msg in ev_received.read() {
        let Ok((entity, mut state, client_id)) = query.get_mut(msg.entity) else {
            continue;
        };
        let now = std::time::Instant::now();

        if let Some(until) = state.throttled_until {
            if now < until {
                ev_exceeded.write(RateLimitExceededEvent {
                    entity,
                    client_id: client_id.0,
                });
                continue;
            }
            state.throttled_until = None;
        }

        let elapsed = now.duration_since(state.last_refill).as_secs_f32();
        state.tokens = (state.tokens + config.refill_rate * elapsed).min(config.capacity);
        state.last_refill = now;

        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            continue;
        }

        ev_exceeded.write(RateLimitExceededEvent {
            entity,
            client_id: client_id.0,
        });

        match &config.action {
            RateLimitAction::Drop => {}
            RateLimitAction::Throttle => {}
            RateLimitAction::Disconnect => {}
        }
    }
}
