use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tracing::{info, warn};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt as _, StreamExt as _};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::api::payload::ApiPayload;
use crate::api::resp::{ApiResp, ApiRespBuilder};
use crate::Event;
use std::time::Duration;
use tokio::time::{sleep, timeout};

use super::{get_resp_builder, WsApiPayload, WsType};

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

pub struct WsConnect {
    pub config: WsConfig,
    ws_read: Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    ws_write: Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    event_sender: broadcast::Sender<Event>,
    api_response_sender: broadcast::Sender<ApiRespBuilder>,
}

impl WsConnect {
    pub async fn new(wsconfig: WsConfig) -> Result<Arc<Self>, anyhow::Error> {
        let (ws_write, ws_read) = Self::connect(&wsconfig).await;
        let (api_response_sender, _) = broadcast::channel(100);
        let self_ = Arc::new(Self {
            config: wsconfig.clone(),
            ws_read: Mutex::new(ws_read),
            ws_write: Mutex::new(ws_write),
            event_sender: broadcast::channel(100).0,
            api_response_sender,
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
        let self_clone = Arc::clone(&self);

        tokio::spawn(async move {
            {
                let mut read = self.ws_read.lock().await;

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(msg) => {
                            let msg_string = msg.to_string();
                            match serde_json::from_str::<Event>(&msg_string) {
                                Ok(event) => match event {
                                    Event::ApiRespBuilder(api_resp_builder) => {
                                        if let Err(e) =
                                            self.api_response_sender.send(api_resp_builder)
                                        {
                                            warn!("Error sending ApiRespBuilder: {}", e);
                                        }
                                    }
                                    other => {
                                        if let Err(e) = self.event_sender.send(other) {
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
        self.event_sender.subscribe()
    }

    pub async fn call_api(self: Arc<Self>, api_data: ApiPayload) -> Result<ApiResp, anyhow::Error> {
        let resp_type = api_data.to_resp_type();
        let ws_api_data: WsApiPayload = api_data.into();
        let echo = ws_api_data.echo.clone();
        let ws_api_string: String = serde_json::to_string(&ws_api_data)?;
        {
            let mut write = self.ws_write.lock().await;
            write.send(Message::Text(ws_api_string)).await?;
        }
        let subscriber = self.api_response_sender.subscribe();
        let resp_builder = timeout(
            Duration::from_secs(30),
            get_resp_builder(subscriber, echo)
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
