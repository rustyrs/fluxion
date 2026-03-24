use fluxion::prelude::*;
use fluxion::server;
use tokio::sync::mpsc;

fn echo_system(
    mut messages: MessageReader<MessageReceived>,
    query: Query<(&ClientId, &ClientSender)>,
) {
    for event in messages.read() {
        for (client_id, sender) in query.iter() {
            if client_id.0 == event.client_id {
                let _ = sender.0.try_send(event.msg.clone());
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
            echo_system,
        )
    );

    app.run();
}