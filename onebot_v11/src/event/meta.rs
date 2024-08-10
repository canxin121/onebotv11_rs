use serde::de::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Meta {
    Lifecycle(Lifecycle),
    Heartbeat(Heartbeat),
}

impl Serialize for Meta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Meta::Lifecycle(m) => m.serialize(serializer),
            Meta::Heartbeat(m) => m.serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for Meta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value["meta_event_type"].as_str() {
            Some("lifecycle") => serde_json::from_value(value)
                .map(Meta::Lifecycle)
                .map_err(D::Error::custom),
            Some("heartbeat") => serde_json::from_value(value)
                .map(Meta::Heartbeat)
                .map_err(D::Error::custom),
            _ => Err(D::Error::custom("Invalid meta_event_type")),
        }
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        *place = match value["meta_event_type"].as_str() {
            Some("lifecycle") => serde_json::from_value(value)
                .map(Meta::Lifecycle)
                .map_err(D::Error::custom)?,
            Some("heartbeat") => serde_json::from_value(value)
                .map(Meta::Heartbeat)
                .map_err(D::Error::custom)?,
            _ => return Err(D::Error::custom("Invalid meta_event_type")),
        };
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Lifecycle {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub meta_event_type: String,
    pub sub_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Heartbeat {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub meta_event_type: String,
    pub status: serde_json::Value,
    pub interval: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub heartbeat_enable: bool,
    pub heartbeat_interval: i64,
}
