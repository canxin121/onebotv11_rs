use serde_json::Value;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Mutex};
use tracing::{warn,info};
use crate::api::payload::ApiPayload;
use crate::api::resp::{ApiResp, ApiRespBuilder};
use crate::traits::EndPoint as _;
use crate::Event;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt as _, StreamExt as _};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

use super::WsType;

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

pub struct ReverseWsConfig {
    pub host: String,
    pub port: u16,
    pub suffix: String,
}

impl Default for ReverseWsConfig {
    fn default() -> Self {
        ReverseWsConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            suffix: "onebot/v11".to_string(),
        }
    }
}

pub struct ReverseWsConnect {
    pub config: ReverseWsConfig,
    pub r#type: Option<WsType>,
    pub r#bot_id: Option<String>,
    ws_read: Arc<Mutex<SplitStream<WebSocketStream<TcpStream>>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    event_sender: Arc<Mutex<broadcast::Sender<Event>>>,
    api_response_sender: Arc<Mutex<mpsc::Sender<ApiRespBuilder>>>,
    api_response_receiver: Arc<Mutex<mpsc::Receiver<ApiRespBuilder>>>,
}

impl ReverseWsConnect {
    pub async fn new(config: ReverseWsConfig) -> Result<Arc<Self>, anyhow::Error> {
        let (ws_read, ws_write) = Self::connect(&config).await?;
        let (api_response_sender, api_response_receiver) = mpsc::channel(100);
        let self_ = Arc::new(Self {
            config,
            r#type: None,
            r#bot_id: None,
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
        config: &ReverseWsConfig,
    ) -> Result<
        (
            SplitStream<WebSocketStream<TcpStream>>,
            SplitSink<WebSocketStream<TcpStream>, Message>,
        ),
        anyhow::Error,
    > {
        loop {
            let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
            match listener.accept().await {
                Ok((stream, _)) => {
                    let mut bot_id = None;
                    let mut r#type = None;
                    match accept_hdr_async(stream, |req: &Request, mut resp: Response| {
                        let path = req.uri().path().trim_end_matches('/');
                        info!(
                            "[ReverseWsConnect::new] Accepting connection, path suffix: {}",
                            path
                        );

                        if !path.ends_with(&config.suffix) {
                            *resp.status_mut() = reqwest::StatusCode::NOT_FOUND;
                            return Ok(resp);
                        }
                        let headers = req.headers();
                        bot_id = headers
                            .get("X-Self-ID")
                            .map(|v| v.to_str().unwrap_or("").to_string());
                        r#type = headers
                            .get("X-Client-Role")
                            .map(|v| WsType::from_str(&v.to_str().unwrap_or("").to_string()));
                        Ok(resp)
                    })
                    .await
                    {
                        Ok(ws_stream) => {
                            let (write, read) = ws_stream.split();

                            info!(
                                "[ReverseWsConnect::connect] Connected, bot_id: {:?}, type: {:?}",
                                bot_id, r#type
                            );
                            return Ok((read, write));
                        }
                        Err(e) => {
                            warn!(
                                "[ReverseWsConnect::connect] Error accepting connection: {}",
                                e
                            );
                        }
                    };
                }
                Err(e) => {
                    warn!(
                        "[ReverseWsConnect::connect] Error accepting connection: {}",
                        e
                    );
                }
            }
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
                                    warn!(
                                    "Error parsing Event: {}, Raw: {}",
                                    e, msg_string
                                );
                                }
                            }
                        }
                        Err(e) => {
                            warn!(
                                "Error receiving WsMessage: {}",
                                e
                            );
                        }
                    }
                }
            }
            warn!(
                "WsMessage stream ended, maybe the connection is closed"
            );

            if let Ok((read, write)) = Self::connect(&self_clone.config).await {
                {
                    *self_clone.ws_read.lock().await = read;
                    *self_clone.ws_write.lock().await = write;
                }
                self_clone.start_event_listener();
            }
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
            "[WsServer.call_api] Error receiving API response, maybe the API response channel is closed or timeout"
        ))?;
        Ok(resp_builder.build(resp_type)?)
    }
}

#[cfg(test)]
mod test_reverse_ws_connect {
    use std::sync::Arc;

    use crate::{
        api::payload::{ApiPayload, SendGroupMsg},
        connect::ws_reverse::ReverseWsConnect,
        event::notice::Notice,
        Event, MessageSegment,
    };

    #[tokio::test]
    async fn test() {
        tracing_subscriber::fmt::init();
        
        let ws_conn = ReverseWsConnect::new(Default::default()).await.unwrap();
        let mut subscriber = ws_conn.subscribe().await;

        while let Ok(event) = subscriber.recv().await {
            println!("Received event: {:#?}", event);
            match event {
                Event::Notice(Notice::GroupCardChange(event)) => {
                    let ws_conn = Arc::clone(&ws_conn);
                    tokio::spawn(async move {
                        let _ = ws_conn
                            .call_api(ApiPayload::SendGroupMsg(SendGroupMsg {
                                group_id: event.group_id,
                                message: vec![MessageSegment::text(format!(
                                    "收到群名片变动消息: {:#?}",
                                    event
                                ))],
                                auto_escape: true,
                            }))
                            .await;
                    });
                }
                _ => {}
            }
        }
    }
}
