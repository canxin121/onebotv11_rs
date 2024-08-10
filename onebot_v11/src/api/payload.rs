use crate::{event::message::Anonymous, traits::EndPoint, MessageSegment};
use onebot_v11_macro::{endpoint, ApiDataDerive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, ApiDataDerive)]
pub enum ApiPayload {
    // 发送私聊消息
    SendPrivateMsg(SendPrivateMsg),
    // 发送群消息
    SendGroupMsg(SendGroupMsg),
    // 发送消息
    SendMsg(SendMsg),
    // 撤回消息
    DeleteMsg(DeleteMsg),
    // 获取消息
    GetMsg(GetMsg),
    // 获取合并转发消息
    GetForwardMsg(GetForwardMsg),
    // 发送好友赞
    SendLike(SendLike),
    // 群组踢人
    SetGroupKick(SetGroupKick),
    // 群组禁言
    SetGroupBan(SetGroupBan),
    // 群组匿名用户禁言
    SetGroupAnonymousBan(SetGroupAnonymousBan),
    // 群组全员禁言
    SetGroupWholeBan(SetGroupWholeBan),
    // 群组设置管理员
    SetGroupAdmin(SetGroupAdmin),
    // 群组匿名
    SetGroupAnonymous(SetGroupAnonymous),
    // 设置群名片（群备注）
    SetGroupCard(SetGroupCard),
    // 设置群名
    SetGroupName(SetGroupName),
    // 退出群组
    SetGroupLeave(SetGroupLeave),
    // 设置群组专属头衔
    SetGroupSpecialTitle(SetGroupSpecialTitle),
    // 处理加好友请求
    SetFriendAddRequest(SetFriendAddRequest),
    // 处理加群请求／邀请
    SetGroupAddRequest(SetGroupAddRequest),
    // 获取登录号信息
    GetLoginInfo(GetLoginInfo),
    // 获取陌生人信息
    GetStrangerInfo(GetStrangerInfo),
    // 获取好友列表
    GetFriendList(GetFriendList),
    // 获取群信息
    GetGroupInfo(GetGroupInfo),
    // 获取群列表
    GetGroupList(GetGroupList),
    // 获取群成员信息
    GetGroupMemberInfo(GetGroupMemberInfo),
    // 获取群成员列表
    GetGroupMemberList(GetGroupMemberList),
    // 获取群荣誉信息
    GetGroupHonorInfo(GetGroupHonorInfo),
    // 获取 Cookies
    GetCookies(GetCookies),
    // 获取 CSRF Token
    GetCsrfToken(GetCsrfToken),
    // 获取 QQ 相关接口凭证
    GetCredentials(GetCredentials),
    // 获取语音
    GetRecord(GetRecord),
    // 获取图片
    GetImage(GetImage),
    // 检查是否可以发送图片
    CanSendImage(CanSendImage),
    // 检查是否可以发送语音
    CanSendRecord(CanSendRecord),
    // 获取运行状态
    GetStatus(GetStatus),
    // 获取版本信息
    GetVersionInfo(GetVersionInfo),
    // 重启 OneBot 实现
    SetRestart(SetRestart),
    // 清理 OneBot 实现缓存
    CleanCache(CleanCache),
    // NapCat/llonebot扩展

    // 设置头像
    SetQQAvatar(SetQQAvatar),
    // 获取群系统通知
    GetGroupSystemMsg(GetGroupSystemMsg),
    // 下载群文件或私聊文件
    GetFile(GetFile),
    // 转发单条消息给好友
    ForwardFriendSingleMsg(ForwardFriendSingleMsg),
    // 转发单条消息给群
    ForwardGroupSingleMsg(ForwardGroupSingleMsg),
    // 设置表情回应
    SetMsgEmojiLike(SetMsgEmojiLike),
    // 标记私聊消息为已读
    MarkPrivateMsgAsRead(MarkPrivateMsgAsRead),
    // 标记群消息为已读
    MarkGroupMsgAsRead(MarkGroupMsgAsRead),
    // 获取官方bot qq号范围
    GetRobotUinRange(GetRobotUinRange),
    // 设置自身在线状态
    SetOnlineStatus(SetOnlineStatus),
    // 获取好友分类列表
    GetFriendsWithCategory(GetFriendsWithCategory),
    // 获取群文件数量
    GetGroupFileCount(GetGroupFileCount),
    // 获取群文件列表
    GetGroupFileList(GetGroupFileList),
    // 创建群文件夹
    SetGroupFileFolder(SetGroupFileFolder),
    // 删除群文件
    DelGroupFile(DelGroupFile),
    // 删除群文件夹
    DelGroupFileFolder(DelGroupFileFolder),

