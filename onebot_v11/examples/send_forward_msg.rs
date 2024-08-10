use onebot_v11::{
    api::payload::{ApiPayload, SendGroupForwardMsg},
    connect::http::{HttpConfig, HttpConnect},
    MessageSegment,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = HttpConfig {
        port: 3000,
        ..Default::default()
    };
    let http_conn = HttpConnect::new(config);
    let payload = ApiPayload::SendGroupForwardMsg(SendGroupForwardMsg {
        group_id: 418012707,
        messages: vec![
            MessageSegment::custom_node(
                123546789,
                "canxin121",
                vec![MessageSegment::text("hello")],
            ),
            MessageSegment::custom_node(
                123465879,
                "canxin121",
                vec![MessageSegment::text("world")],
            ),
        ],
    });
    let resp = http_conn.call_api(payload).await.unwrap();
    println!("resp: {:#?}", resp);
}
