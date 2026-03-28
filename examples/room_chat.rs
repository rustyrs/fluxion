// v0.0.4

use fluxion::{plugin::ChatPlugin, prelude::*};

fn handle_disconnections_system(
    mut commands: Commands,
    mut ev_disconnected: MessageReader<UserDisconnected>,
    mut ev_send: MessageWriter<SendMessage>,
    client_query: Query<&Room>,
    mut room_map: ResMut<RoomMap>,
) {
    for disconnect in ev_disconnected.read() {
        let entity = disconnect.entity;

        if let Ok(room) = client_query.get(entity) {
            // RoomMapから対象エンティティを削除
            if let Some(members) = room_map.0.get_mut(&room.0) {
                members.remove(&entity);

                // 同じルームの「残りのメンバー」に退室メッセージを送信
                let msg = format!("[System] User {} has left.", disconnect.client_id);
                for &target_entity in members.iter() {
                    ev_send.write(SendMessage {
                        target: target_entity,
                        payload: NetworkPayload::Text(msg.clone()),
                    });
                }
            }
        }

        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn();
        }
    }
}

fn parse_chat_messages_system(
    mut ev_received: MessageReader<MessageReceived>,
    mut ev_command: MessageWriter<ChatCommand>,
) {
    for msg in ev_received.read() {
        let NetworkPayload::Text(text) = &msg.payload else {
            continue;
        };
        let text = text.trim();

        if let Some(room_name) = text.strip_prefix("/join ") {
            ev_command.write(ChatCommand::JoinRoom {
                entity: msg.entity,
                room_name: room_name.trim().to_string(),
            });
        } else if let Some(name) = text.strip_prefix("/nick ") {
            // /nick コマンドの解析
            ev_command.write(ChatCommand::Nick {
                entity: msg.entity,
                name: name.trim().to_string(),
            });
        } else if text == "/list" {
            ev_command.write(ChatCommand::ListRooms { entity: msg.entity });
        } else if text.starts_with('/') {
            // 未知のコマンドに対するエラーハンドリング
            ev_command.write(ChatCommand::Error {
                entity: msg.entity,
                message: format!(
                    "Unknown command: {}",
                    text.split_whitespace().next().unwrap_or(text)
                ),
            });
        } else {
            // 通常のチャット
            ev_command.write(ChatCommand::Broadcast {
                entity: msg.entity,
                text: text.to_string(),
            });
        }
    }
}

fn handle_join_room_system(
    mut commands: Commands,
    mut ev_command: MessageReader<ChatCommand>,
    mut ev_send: MessageWriter<SendMessage>,
    client_query: Query<Option<&Room>>,
    mut room_map: ResMut<RoomMap>,
) {
    for command in ev_command.read() {
        if let ChatCommand::JoinRoom { entity, room_name } = command {
            let Ok(current_room) = client_query.get(*entity) else {
                continue;
            };

            // 古いルームからの離脱処理
            if let Some(old_room) = current_room
                && let Some(members) = room_map.0.get_mut(&old_room.0)
            {
                members.remove(entity);
            }

            // 新しいルームへの参加処理
            room_map
                .0
                .entry(room_name.clone())
                .or_default()
                .insert(*entity);
            commands.entity(*entity).insert(Room(room_name.clone()));

            ev_send.write(SendMessage {
                target: *entity,
                payload: NetworkPayload::Text(format!("[System] Joined room: {}", room_name)),
            });
        }
    }
}

fn handle_broadcast_system(
    mut ev_command: MessageReader<ChatCommand>,
    mut ev_send: MessageWriter<SendMessage>,
    // Queryに Option<&Username> を追加
    client_query: Query<(&ClientId, Option<&Username>, Option<&Room>)>,
    room_map: Res<RoomMap>,
) {
    for command in ev_command.read() {
        if let ChatCommand::Broadcast { entity, text } = command {
            let Ok((client_id, username, current_room)) = client_query.get(*entity) else {
                continue;
            };

            if let Some(room) = current_room {
                // 名前が設定されていればそれを、なければ "User [ID]" を表示名にする
                let display_name = match username {
                    Some(u) => u.0.clone(),
                    None => format!("User {}", client_id.0),
                };

                let broadcast_text = format!("{}: {}", display_name, text);

                if let Some(members) = room_map.0.get(&room.0) {
                    for &target_entity in members {
                        ev_send.write(SendMessage {
                            target: target_entity,
                            payload: NetworkPayload::Text(broadcast_text.clone()),
                        });
                    }
                }
            } else {
                ev_send.write(SendMessage {
                    target: *entity,
                    payload: NetworkPayload::Text(
                        "[System] You are not in a room. Type '/join <room_name>' first.".into(),
                    ),
                });
            }
        }
    }
}

fn handle_nick_system(
    mut commands: Commands,
    mut ev_command: MessageReader<ChatCommand>,
    mut ev_send: MessageWriter<SendMessage>,
) {
    for command in ev_command.read() {
        if let ChatCommand::Nick { entity, name } = command {
            // Usernameコンポーネントをエンティティに付与（すでにあれば上書き）
            commands.entity(*entity).insert(Username(name.clone()));

            // 本人に成功メッセージを返す
            ev_send.write(SendMessage {
                target: *entity,
                payload: NetworkPayload::Text(format!("[System] Your nickname is now: {}", name)),
            });
        }
    }
}

fn handle_error_system(
    mut ev_command: MessageReader<ChatCommand>,
    mut ev_send: MessageWriter<SendMessage>,
) {
    for command in ev_command.read() {
        if let ChatCommand::Error { entity, message } = command {
            // エラー原因を本人にだけシステムメッセージとして返す
            ev_send.write(SendMessage {
                target: *entity,
                payload: NetworkPayload::Text(format!("[Error] {}", message)),
            });
        }
    }
}

fn handle_list_rooms_system(
    mut ev_command: MessageReader<ChatCommand>,
    mut ev_send: MessageWriter<SendMessage>,
    room_map: Res<RoomMap>,
) {
    for command in ev_command.read() {
        if let ChatCommand::ListRooms { entity } = command {
            let mut list_text = String::from("[System] Active Rooms:\n");

            let mut has_active_rooms = false;

            // RoomMap の中身をループして一覧を作成
            for (room_name, members) in room_map.0.iter() {
                // 誰もいないルームは非表示にする
                if !members.is_empty() {
                    list_text.push_str(&format!("  - {} ({} users)\n", room_name, members.len()));
                    has_active_rooms = true;
                }
            }

            if !has_active_rooms {
                list_text.push_str("  No active rooms right now.");
            }

            // 要求した本人に結果を送信
            ev_send.write(SendMessage {
                target: *entity,
                payload: NetworkPayload::Text(list_text),
            });
        }
    }
}

fn main() {
    FluxionApp::new()
        .add_plugins(ChatPlugin)
        .add_plugins(FluxionWebSocketPlugin::new("127.0.0.1:8080"))
        .add_systems(
            MainSchedule,
            (
                parse_chat_messages_system,
                handle_join_room_system,
                handle_nick_system,
                handle_list_rooms_system,
                handle_error_system,
                handle_broadcast_system,
                handle_disconnections_system,
            ),
        )
        .run()
}
