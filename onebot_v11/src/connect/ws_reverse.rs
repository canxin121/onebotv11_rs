use crate::api::payload::ApiPayload;
use crate::api::resp::{ApiResp, ApiRespBuilder};
use crate::Event;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt as _, StreamExt as _};
use reqwest::header::AUTHORIZATION;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio::time::timeout;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::{info, warn};

use super::{get_resp_builder, WsApiPayload, WsType};

pub struct ReverseWsConfig {
    pub host: String,
    pub port: u16,
    pub suffix: String,
    pub access_token: Option<String>,
}

impl Default for ReverseWsConfig {
    fn default() -> Self {
        ReverseWsConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            suffix: "onebot/v11".to_string(),
            access_token: None,
        }
    }
}

pub struct ReverseWsConnect {
    pub config: ReverseWsConfig,
    pub r#type: RwLock<Option<WsType>>,
    pub r#bot_id: RwLock<Option<String>>,
    ws_read: Mutex<SplitStream<WebSocketStream<TcpStream>>>,
    ws_write: Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>,
    event_sender: broadcast::Sender<Event>,
    api_response_sender: broadcast::Sender<ApiRespBuilder>,
}

impl ReverseWsConnect {
    pub async fn new(config: ReverseWsConfig) -> Result<Arc<Self>, anyhow::Error> {
        let ((ws_read, ws_write), bot_id, r#type) = Self::connect(&config).await?;
        let (api_response_sender, _) = broadcast::channel(100);
        let self_ = Arc::new(Self {
            config,
            r#type: RwLock::new(r#type),
            r#bot_id: RwLock::new(bot_id),
            ws_read: Mutex::new(ws_read),
            ws_write: Mutex::new(ws_write),
            event_sender: broadcast::channel(100).0,
            api_response_sender,
        });

        self_.clone().start_event_listener();

        Ok(self_)
    }

    async fn connect(
        config: &ReverseWsConfig,
    ) -> Result<
        (
            (
                SplitStream<WebSocketStream<TcpStream>>,
                SplitSink<WebSocketStream<TcpStream>, Message>,
            ),
            Option<String>,
            Option<WsType>,
        ),
        anyhow::Error,
    > {
        loop {
            let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

            let mut bot_id = None;
            let mut r#type = None;
            match listener.accept().await {
                Ok((stream, _)) => {
                    match accept_hdr_async(stream, |req: &Request, mut resp: Response| {
                        let path = req.uri().path().trim_end_matches('/');
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
                        let bear_token = headers
                            .get(AUTHORIZATION)
                            .map(|v| v.to_str().unwrap_or("").to_string());
                        tracing::info!(
                            "Connection accecpting: bot_id: {:?}, type: {:?}, bear_token: {:?}",
                            bot_id,
                            r#type,
                            bear_token
                        );
                        if bear_token
                            != config
                                .access_token
                                .as_ref()
                                .and_then(|s| Some(format!("Bearer {}", s)))
                        {
                            tracing::error!(
                                "Connection failed: Unauthorized, bear_token: {:?}",
                                bear_token
                            );
                            *resp.status_mut() = reqwest::StatusCode::UNAUTHORIZED;
                            return Ok(resp);
                        }
                        Ok(resp)
                    })
                    .await
                    {
                        Ok(ws_stream) => {
                            let (write, read) = ws_stream.split();

                            info!(
                                "Connection succeed, bot_id: {:?}, type: {:?}",
                                bot_id, r#type
                            );
                            return Ok(((read, write), bot_id, r#type));
                        }
                        Err(e) => {
                            warn!("Connection failed: {}", e);
                        }
                    };
                }
                Err(e) => {
                    warn!("Connection failed: {}", e);
                }
            }
        }
    }

    fn start_event_listener(self: Arc<Self>) {
        let self_clone = Arc::clone(&self);

        tokio::spawn(async move {
            {
                let mut read = self.ws_read.lock().await;
                let sender = self.event_sender.clone();

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
            }
            warn!("WsMessage stream ended, maybe the connection is closed");

            if let Ok(((read, write), bot_id, r#type)) = Self::connect(&self_clone.config).await {
                {
                    *self_clone.ws_read.lock().await = read;
                    *self_clone.ws_write.lock().await = write;
                }
                {
                    *self_clone.r#bot_id.write().await = bot_id;
                    *self_clone.r#type.write().await = r#type;
                }
                self_clone.start_event_listener();
            }
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
            "[WsServer.call_api] Error receiving API response, maybe the API response channel is closed or timeout"
        ))?;
        Ok(resp_builder.build(resp_type)?)
    }
}