    // NapCat gocq拓展

    // 合并转发消息给群聊
    SendGroupForwardMsg(SendGroupForwardMsg),
    // 合并转发消息给好友
    SendPrivateForwardMsg(SendPrivateForwardMsg),
}

// 发送私聊消息结构体
#[endpoint("send_private_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendPrivateMsg {
    pub user_id: i64,                 // 对方 QQ 号
    pub message: Vec<MessageSegment>, // 要发送的内容
    pub auto_escape: bool, // 消息内容是否作为纯文本发送（即不解析 CQ 码），只在 message 字段是字符串时有效
}

// 发送群消息结构体
#[endpoint("send_group_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendGroupMsg {
    pub group_id: i64,                // 群号
    pub message: Vec<MessageSegment>, // 要发送的消息内容
    pub auto_escape: bool,            // 消息内容是否作为纯文本发送（不解析 CQ 码）
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
}

// 发送消息结构体
#[endpoint("send_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendMsg {
    pub message_type: MessageType,    // 消息类型（private 或 group）
    pub user_id: Option<i64>,         // 对方 QQ 号（私聊时需要）
    pub group_id: Option<i64>,        // 群号（群聊时需要）
    pub message: Vec<MessageSegment>, // 要发送的消息内容
    pub auto_escape: bool,            // 消息内容是否作为纯文本发送（不解析 CQ 码）
}

// 撤回消息结构体
#[endpoint("delete_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeleteMsg {
    pub message_id: i32, // 消息 ID
}

// 获取消息结构体
#[endpoint("get_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMsg {
    pub message_id: i32, // 消息 ID
}

// 获取合并转发消息结构体
#[endpoint("get_forward_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetForwardMsg {
    pub id: String, // 合并转发 ID
}

// 发送好友赞结构体
#[endpoint("send_like")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendLike {
    pub user_id: i64, // 对方 QQ 号
    pub times: i32,   // 赞的次数，每个好友每天最多 10 次
}

// 群组踢人结构体
#[endpoint("set_group_kick")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupKick {
    pub group_id: i64,            // 群号
    pub user_id: i64,             // 要踢的 QQ 号
    pub reject_add_request: bool, // 拒绝此人的加群请求
}

// 群组单人禁言结构体
#[endpoint("set_group_ban")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupBan {
    pub group_id: i64, // 群号
    pub user_id: i64,  // 要禁言的 QQ 号
    pub duration: i32, // 禁言时长，单位秒，0 表示取消禁言
}

// 群组匿名用户禁言结构体
#[endpoint("set_group_anonymous_ban")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupAnonymousBan {
    pub group_id: i64,                  // 群号
    pub anonymous: Option<Anonymous>,   // 可选，要禁言的匿名用户对象
    pub anonymous_flag: Option<String>, // 可选，要禁言的匿名用户的 flag
    pub duration: i32,                  // 禁言时长，单位秒，无法取消匿名用户禁言
}

// 群组全员禁言结构体
#[endpoint("set_group_whole_ban")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupWholeBan {
    pub group_id: i64, // 群号
    pub enable: bool,  // 是否禁言
}

// 群组设置管理员结构体
#[endpoint("set_group_admin")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupAdmin {
    pub group_id: i64, // 群号
    pub user_id: i64,  // 要设置管理员的 QQ 号
    pub enable: bool,  // true 为设置，false 为取消
}

