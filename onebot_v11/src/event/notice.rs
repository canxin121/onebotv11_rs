use serde::de::Error;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Notice {
    // 群文件上传事件
    GroupFileUpload(GroupFileUploadEvent),
    // 群管理员变动事件
    GroupAdminChange(GroupAdminChangeEvent),
    // 群成员减少事件
    GroupMemberDecrease(GroupMemberDecreaseEvent),
    // 群成员增加事件
    GroupMemberIncrease(GroupMemberIncreaseEvent),
    // 群禁言事件
    GroupBan(GroupBanEvent),
    // 好友添加事件
    FriendAdd(FriendAddEvent),
    // 群消息撤回事件
    GroupMessageRecall(GroupMessageRecallEvent),
    // 好友消息撤回事件
    FriendMessageRecall(FriendMessageRecallEvent),
    // 群内戳一戳事件
    GroupPoke(GroupPokeEvent),
    // 群红包运气王事件
    GroupLuckyKing(GroupLuckyKingEvent),
    // 群成员荣誉变更事件
    GroupMemberHonorChange(GroupMemberHonorChangeEvent),

    // 仅Napcat/llonebot支持的事件
    // 私聊输入状态事件
    FriendInputStatusChange(FriendInputStatusChangeEvent),
    // 群精华消息事件
    GroupEssenceMessageChange(GroupEssenceMessageChangeEvent),
    // 群名片变更事件
    GroupCardChange(GroupCardChangeEvent),
}
impl<'de> Deserialize<'de> for Notice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value["notice_type"].as_str() {
            Some("group_upload") => serde_json::from_value(value)
                .map(Notice::GroupFileUpload)
                .map_err(D::Error::custom),
            Some("group_admin") => serde_json::from_value(value)
                .map(Notice::GroupAdminChange)
                .map_err(D::Error::custom),
            Some("group_decrease") => serde_json::from_value(value)
                .map(Notice::GroupMemberDecrease)
                .map_err(D::Error::custom),
            Some("group_increase") => serde_json::from_value(value)
                .map(Notice::GroupMemberIncrease)
                .map_err(D::Error::custom),
            Some("group_ban") => serde_json::from_value(value)
                .map(Notice::GroupBan)
                .map_err(D::Error::custom),
            Some("friend_add") => serde_json::from_value(value)
                .map(Notice::FriendAdd)
                .map_err(D::Error::custom),
            Some("group_recall") => serde_json::from_value(value)
                .map(Notice::GroupMessageRecall)
                .map_err(D::Error::custom),
            Some("friend_recall") => serde_json::from_value(value)
                .map(Notice::FriendMessageRecall)
                .map_err(D::Error::custom),
            // 仅Napcat/llonebot支持的事件
            Some("essence") => serde_json::from_value(value)
                .map(Notice::GroupEssenceMessageChange)
                .map_err(D::Error::custom),
            Some("group_card") => serde_json::from_value(value)
                .map(Notice::GroupCardChange)
                .map_err(D::Error::custom),

