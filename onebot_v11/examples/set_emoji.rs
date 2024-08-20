// TARGET=123456789 cargo run --example --release set_emoji
// 可选环境变量 HOST, PORT, SUFFIX(反向ws后缀，默认onebot/v11), TARGET(设置表情的目标qq)
use onebot_v11::{
    api::payload::{ApiPayload, SetMsgEmojiLike},
    connect::ws_reverse::{ReverseWsConfig, ReverseWsConnect},
    event::message::{GroupMessage, Message, PrivateMessage},
    Event,
};
use std::env;

macro_rules! set_msg_emoji_likes {
    ($ws_conn:expr, $message_id:expr, [$($emoji_id:expr),*]) => {
        let ws_conn = $ws_conn.clone();
        tokio::spawn(async move {
            $(
                let _ = ws_conn
                    .clone()
                    .call_api(ApiPayload::SetMsgEmojiLike(SetMsgEmojiLike {
                        message_id: $message_id.to_string(),
                        emoji_id: $emoji_id.to_string(),
                    }))
                    .await;
            )*
        });
    };
}

fn get_env_var_case_insensitive(key: &str) -> Option<String> {
    for (k, v) in env::vars() {
        if k.eq_ignore_ascii_case(key) {
            return Some(v);
        }
    }
    None
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut ws_reverse_config = ReverseWsConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        suffix: "onebot/v11".to_string(),
        ..Default::default()
    };
    // 从环境变量读取配置并覆盖
    if let Some(ws_host) = get_env_var_case_insensitive("HOST") {
        ws_reverse_config.host = ws_host;
    }
    if let Some(ws_port) = get_env_var_case_insensitive("PORT") {
        if let Ok(port) = ws_port.parse::<u16>() {
            ws_reverse_config.port = port;
        }
    }
    if let Some(ws_suffix) = get_env_var_case_insensitive("SUFFIX") {
        ws_reverse_config.suffix = ws_suffix;
    }
    let target = match get_env_var_case_insensitive("TARGET") {
        Some(target) => target.parse::<i64>().unwrap(),
        None => panic!("TARGET not set"),
    };

    let ws_conn = ReverseWsConnect::new(ws_reverse_config).await.unwrap();
    let mut subscriber = ws_conn.subscribe().await;

    while let Ok(event) = subscriber.recv().await {
        if let Event::Message(
            Message::PrivateMessage(PrivateMessage {
                message_id,
                user_id,
                ..
            })
            | Message::GroupMessage(GroupMessage {
                message_id,
                user_id,
                ..
            }),
        ) = event
        {
            if user_id == target {
                set_msg_emoji_likes!(
                    ws_conn,
                    message_id,
                    ["128514", "128531", "128536", "128147"]
                );
            }
        }
    }
}