// 群组匿名结构体
#[endpoint("set_group_anonymous")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupAnonymous {
    pub group_id: i64, // 群号
    pub enable: bool,  // 是否允许匿名聊天
}

// 设置群名片（群备注）结构体
#[endpoint("set_group_card")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupCard {
    pub group_id: i64, // 群号
    pub user_id: i64,  // 要设置的 QQ 号
    pub card: String,  // 群名片内容，不填或空字符串表示删除群名片
}

// 设置群名结构体
#[endpoint("set_group_name")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupName {
    pub group_id: i64,      // 群号
    pub group_name: String, // 新群名
}

// 退出群组结构体
#[endpoint("set_group_leave")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupLeave {
    pub group_id: i64,    // 群号
    pub is_dismiss: bool, // 是否解散，如果登录号是群主，则仅在此项为 true 时能够解散
}

// 设置群组专属头衔结构体
#[endpoint("set_group_special_title")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupSpecialTitle {
    pub group_id: i64,         // 群号
    pub user_id: i64,          // 要设置的 QQ 号
    pub special_title: String, // 专属头衔，不填或空字符串表示删除专属头衔
    pub duration: i32,         // 专属头衔有效期，单位秒，-1 表示永久，此项似乎没有效果，有待测试
}

// 处理加好友请求结构体
#[endpoint("set_friend_add_request")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetFriendAddRequest {
    pub flag: String,   // 加好友请求的 flag
    pub approve: bool,  // 是否同意请求
    pub remark: String, // 添加后的好友备注（仅在同意时有效）
}

// 处理加群请求／邀请结构体
#[endpoint("set_group_add_request")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupAddRequest {
    pub flag: String,     // 加群请求的 flag
    pub sub_type: String, // 请求类型（add 或 invite）
    pub approve: bool,    // 是否同意请求／邀请
    pub reason: String,   // 拒绝理由（仅在拒绝时有效）
}

// 获取登录号信息结构体
#[endpoint("get_login_info")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetLoginInfo {}

// 获取陌生人信息结构体
#[endpoint("get_stranger_info")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetStrangerInfo {
    pub user_id: i64,   // QQ 号
    pub no_cache: bool, // 是否不使用缓存
}

// 获取好友列表结构体
#[endpoint("get_friend_list")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendList {}

// 获取群信息结构体
#[endpoint("get_group_info")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupInfo {
    pub group_id: i64,  // 群号
    pub no_cache: bool, // 是否不使用缓存
}

// 获取群列表结构体
#[endpoint("get_group_list")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupList {}

// 获取群成员信息结构体
#[endpoint("get_group_member_info")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupMemberInfo {
    pub group_id: i64,  // 群号
    pub user_id: i64,   // QQ 号
    pub no_cache: bool, // 是否不使用缓存
}

// 获取群成员列表结构体
#[endpoint("get_group_member_list")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupMemberList {
    pub group_id: i64, // 群号
}

// 获取群荣誉信息结构体
#[endpoint("get_group_honor_info")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupHonorInfo {
    pub group_id: i64, // 群号
    #[serde(rename = "type")]
    pub honor_type: String, // 要获取的群荣誉类型
}

// 获取 Cookies 结构体
#[endpoint("get_cookies")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetCookies {
    pub domain: String, // 需要获取 cookies 的域名
}

// 获取 CSRF Token 结构体
#[endpoint("get_csrf_token")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetCsrfToken {}

// 获取 QQ 相关接口凭证结构体
#[endpoint("get_credentials")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetCredentials {
    pub domain: String, // 需要获取 cookies 的域名
}

// 获取语音结构体
#[endpoint("get_record")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetRecord {
    pub file: String,       // 收到的语音文件名
    pub out_format: String, // 要转换到的格式
}

// 获取图片结构体
#[endpoint("get_image")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetImage {
    pub file: String, // 收到的图片文件名
}

// 检查是否可以发送图片结构体
#[endpoint("can_send_image")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CanSendImage {}

