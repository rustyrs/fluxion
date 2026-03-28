// v0.0.3

use fluxion::prelude::*;

fn handle_disconnections_system(
    mut commands: Commands,
    mut ev_disconnected: MessageReader<UserDisconnected>,
    mut ev_send: MessageWriter<SendMessage>,
    client_query: Query<&Room>,
    room_query: Query<(Entity, &Room)>,
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

fn chat_server_system(
    mut commands: Commands,
    mut ev_received: MessageReader<MessageReceived>,
    mut ev_send: MessageWriter<SendMessage>,
    // メッセージ送信元のクライアント情報を取得するクエリ
    client_query: Query<(Entity, &ClientId, Option<&Room>)>,
    // ルームに所属している全クライアントを取得するクエリ
    room_query: Query<(Entity, &Room)>,
    mut room_map: ResMut<RoomMap>,
) {
    // ECS上で発火したすべての受信メッセージを処理
    for msg in ev_received.read() {
        // テキストメッセージのみを処理対象とする
        let NetworkPayload::Text(text) = &msg.payload else {
            continue;
        };
        let sender_entity = msg.entity;
        let text = text.trim();

        // 1. ルーム参加コマンドの処理
        if let Some(room_name) = text.strip_prefix("/join") {
            let room_name = room_name.trim().to_string();

            let Ok((_, _, current_room)) = client_query.get(sender_entity) else {
                continue;
            };

            // 以前のルームにいた場合はRoomMapから削除
            if let Some(old_room) = current_room
                && let Some(members) = room_map.0.get_mut(&old_room.0)
            {
                members.remove(&sender_entity);
            }

            // 新しいルームにRoomMap上で追加
            room_map.0.entry(room_name.clone()).or_default().insert(sender_entity);

            // エンティティにRoomコンポーネントを追加(既にあれば上書きされる)
            commands.entity(sender_entity).insert(Room(room_name.clone()));

            // 本人にシステムメッセージを返信
            ev_send.write(SendMessage {
                target: sender_entity,
                payload: NetworkPayload::Text(format!("[System] Joined room: {}", room_name)),
            });
            continue;
        }

        // 2. 通常のチャットメッセージの処理
        // 送信元の現在の所属ルームを取得
        let Ok((_, client_id, current_room)) = client_query.get(sender_entity) else { continue };

        if let Some(room) = current_room {
            let broadcast_text = format!("User {}: {}", client_id.0, text);

            if let Some(members) = room_map.0.get(&room.0) {
                for &target_entity in members {
                    ev_send.write(SendMessage {
                        target: target_entity,
                        payload: NetworkPayload::Text(broadcast_text.clone()),
                    });
                }
            }
        } else {
            // ルームに所属していない場合はエラーメッセージを返す
            ev_send.write(SendMessage {
                target: sender_entity,
                payload: NetworkPayload::Text(
                    "[System] You are not in a room. Type '/join <room_name>' first.".into(),
                ),
            });
        }
    }
}

fn main() {
    FluxionApp::new()
        .add_plugins(FluxionWebSocketPlugin::new("127.0.0.1:8080"))
        .add_systems(
            MainSchedule,
            (chat_server_system, handle_disconnections_system),
        )
        .run()
}