            Some("notify") => match value["sub_type"].as_str() {
                Some("poke") => serde_json::from_value(value)
                    .map(Notice::GroupPoke)
                    .map_err(D::Error::custom),
                Some("lucky_king") => serde_json::from_value(value)
                    .map(Notice::GroupLuckyKing)
                    .map_err(D::Error::custom),
                Some("honor") => serde_json::from_value(value)
                    .map(Notice::GroupMemberHonorChange)
                    .map_err(D::Error::custom),
                // 仅Napcat/llonebot支持的事件
                Some("input_status") => serde_json::from_value(value)
                    .map(Notice::FriendInputStatusChange)
                    .map_err(D::Error::custom),
                _ => Err(D::Error::custom("Invalid notify sub_type")),
            },
            _ => Err(D::Error::custom("Invalid notice_type")),
        }
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        *place = match value["notice_type"].as_str() {
            Some("group_upload") => serde_json::from_value(value)
                .map(Notice::GroupFileUpload)
                .map_err(D::Error::custom)?,
            Some("group_admin") => serde_json::from_value(value)
                .map(Notice::GroupAdminChange)
                .map_err(D::Error::custom)?,
            Some("group_decrease") => serde_json::from_value(value)
                .map(Notice::GroupMemberDecrease)
                .map_err(D::Error::custom)?,
            Some("group_increase") => serde_json::from_value(value)
                .map(Notice::GroupMemberIncrease)
                .map_err(D::Error::custom)?,
            Some("group_ban") => serde_json::from_value(value)
                .map(Notice::GroupBan)
                .map_err(D::Error::custom)?,
            Some("friend_add") => serde_json::from_value(value)
                .map(Notice::FriendAdd)
                .map_err(D::Error::custom)?,
            Some("group_recall") => serde_json::from_value(value)
                .map(Notice::GroupMessageRecall)
                .map_err(D::Error::custom)?,
            Some("friend_recall") => serde_json::from_value(value)
                .map(Notice::FriendMessageRecall)
                .map_err(D::Error::custom)?,
            // 仅Napcat/llonebot支持的事件
            Some("essence") => serde_json::from_value(value)
                .map(Notice::GroupEssenceMessageChange)
                .map_err(D::Error::custom)?,
            Some("group_card") => serde_json::from_value(value)
                .map(Notice::GroupCardChange)
                .map_err(D::Error::custom)?,

            Some("notify") => match value["sub_type"].as_str() {
                Some("poke") => serde_json::from_value(value)
                    .map(Notice::GroupPoke)
                    .map_err(D::Error::custom)?,
                Some("lucky_king") => serde_json::from_value(value)
                    .map(Notice::GroupLuckyKing)
                    .map_err(D::Error::custom)?,
                Some("honor") => serde_json::from_value(value)
                    .map(Notice::GroupMemberHonorChange)
                    .map_err(D::Error::custom)?,
                // 仅Napcat/llonebot支持的事件
                Some("input_status") => serde_json::from_value(value)
                    .map(Notice::FriendInputStatusChange)
                    .map_err(D::Error::custom)?,
                _ => return Err(D::Error::custom("Invalid notify sub_type")),
            },
            _ => return Err(D::Error::custom("Invalid notice_type")),
        };
        Ok(())
    }
}

impl Serialize for Notice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Notice::GroupFileUpload(m) => m.serialize(serializer),
            Notice::GroupAdminChange(m) => m.serialize(serializer),
            Notice::GroupMemberDecrease(m) => m.serialize(serializer),
            Notice::GroupMemberIncrease(m) => m.serialize(serializer),
            Notice::GroupBan(m) => m.serialize(serializer),
            Notice::FriendAdd(m) => m.serialize(serializer),
            Notice::GroupMessageRecall(m) => m.serialize(serializer),
            Notice::FriendMessageRecall(m) => m.serialize(serializer),
            Notice::GroupPoke(m) => m.serialize(serializer),
            Notice::GroupLuckyKing(m) => m.serialize(serializer),
            Notice::GroupMemberHonorChange(m) => m.serialize(serializer),
            Notice::FriendInputStatusChange(m) => m.serialize(serializer),
            Notice::GroupEssenceMessageChange(m) => m.serialize(serializer),
            Notice::GroupCardChange(m) => m.serialize(serializer),
        }
    }
}

// 群文件上传事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupFileUploadEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub group_id: i64,       // 群号
    pub user_id: i64,        // 发送者 QQ 号
    pub file: GroupFile,     // 文件信息
}

// 群文件信息
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupFile {
    pub id: String,   // 文件 ID
    pub name: String, // 文件名
    pub size: i64,    // 文件大小（字节数）
    pub busid: i64,   // busid（目前不清楚有什么作用）
}

// 群管理员变动事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupAdminChangeEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 事件子类型，分别表示设置和取消管理员
    pub group_id: i64,       // 群号
    pub user_id: i64,        // 管理员 QQ 号
}

