use crate::MessageSegment;
use serde::de::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    PrivateMessage(PrivateMessage),
    GroupMessage(GroupMessage),
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Message::PrivateMessage(m) => m.serialize(serializer),
            Message::GroupMessage(m) => m.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value["message_type"].as_str() {
            Some("private") => serde_json::from_value(value)
                .map(Message::PrivateMessage)
                .map_err(D::Error::custom),
            Some("group") => serde_json::from_value(value)
                .map(Message::GroupMessage)
                .map_err(D::Error::custom),
            _ => Err(D::Error::custom("Invalid message_type")),
        }
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        *place = match value["message_type"].as_str() {
            Some("private") => serde_json::from_value(value)
                .map(Message::PrivateMessage)
                .map_err(D::Error::custom)?,
            Some("group") => serde_json::from_value(value)
                .map(Message::GroupMessage)
                .map_err(D::Error::custom)?,
            _ => return Err(D::Error::custom("Invalid message_type")),
        };
        Ok(())
    }
}

/// 定义私聊消息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PrivateMessage {
    /// 事件发生的时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    pub self_id: i64,
    /// 上报类型
    pub post_type: String,
    /// 消息类型
    pub message_type: String,
    /// 消息子类型
    pub sub_type: String,
    /// 消息 ID
    pub message_id: i64,
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 消息内容
    pub message: Vec<MessageSegment>,
    /// 原始消息内容
    pub raw_message: String,
    /// 字体
    pub font: i64,
    /// 发送人信息
    pub sender: PrivateMessageSender,
}

/// 发送人信息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PrivateMessageSender {
    /// 发送者 QQ 号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// 昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    /// 年龄
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
}

/// 定义群消息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMessage {
    /// 事件发生的时间戳
    pub time: i64,
    /// 收到事件的机器人 QQ 号
    pub self_id: i64,
    /// 上报类型
    pub post_type: String,
    /// 消息类型
    pub message_type: String,
    /// 消息子类型
    pub sub_type: String,
    /// 消息 ID
    pub message_id: i64,
    /// 群号
    pub group_id: i64,
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 匿名信息，如果不是匿名消息则为 null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<Anonymous>,
    /// 消息内容
    pub message: Vec<MessageSegment>,
    /// 原始消息内容
    pub raw_message: String,
    /// 字体
    pub font: i64,
    /// 发送人信息
    pub sender: GroupMessageSender,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMessageSender {
    /// 发送者 QQ 号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// 昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// 群名片／备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    /// 年龄
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    /// 地区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 成员等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 专属头衔
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 匿名消息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Anonymous {
    /// 匿名用户 ID
    pub id: i64,
    /// 匿名用户名称
    pub name: String,
    /// 匿名用户 flag，在调用禁言 API 时需要传入
    pub flag: String,
}

// 以下暂且不用

/// 私聊消息的快速操作结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PrivateMessageQuickOperation {
    /// 要回复的内容
    pub reply: Vec<MessageSegment>,
    /// 消息内容是否作为纯文本发送
    pub auto_escape: bool,
}
/// 群消息的快速操作结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMessageQuickOperation {
    /// 要回复的内容
    pub reply: Vec<MessageSegment>,
    /// 消息内容是否作为纯文本发送
    pub auto_escape: bool,
    /// 是否要在回复开头 at 发送者
    pub at_sender: bool,
    /// 撤回该条消息
    pub delete: bool,
    /// 把发送者踢出群组
    pub kick: bool,
    /// 把发送者禁言
    pub ban: bool,
    /// 禁言时长
    pub ban_duration: i32,
}
