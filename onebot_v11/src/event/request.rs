use serde::de::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    FriendRequestEvent(FriendRequestEvent),
    GroupRequestEvent(GroupRequestEvent),
}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Request::FriendRequestEvent(m) => m.serialize(serializer),
            Request::GroupRequestEvent(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Request {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value["request_type"].as_str() {
            Some("friend") => serde_json::from_value(value)
                .map(Request::FriendRequestEvent)
                .map_err(D::Error::custom),
            Some("group") => serde_json::from_value(value)
                .map(Request::GroupRequestEvent)
                .map_err(D::Error::custom),
            _ => Err(D::Error::custom("Invalid request_type")),
        }
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        *place = match value["request_type"].as_str() {
            Some("friend") => serde_json::from_value(value)
                .map(Request::FriendRequestEvent)
                .map_err(D::Error::custom)?,
            Some("group") => serde_json::from_value(value)
                .map(Request::GroupRequestEvent)
                .map_err(D::Error::custom)?,
            _ => return Err(D::Error::custom("Invalid request_type")),
        };
        Ok(())
    }
}

// 加好友请求事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FriendRequestEvent {
    pub time: i64,            // 事件发生的时间戳
    pub self_id: i64,         // 收到事件的机器人 QQ 号
    pub post_type: String,    // 上报类型
    pub request_type: String, // 请求类型
    pub user_id: i64,         // 发送请求的 QQ 号
    pub comment: String,      // 验证信息
    pub flag: String,         // 请求 flag，在调用处理请求的 API 时需要传入
}

// 加群请求/邀请事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupRequestEvent {
    pub time: i64,            // 事件发生的时间戳
    pub self_id: i64,         // 收到事件的机器人 QQ 号
    pub post_type: String,    // 上报类型
    pub request_type: String, // 请求类型
    pub sub_type: String,     // 请求子类型，分别表示加群请求、邀请登录号入群
    pub group_id: i64,        // 群号
    pub user_id: i64,         // 发送请求的 QQ 号
    pub comment: String,      // 验证信息
    pub flag: String,         // 请求 flag，在调用处理请求的 API 时需要传入
}
