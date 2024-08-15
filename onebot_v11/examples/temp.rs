use onebot_v11::{
    api::payload::SendPrivateMsg,
    connect::ws_reverse::{ReverseWsConfig, ReverseWsConnect},
    event::message::{Message, PrivateMessage},
    Event,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let ws_reverse_config = ReverseWsConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        suffix: "onebot/v11".to_string(),
    };

    let ws_conn = ReverseWsConnect::new(ws_reverse_config).await.unwrap();
    let mut subscriber = ws_conn.subscribe().await;

    while let Ok(event) = subscriber.recv().await {
        if let Event::Message(Message::PrivateMessage(PrivateMessage {
            message_id,
            user_id,
            message,
            ..
        })) = event
        {
            ws_conn
                .clone()
                .call_api(onebot_v11::api::payload::ApiPayload::SendPrivateMsg(
                    SendPrivateMsg {
                        user_id,
                        message,
                        auto_escape: true,
                    },
                ))
                .await
                .unwrap();
        }
    }
}
