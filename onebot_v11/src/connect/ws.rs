use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use tracing::{info, warn};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt as _, StreamExt as _};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::api::payload::ApiPayload;
use crate::api::resp::{ApiResp, ApiRespBuilder};
use crate::traits::EndPoint as _;
use crate::Event;
use std::time::Duration;
use tokio::time::{sleep, timeout};

use super::WsType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WsConfig {
    pub host: String,
    pub port: u16,
    pub r#type: WsType,
    pub bot_id: Option<String>,
    pub bot_nick_name: Option<String>,
    pub access_token: Option<String>,
}

impl Default for WsConfig {
    fn default() -> Self {
        WsConfig {
            host: "127.0.0.1".to_string(),
            r#type: WsType::Universal,
            port: 8081,
            access_token: None,
            bot_id: None,
            bot_nick_name: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WsApiPayload {
    pub action: String,
    pub params: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,
}
impl Into<String> for WsApiPayload {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<WsApiPayload> for ApiPayload {
    fn into(self) -> WsApiPayload {
        WsApiPayload {
            action: self.endpoint(),
            params: serde_json::to_value(self).unwrap(),
            echo: Some("123".to_string()),
        }
    }
}

pub struct WsConnect {
    pub config: WsConfig,
    ws_read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    event_sender: Arc<Mutex<broadcast::Sender<Event>>>,
    api_response_sender: Arc<Mutex<mpsc::Sender<ApiRespBuilder>>>,
    api_response_receiver: Arc<Mutex<mpsc::Receiver<ApiRespBuilder>>>,
}

impl WsConnect {
    pub async fn new(wsconfig: WsConfig) -> Result<Arc<Self>, anyhow::Error> {
        let (ws_write, ws_read) = Self::connect(&wsconfig).await;
        let (api_response_sender, api_response_receiver) = mpsc::channel(100);
        let self_ = Arc::new(Self {
            config: wsconfig.clone(),
            ws_read: Arc::new(Mutex::new(ws_read)),
            ws_write: Arc::new(Mutex::new(ws_write)),
            event_sender: Arc::new(Mutex::new(broadcast::channel(100).0)),
            api_response_sender: Arc::new(Mutex::new(api_response_sender)),
            api_response_receiver: Arc::new(Mutex::new(api_response_receiver)),
        });

        self_.clone().start_event_listener();
        Ok(self_)
    }

    async fn connect(
        config: &WsConfig,
    ) -> (
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let url = format!(
            "ws://{}:{}{}",
            config.host,
            config.port,
            match config.r#type {
                WsType::Event => "/event",
                WsType::Api => "/api",
                WsType::Universal => "",
            }
        );
        loop {
            let url = url.clone();
            match connect_async(url).await {
                Ok((ws_stream, _)) => {
                    let (write, read) = ws_stream.split();
                    info!("Connected to WebSocket server");
                    break (write, read);
                }
                Err(e) => {
                    warn!(
                        "Error connecting to WebSocket server: {}, will retry in 3 seconds",
                        e
                    );
                }
            }
            sleep(Duration::from_secs(3)).await;
        }
    }

    fn start_event_listener(self: Arc<Self>) {
        let read = Arc::clone(&self.ws_read);
        let event_sender = Arc::clone(&self.event_sender);
        let api_response_sender = Arc::clone(&self.api_response_sender);
        let self_clone = Arc::clone(&self);

        tokio::spawn(async move {
            {
                let mut read = read.lock().await;
                let sender = event_sender.lock().await;

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(msg) => {
                            let msg_string = msg.to_string();
                            match serde_json::from_str::<Event>(&msg_string) {
                                Ok(event) => match event {
                                    Event::ApiRespBuilder(api_resp_builder) => {
                                        if let Err(e) = api_response_sender
                                            .lock()
                                            .await
                                            .send(api_resp_builder)
                                            .await
                                        {
                                            warn!("Error sending ApiRespBuilder: {}", e);
                                        }
                                    }
                                    other => {
                                        if let Err(e) = sender.send(other) {
                                            warn!("Error sending Event: {}", e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    warn!("Error parsing Event: {}, Raw: {}", e, msg_string);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Error receiving WsMessage: {}", e);
                        }
                    }
                }
                warn!("WsMessage stream ended, attempting to reconnect");
            }
            let (ws_write, ws_read) = Self::connect(&self_clone.config).await;
            *self_clone.ws_write.lock().await = ws_write;
            *self_clone.ws_read.lock().await = ws_read;
            self_clone.start_event_listener();
            info!("Reconnected to WebSocket server",)
        });
    }

    pub async fn subscribe(&self) -> broadcast::Receiver<Event> {
        let event_sender = self.event_sender.clone();
        let sender = event_sender.lock().await;
        sender.subscribe()
    }

    pub async fn call_api(self: Arc<Self>, api_data: ApiPayload) -> Result<ApiResp, anyhow::Error> {
        let resp_type = api_data.to_resp_type();

        {
            let ws_api_data: WsApiPayload = api_data.into();
            let ws_api_string: String = ws_api_data.into();
            let mut write = self.ws_write.lock().await;
            write.send(Message::Text(ws_api_string)).await?;
        }

        let resp_builder = timeout(
            Duration::from_secs(30),
            self.api_response_receiver.lock().await.recv(),
        )
        .await
        .ok()
        .flatten()
        .ok_or(anyhow::anyhow!(
            "[WsConnect.call_api] Error receiving API response, maybe the API response channel is closed or timeout"
        ))?;
        Ok(resp_builder.build(resp_type)?)
    }
}

#[cfg(test)]
mod test_ws_connect {
    use std::sync::Arc;

    use crate::{
        api::payload::{ApiPayload, SendGroupMsg},
        connect::{
            ws::{WsConfig, WsConnect},
            WsType,
        },
        event::message::Message,
        message::segment::MfaceData,
        Event, MessageSegment,
    };

    #[tokio::test]
    async fn test_ws_connect() {
        tracing_subscriber::fmt::init();
        let ws_config = WsConfig {
            r#type: WsType::Universal,
            host: "127.0.0.1".to_string(),
            port: 3001,
            access_token: None,
            bot_id: Some("261253615".to_string()),
            bot_nick_name: Some("jinx".to_string()),
        };
        let ws_conn = WsConnect::new(ws_config).await.unwrap();
        let mut subscriber = ws_conn.subscribe().await;
        while let Ok(event) = subscriber.recv().await {
            match event {
                Event::Message(Message::GroupMessage(msg)) => {
                    let group_id = msg.group_id;
                    let message_id = msg.message_id;
                    if msg.sender.user_id == Some(1969730106) {
                        for message in msg.message {
                            if let MessageSegment::Mface {
                                data: MfaceData { url, .. },
                            } = message
                            {
                                let ws_conn = Arc::clone(&ws_conn);
                                tokio::spawn(async move {
                                    let _ = ws_conn
                                        .clone()
                                        .call_api(ApiPayload::SendGroupMsg(SendGroupMsg {
                                            group_id,
                                            message: vec![MessageSegment::text(format!(
                                                "收到Mface消息,url: {}，开始发送图片消息",
                                                url
                                            ))],
                                            auto_escape: true,
                                        }))
                                        .await;
                                    let payload = ApiPayload::SendGroupMsg(SendGroupMsg {
                                        group_id,
                                        message: vec![MessageSegment::easy_image(
                                            url,
                                            Some("Mface Image"),
                                        )],
                                        auto_escape: true,
                                    });
                                    let _ = ws_conn
                                        .clone()
                                        .call_api(ApiPayload::SendGroupMsg(SendGroupMsg {
                                            group_id,
                                            message: vec![MessageSegment::text(format!(
                                                "发送图片消息, json string: {}",
                                                serde_json::to_string_pretty(&payload).unwrap()
                                            ))],
                                            auto_escape: true,
                                        }))
                                        .await;

                                    let _ = ws_conn.call_api(payload).await;
                                });
                            } else if let MessageSegment::File { data: file } = message {
                                let ws_conn = Arc::clone(&ws_conn);
                                tokio::spawn(async move {
                                    let _ = ws_conn
                                        .clone()
                                        .call_api(ApiPayload::SendGroupMsg(SendGroupMsg {
                                            group_id,
                                            message: vec![
                                                MessageSegment::text(format!(
                                                    "File消息: {:#?}",
                                                    file
                                                )),
                                                MessageSegment::reply(message_id.to_string()),
                                            ],
                                            auto_escape: true,
                                        }))
                                        .await;
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