// 群成员减少事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMemberDecreaseEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 事件子类型，分别表示主动退群、成员被踢、登录号被踢
    pub group_id: i64,       // 群号
    pub operator_id: i64,    // 操作者 QQ 号（如果是主动退群，则和 `user_id` 相同）
    pub user_id: i64,        // 离开者 QQ 号
}

// 群成员增加事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMemberIncreaseEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 事件子类型，分别表示管理员已同意入群、管理员邀请入群
    pub group_id: i64,       // 群号
    pub operator_id: i64,    // 操作者 QQ 号
    pub user_id: i64,        // 加入者 QQ 号
}

// 群禁言事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupBanEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 事件子类型，分别表示禁言、解除禁言
    pub group_id: i64,       // 群号
    pub operator_id: i64,    // 操作者 QQ 号
    pub user_id: i64,        // 被禁言 QQ 号
    pub duration: i64,       // 禁言时长，单位秒
}

// 好友添加事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FriendAddEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub user_id: i64,        // 新添加好友 QQ 号
}

// 群消息撤回事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMessageRecallEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub group_id: i64,       // 群号
    pub user_id: i64,        // 消息发送者 QQ 号
    pub operator_id: i64,    // 操作者 QQ 号
    pub message_id: i64,     // 被撤回的消息 ID
}

// 好友消息撤回事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FriendMessageRecallEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub user_id: i64,        // 好友 QQ 号
    pub message_id: i64,     // 被撤回的消息 ID
}

// 群内戳一戳事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupPokeEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 子类型
    pub group_id: i64,       // 群号
    pub user_id: i64,        // 发送者 QQ 号
    pub target_id: i64,      // 被戳者 QQ 号
}

// 群红包运气王事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupLuckyKingEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 子类型
    pub group_id: i64,       // 群号
    pub user_id: i64,        // 红包发送者 QQ 号
    pub target_id: i64,      // 运气王 QQ 号
}

// 群成员荣誉变更事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMemberHonorChangeEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 子类型
    pub group_id: i64,       // 群号
    pub honor_type: String,  // 荣誉类型，分别表示龙王、群聊之火、快乐源泉
    pub user_id: i64,        // 成员 QQ 号
}

// 仅Napcat/llonebot支持的事件
// 私聊输入状态事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FriendInputStatusChangeEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub sub_type: String,    // 子类型
    pub status_text: String, // 输入状态文本
    pub event_type: u8, // 事件类型
    pub user_id: i64,        // 好友 QQ 号
}

// 输入状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputStatus {
    // 开始输入
    Start,
    // 继续输入
    Typing,
    // 清空输入
    Cleared,
}

impl FriendInputStatusChangeEvent {
    pub fn to_status(&self) -> InputStatus {
        if !self.status_text.is_empty() {
            if self.event_type == 1 {
                InputStatus::Start
            } else {
                InputStatus::Typing
            }
        } else {
            InputStatus::Cleared
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum EssenseMessageChangeType {
    // 精华消息添加
    #[serde(rename = "add")]
    Add,
    // 精华消息删除
    #[serde(rename = "delete")]
    Delete,
}

// 仅Napcat/llonebot支持的事件

// 精华消息事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupEssenceMessageChangeEvent {
    pub time: i64,                          // 事件发生的时间戳
    pub self_id: i64,                       // 收到事件的机器人 QQ 号
    pub post_type: String,                  // 上报类型
    pub group_id: i64,                      // 群号
    pub user_id: i64,                       // 操作者 QQ 号
    pub notice_type: String,                // 通知类型
    pub message_id: i64,                    // 消息 ID
    pub sender_id: i64,                     // 发送者 QQ 号
    pub sub_type: EssenseMessageChangeType, // 子类型
}

// 群名片变更事件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupCardChangeEvent {
    pub time: i64,           // 事件发生的时间戳
    pub self_id: i64,        // 收到事件的机器人 QQ 号
    pub post_type: String,   // 上报类型
    pub notice_type: String, // 通知类型
    pub group_id: i64,       // 群号
    pub user_id: i64,        // 用户 QQ 号
    pub card_new: String,    // 新名片
    pub card_old: String,    // 旧名片
}
