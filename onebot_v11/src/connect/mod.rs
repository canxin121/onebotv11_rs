use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast;

use crate::api::payload::ApiPayload;
use crate::api::resp::ApiRespBuilder;
use crate::traits::EndPoint;

pub mod http;
pub mod ws;
pub mod ws_reverse;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WsApiPayload {
    pub action: String,
    pub params: Value,
    pub echo: String,
}

impl Into<WsApiPayload> for ApiPayload {
    fn into(self) -> WsApiPayload {
        WsApiPayload {
            action: self.endpoint(),
            params: serde_json::to_value(self).unwrap(),
            echo: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect(),
        }
    }
}

async fn get_resp_builder(
    mut subscriber: broadcast::Receiver<ApiRespBuilder>,
    echo: String,
) -> Option<ApiRespBuilder> {
    while let Ok(resp) = subscriber.recv().await {
        if resp.echo == echo {
            return Some(resp);
        }
    }
    None
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WsType {
    Event,
    Api,
    Universal,
}
impl WsType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "event" => WsType::Event,
            "api" => WsType::Api,
            "universal" => WsType::Universal,
            _ => WsType::Universal,
        }
    }
}
