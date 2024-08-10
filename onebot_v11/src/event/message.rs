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

// 定义私聊消息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PrivateMessage {
    pub time: i64,                    // 事件发生的时间戳
    pub self_id: i64,                 // 收到事件的机器人 QQ 号
    pub post_type: String,            // 上报类型
    pub message_type: String,         // 消息类型
    pub sub_type: String,             // 消息子类型
    pub message_id: i64,              // 消息 ID
    pub user_id: i64,                 // 发送者 QQ 号
    pub message: Vec<MessageSegment>, // 消息内容
    pub raw_message: String,          // 原始消息内容
    pub font: i64,                    // 字体
    pub sender: PrivateMessageSender, // 发送人信息
}

// 发送人信息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PrivateMessageSender {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>, // 发送者 QQ 号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>, // 昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>, // 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>, // 年龄
}

// 定义群消息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMessage {
    pub time: i64,            // 事件发生的时间戳
    pub self_id: i64,         // 收到事件的机器人 QQ 号
    pub post_type: String,    // 上报类型
    pub message_type: String, // 消息类型
    pub sub_type: String,     // 消息子类型
    pub message_id: i64,      // 消息 ID
    pub group_id: i64,        // 群号
    pub user_id: i64,         // 发送者 QQ 号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<Anonymous>, // 匿名信息，如果不是匿名消息则为 null
    pub message: Vec<MessageSegment>, // 消息内容
    pub raw_message: String,  // 原始消息内容
    pub font: i64,            // 字体
    pub sender: GroupMessageSender, // 发送人信息
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMessageSender {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>, // 发送者 QQ 号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>, // 昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>, // 群名片／备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>, // 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>, // 年龄
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>, // 地区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>, // 成员等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>, // 角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, // 专属头衔
}

// 匿名消息结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Anonymous {
    pub id: i64,      // 匿名用户 ID
    pub name: String, // 匿名用户名称
    pub flag: String, // 匿名用户 flag，在调用禁言 API 时需要传入
}

// 以下暂且不用

// // 私聊消息的快速操作结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PrivateMessageQuickOperation {
    pub reply: Vec<MessageSegment>, // 要回复的内容
    pub auto_escape: bool,          // 消息内容是否作为纯文本发送
}
// 群消息的快速操作结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMessageQuickOperation {
    pub reply: Vec<MessageSegment>, // 要回复的内容
    pub auto_escape: bool,          // 消息内容是否作为纯文本发送
    pub at_sender: bool,            // 是否要在回复开头 at 发送者
    pub delete: bool,               // 撤回该条消息
    pub kick: bool,                 // 把发送者踢出群组
    pub ban: bool,                  // 把发送者禁言
    pub ban_duration: i32,          // 禁言时长
}
