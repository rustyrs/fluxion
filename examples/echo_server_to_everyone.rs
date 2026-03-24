use fluxion::prelude::*;
use fluxion::server;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message as WsMessage;

// ブロードキャストシステム
fn broadcast_system(
    mut messages: MessageReader<MessageReceived>,
    query: Query<(&ClientId, &ClientSender)>,
) {
    for event in messages.read() {
        // テキストメッセージだけを抽出（バイナリやPing等は一旦無視）
        let text_content = match &event.msg {
            WsMessage::Text(text) => text.to_string(),
            _ => continue, 
        };

        // 「誰からの発言か」分かるようにIPとポートをプレフィックスにつける
        let broadcast_text = format!("[{}]: {}", event.client_id, text_content);
        let broadcast_msg = WsMessage::Text(broadcast_text.into());

        // クエリで取得した「全クライアント」に対してメッセージを送信
        for (target_client_id, sender) in query.iter() {
            if let Err(e) = sender.0.try_send(broadcast_msg.clone()) {
                // 送信失敗時（すでに切断処理中など）のハンドリング
                // println!("{} への送信に失敗: {}", target_client_id.0, e);
            }
        }
    }
}

fn main() {
    let (ecs_tx, ecs_rx) = mpsc::channel::<NetworkEvent>(1024);

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = server::run("127.0.0.1:8080", ecs_tx).await {
                eprintln!("Server error: {}", e);
            }
        });
    });

    let mut app = FluxionApp::new();
    
    app.world.insert_resource(NetworkReceiver(ecs_rx));
    app.world.insert_resource(Messages::<MessageReceived>::default());

    app.add_systems(
        MainSchedule,
        (
            |mut msgs: ResMut<Messages<MessageReceived>>| msgs.update(),
            receive_network_messages_system,
            broadcast_system,
        )
    );

    app.run();
}