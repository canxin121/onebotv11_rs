use crate::api::resp::ApiRespBuilder;

use self::{message::Message, meta::Meta, notice::Notice, request::Request};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Message(Message),
    Meta(Meta),
    Notice(Notice),
    Request(Request),
    ApiRespBuilder(ApiRespBuilder),
}
impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Event::Message(m) => m.serialize(serializer),
            Event::Meta(m) => m.serialize(serializer),
            Event::Notice(m) => m.serialize(serializer),
            Event::Request(m) => m.serialize(serializer),
            Event::ApiRespBuilder(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if value["post_type"].as_null().is_some() {
            return serde_json::from_value(value)
                .map(Event::ApiRespBuilder)
                .map_err(D::Error::custom);
        }
        match value["post_type"].as_str() {
            Some("message") => serde_json::from_value(value)
                .map(Event::Message)
                .map_err(D::Error::custom),
            Some("meta_event") => serde_json::from_value(value)
                .map(Event::Meta)
                .map_err(D::Error::custom),
            Some("notice") => serde_json::from_value(value)
                .map(Event::Notice)
                .map_err(D::Error::custom),
            Some("request") => serde_json::from_value(value)
                .map(Event::Request)
                .map_err(D::Error::custom),
            _ => Err(D::Error::custom("Invalid post_type")),
        }
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        if value["post_type"].as_null().is_some() {
            *place = serde_json::from_value(value)
                .map(Event::ApiRespBuilder)
                .map_err(D::Error::custom)?;
            return Ok(());
        }
        *place = match value["post_type"].as_str() {
            Some("message") => serde_json::from_value(value)
                .map(Event::Message)
                .map_err(D::Error::custom)?,
            Some("meta_event") => serde_json::from_value(value)
                .map(Event::Meta)
                .map_err(D::Error::custom)?,
            Some("notice") => serde_json::from_value(value)
                .map(Event::Notice)
                .map_err(D::Error::custom)?,
            Some("request") => serde_json::from_value(value)
                .map(Event::Request)
                .map_err(D::Error::custom)?,
            _ => return Err(D::Error::custom("Invalid post_type")),
        };
        Ok(())
    }
}
