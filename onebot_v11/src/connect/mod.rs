use serde::{Deserialize, Serialize};

pub mod http;
pub mod ws;
pub mod ws_reverse;

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