// 检查是否可以发送语音结构体
#[endpoint("can_send_record")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CanSendRecord {}

// 获取运行状态结构体
#[endpoint("get_status")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetStatus {}

// 获取版本信息结构体
#[endpoint("get_version_info")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetVersionInfo {}

// 重启 OneBot 实现结构体
#[endpoint("set_restart")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetRestart {
    pub delay: i32, // 要延迟的毫秒数
}

// 清理缓存结构体
#[endpoint("clean_cache")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CleanCache {}

// NapCat/llonebot扩展

// 设置头像✔
#[endpoint("set_qq_avatar")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetQQAvatar {
    pub file: String, // 图片路径/链接/base64
}

// 获取群系统通知✔
#[endpoint("get_group_system_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupSystemMsg {
    pub group_id: i64, // 群号
}

// 下载群文件或私聊文件
#[endpoint("get_file")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFile {
    pub file_id: String, // 文件 ID
}

// 转发单条消息给好友✔
#[endpoint("forward_friend_single_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForwardFriendSingleMsg {
    pub user_id: i64,    // 对方 QQ 号
    pub message_id: i64, // 消息 ID
}

// 转发单条消息给群✔
#[endpoint("forward_group_single_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForwardGroupSingleMsg {
    pub group_id: i64,   // 群号
    pub message_id: i64, // 消息 ID
}

// 设置表情回应✔
#[endpoint("set_msg_emoji_like")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMsgEmojiLike {
    pub message_id: String, // 消息 ID
    // emoji_id 参考 https://bot.q.qq.com/wiki/develop/api-v2/openapi/emoji/model.html#EmojiType
    pub emoji_id: String, // 表情 ID
}

// 设置私聊消息已读✔
#[endpoint("mark_private_msg_as_read")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MarkPrivateMsgAsRead {
    pub user_id: i64, // 对方 QQ 号
}

// 设置群消息已读✔
#[endpoint("mark_group_msg_as_read")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MarkGroupMsgAsRead {
    pub group_id: i64, // 群号
}

// 获取官方bot qq号范围✔
#[endpoint("get_robot_uin_range")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetRobotUinRange {}

// 设置自身在线状态✔
// 参考: https://napneko.github.io/zh-CN/develop/status_list
#[endpoint("set_online_status")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetOnlineStatus {
    pub status: u32, // 在线状态
    #[serde(rename = "extStatus")]
    pub ext_status: u32, // 扩展在线状态
    #[serde(rename = "batteryStatus")]
    pub battery_status: u32, // 电量状态
}

// 获取好友分类列表(untested)
#[endpoint("get_friends_with_category")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendsWithCategory {}

// 获取群文件数量✔
#[endpoint("get_group_file_count")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupFileCount {
    pub group_id: i64, // 群号
}

// 获取群文件列表✔
#[endpoint("get_group_file_list")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupFileList {
    pub group_id: i64,    // 群号
    pub start_index: i64, // 起始文件序号
    pub file_count: i64,  // 获取的文件数量
}

// 创建群文件夹✔
#[endpoint("set_group_file_folder")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupFileFolder {
    pub group_id: i64,       // 群号
    pub folder_name: String, // 文件夹名称
}

// 删除群文件✔
#[endpoint("del_group_file")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DelGroupFile {
    pub group_id: i64,   // 群号
    pub file_id: String, // 文件 ID
}

// 删除群文件夹✔
#[endpoint("del_group_file_folder")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DelGroupFileFolder {
    pub group_id: i64,     // 群号
    pub folder_id: String, // 文件夹 ID
}

// NapCat gocq拓展

// 合并转发消息给群聊
#[endpoint("send_group_forward_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendGroupForwardMsg {
    pub group_id: i64,                 // 群号
    pub messages: Vec<MessageSegment>, // 要发送的消息内容
}

// 合并转发消息给好友
#[endpoint("send_private_forward_msg")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendPrivateForwardMsg {
    pub user_id: i64,                  // 对方 QQ 号
    pub messages: Vec<MessageSegment>, // 要发送的消息内容
}
