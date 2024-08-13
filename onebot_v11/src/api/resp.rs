use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::message::segment::MessageSegment;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiResp {
    pub status: String,
    pub retcode: u32,
    pub data: ApiRespData,
    pub echo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiRespBuilder {
    pub status: String,
    pub retcode: u32,
    // ApiRespData，但是无法直接序列化，提供一个type_id
    pub data: Value,
    pub echo: Option<String>,
}
impl ApiRespBuilder {
    pub fn build(self, resp_type: u8) -> Result<ApiResp, anyhow::Error> {
        let data = ApiRespData::from_resp_type(resp_type, self.data)?;
        Ok(ApiResp {
            status: self.status,
            retcode: self.retcode,
            data,
            echo: self.echo,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiRespData {
    SendPrivateMsgResponse(SendPrivateMsgResponse),
    SendGroupMsgResponse(SendGroupMsgResponse),
    SendMsgResponse(SendMsgResponse),
    DeleteMsgResponse(DeleteMsgResponse),
    GetMsgResponse(GetMsgResponse),
    GetForwardMsgResponse(GetForwardMsgResponse),
    GetLoginInfoResponse(GetLoginInfoResponse),
    GetStrangerInfoResponse(GetStrangerInfoResponse),
    GetFriendListResponse(Vec<GetFriendListResponseItem>),
    GetGroupInfoResponse(GetGroupInfoResponse),
    GetGroupListResponse(Vec<GetGroupListResponseItem>),
    GetGroupMemberInfoResponse(GetGroupMemberInfoResponse),
    GetGroupMemberListResponse(Vec<GetGroupMemberListResponseItem>),
    GetGroupHonorInfoResponse(GetGroupHonorInfoResponse),
    GetCookiesResponse(GetCookiesResponse),
    GetCsrfTokenResponse(GetCsrfTokenResponse),
    GetCredentialsResponse(GetCredentialsResponse),
    GetRecordResponse(GetRecordResponse),
    GetImageResponse(GetImageResponse),
    CanSendImageResponse(CanSendImageResponse),
    CanSendRecordResponse(CanSendRecordResponse),
    GetStatusResponse(GetStatusResponse),
    GetVersionInfoResponse(GetVersionInfoResponse),
    NoResponse(Option<()>),
    // NapCat/llonebot扩展

    // 获取群系统通知
    GetGroupSystemMsgResponse(GetGroupSystemMsgResponse),
    // 下载群文件或私聊文件
    GetFileResponse(GetFileResponse),
    // 获取好友分类列表
    GetFriendsWithCategoryResponse(Vec<GetFriendsWithCategoryResponseItem>),
    // 获取机器人QQ号范围
    GetRobotUinRangeResponse(Vec<GetRobotUinRangeResponseItem>),
    // 获取群文件数量
    GetGroupFileCountResponse(GetGroupFileCountResponse),
    // 获取群文件列表
    GetGroupFileListResponse(GetGroupFileListResponse),
    // 创建群文件夹
    SetGroupFileFolderResponse(SetGroupFileFolderResponse),
    // 删除群文件
    DelGroupFileResponse(DelGroupFileResponse),
    // 删除群文件夹
    DelGroupFileFolderResponse(CommonClientResponseResult),

    // NapCat Gocq 扩展

    // 向群发送合并转发消息
    SendGroupForwardMsgResponse(SendGroupForwardMsgResponse),
    // 向私聊发送合并转发消息
    SendPrivateForwardMsgResponse(SendPrivateForwardMsgResponse),
}

impl Serialize for ApiRespData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ApiRespData::SendPrivateMsgResponse(data) => data.serialize(serializer),
            ApiRespData::SendGroupMsgResponse(data) => data.serialize(serializer),
            ApiRespData::SendMsgResponse(data) => data.serialize(serializer),
            ApiRespData::DeleteMsgResponse(data) => data.serialize(serializer),
            ApiRespData::GetMsgResponse(data) => data.serialize(serializer),
            ApiRespData::GetForwardMsgResponse(data) => data.serialize(serializer),
            ApiRespData::GetLoginInfoResponse(data) => data.serialize(serializer),
            ApiRespData::GetStrangerInfoResponse(data) => data.serialize(serializer),
            ApiRespData::GetFriendListResponse(data) => data.serialize(serializer),
            ApiRespData::GetGroupInfoResponse(data) => data.serialize(serializer),
            ApiRespData::GetGroupListResponse(data) => data.serialize(serializer),
            ApiRespData::GetGroupMemberInfoResponse(data) => data.serialize(serializer),
            ApiRespData::GetGroupMemberListResponse(data) => data.serialize(serializer),
            ApiRespData::GetGroupHonorInfoResponse(data) => data.serialize(serializer),
            ApiRespData::GetCookiesResponse(data) => data.serialize(serializer),
            ApiRespData::GetCsrfTokenResponse(data) => data.serialize(serializer),
            ApiRespData::GetCredentialsResponse(data) => data.serialize(serializer),
            ApiRespData::GetRecordResponse(data) => data.serialize(serializer),
            ApiRespData::GetImageResponse(data) => data.serialize(serializer),
            ApiRespData::CanSendImageResponse(data) => data.serialize(serializer),
            ApiRespData::CanSendRecordResponse(data) => data.serialize(serializer),
            ApiRespData::GetStatusResponse(data) => data.serialize(serializer),
            ApiRespData::GetVersionInfoResponse(data) => data.serialize(serializer),
            ApiRespData::NoResponse(data) => data.serialize(serializer),
            // NapCat/llonebot扩展
            ApiRespData::GetGroupSystemMsgResponse(data) => data.serialize(serializer),
            ApiRespData::GetFileResponse(data) => data.serialize(serializer),
            ApiRespData::GetFriendsWithCategoryResponse(data) => data.serialize(serializer),
            ApiRespData::GetGroupFileListResponse(data) => data.serialize(serializer),
            ApiRespData::SetGroupFileFolderResponse(data) => data.serialize(serializer),
            ApiRespData::DelGroupFileResponse(data) => data.serialize(serializer),
            ApiRespData::DelGroupFileFolderResponse(data) => data.serialize(serializer),
            ApiRespData::GetRobotUinRangeResponse(data) => data.serialize(serializer),
            ApiRespData::GetGroupFileCountResponse(data) => data.serialize(serializer),
            // NapCat Gocq 扩展
            ApiRespData::SendGroupForwardMsgResponse(data) => data.serialize(serializer),
            ApiRespData::SendPrivateForwardMsgResponse(data) => data.serialize(serializer),
        }
    }
}

/// 结构体表示发送私聊消息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendPrivateMsgResponse {
    /// 消息 ID
    pub message_id: i64,
}

/// 结构体表示发送群消息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendGroupMsgResponse {
    /// 消息 ID
    pub message_id: i64,
}

/// 结构体表示发送消息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendMsgResponse {
    /// 消息 ID
    pub message_id: i64,
}

/// 结构体表示撤回消息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeleteMsgResponse;

/// 结构体表示获取消息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMsgResponse {
    /// 发送时间
    pub time: i64,
    /// 消息类型
    pub message_type: String,
    /// 消息 ID
    pub message_id: i64,
    /// 消息真实 ID
    pub real_id: i64,
    /// 发送人信息
    pub sender: MessageSender,
    /// 消息内容
    pub message: Vec<MessageSegment>,
}

/// 结构体表示消息发送者的公共信息
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageSender {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 发送者 QQ 号
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 昵称
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 群名片／备注
    pub card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 性别
    pub sex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 年龄
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 地区
    pub area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 成员等级
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 角色
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 专属头衔
    pub title: Option<String>,
}

/// 结构体表示获取合并转发消息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetForwardMsgResponse {
    /// 消息内容
    pub message: Vec<MessageSegment>,
}

/// 结构体表示获取登录号信息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetLoginInfoResponse {
    /// QQ 号
    pub user_id: i64,
    /// QQ 昵称
    pub nickname: String,
}

/// 结构体表示获取陌生人信息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetStrangerInfoResponse {
    /// QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 性别
    pub sex: String,
    /// 年龄
    pub age: i64,
}

/// 结构体表示获取好友列表的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendListResponseItem {
    /// QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 备注名
    pub remark: String,
}

/// 结构体表示获取群信息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupInfoResponse {
    /// 群号
    pub group_id: i64,
    /// 群名称
    pub group_name: String,
    /// 成员数
    pub member_count: i64,
    /// 最大成员数（群容量）
    pub max_member_count: i64,
}

/// 结构体表示获取群列表的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupListResponseItem {
    /// 群号
    pub group_id: i64,
    /// 群名称
    pub group_name: String,
    /// 成员数
    pub member_count: i64,
    /// 最大成员数（群容量）
    pub max_member_count: i64,
}

/// 结构体表示获取群成员信息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupMemberInfoResponse {
    /// 群号
    pub group_id: i64,
    /// QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 群名片／备注
    pub card: String,
    /// 性别
    pub sex: String,
    /// 年龄
    pub age: i64,
    /// 地区
    pub area: String,
    /// 加群时间戳
    pub join_time: i64,
    /// 最后发言时间戳
    pub last_sent_time: i64,
    /// 成员等级
    pub level: String,
    /// 角色
    pub role: String,
    /// 是否不良记录成员
    pub unfriendly: bool,
    /// 专属头衔
    pub title: String,
    /// 专属头衔过期时间戳
    pub title_expire_time: i64,
    /// 是否允许修改群名片
    pub card_changeable: bool,
}

/// 结构体表示获取群成员列表的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupMemberListResponseItem {
    /// 群号
    pub group_id: i64,
    /// QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 群名片／备注
    pub card: String,
    /// 性别
    pub sex: String,
    /// 年龄
    pub age: i64,
    /// 加群时间戳
    pub join_time: i64,
    /// 最后发言时间戳
    pub last_sent_time: i64,
    /// 成员等级
    pub level: String,
    /// 角色
    pub role: String,
    /// 是否不良记录成员
    pub unfriendly: bool,
    /// 专属头衔
    pub title: String,
    /// 专属头衔过期时间戳
    pub title_expire_time: i64,
    /// 是否允许修改群名片
    pub card_changeable: bool,
}

/// 结构体表示获取群荣誉信息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupHonorInfoResponse {
    /// 群号
    pub group_id: i64,
    /// 当前龙王信息
    pub current_talkative: TalkativeInfo,
    /// 历史龙王列表
    pub talkative_list: Vec<HonorInfo>,
    /// 群聊之火列表
    pub performer_list: Vec<HonorInfo>,
    /// 群聊炽焰列表
    pub legend_list: Vec<HonorInfo>,
    /// 冒尖小春笋列表
    pub strong_newbie_list: Vec<HonorInfo>,
    /// 快乐之源列表
    pub emotion_list: Vec<HonorInfo>,
}

/// 结构体表示当前龙王的信息
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TalkativeInfo {
    /// QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 头像 URL
    pub avatar: String,
    /// 持续天数
    pub day_count: i64,
}

/// 结构体表示群荣誉信息
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct HonorInfo {
    /// QQ 号
    pub user_id: i64,
    /// 昵称
    pub nickname: String,
    /// 头像 URL
    pub avatar: String,
    /// 荣誉描述
    pub description: String,
}

/// 结构体表示获取 Cookies 的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetCookiesResponse {
    /// Cookies
    pub cookies: String,
}

/// 结构体表示获取 CSRF Token 的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetCsrfTokenResponse {
    /// CSRF Token
    pub token: i64,
}

/// 结构体表示获取 QQ 相关接口凭证的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetCredentialsResponse {
    /// Cookies
    pub cookies: String,
    /// CSRF Token
    pub csrf_token: i64,
}

/// 结构体表示获取语音记录的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetRecordResponse {
    /// 语音文件路径
    pub file: String,
}

/// 结构体表示获取图片的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetImageResponse {
    /// 图片文件路径
    pub file: String,
}

/// 结构体表示检查是否可以发送图片的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CanSendImageResponse {
    /// 是否可以发送图片
    pub yes: bool,
}

/// 结构体表示检查是否可以发送语音的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CanSendRecordResponse {
    /// 是否可以发送语音
    pub yes: bool,
}

/// 结构体表示获取运行状态的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetStatusResponse {
    /// 当前 QQ 是否在线
    pub online: Option<bool>,
    /// 状态是否正常
    pub good: bool,
}

/// 结构体表示获取版本信息的响应
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetVersionInfoResponse {
    /// 应用标识
    pub app_name: String,
    /// 应用版本
    pub app_version: String,
    /// OneBot 标准版本
    pub protocol_version: String,
}

// NapCat/llonebot扩展

// 获取群系统通知
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupSystemMsgResponse {
    // 入群邀请
    #[serde(rename = "InvitedRequest")]
    pub invited_requests: Vec<InvitedRequest>,
    // 被过滤的加群申请
    pub join_requests: Vec<JoinRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InvitedRequest {
    // 请求id
    pub request_id: String,
    // 邀请者uin
    pub invitor_uin: i64,
    // 邀请者昵称
    pub invitor_nick: String,
    // 群号
    pub group_id: i64,
    // 群名称
    pub group_name: String,
    // 是否处理
    pub checked: bool,
    // 0: 未处理 other: 处理者qq
    pub actor: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct JoinRequest {
    // 请求id
    pub request_id: String,
    // 请求者uin
    pub requester_uin: i64,
    // 请求者昵称
    pub requester_nick: String,
    // 群号
    pub group_id: i64,
    // 群名称
    pub group_name: String,
    // 是否处理
    pub checked: bool,
    // 0: 未处理 other: 处理者qq
    pub actor: i64,
}

// 下载群文件或私聊文件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFileResponse {
    pub file: String,
    pub file_name: String,
    pub file_size: u64,
    pub base64: String,
}

// 获取好友分类列表
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFriendsWithCategoryResponseItem {
    pub user_id: i64,
    pub nickname: String,
    pub remark: Option<String>,
    pub sex: Option<Sex>,
    pub level: Option<i64>,
    pub age: Option<i64>,
    pub qid: Option<String>,
    pub login_days: Option<i64>,
    pub categroy_name: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Sex {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "unknown")]
    Unknown,
}

// 获取机器人QQ号范围
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetRobotUinRangeResponseItem {
    #[serde(rename = "maxUin")]
    pub max_uin: String,
    #[serde(rename = "minUin")]
    pub min_uin: String,
}

// 获取群文件数量
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupFileCountResponse {
    pub count: u64, // 文件数量
}

// 获取群文件列表
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGroupFileListResponse {
    #[serde(rename = "FileList")]
    pub file_list: Vec<GroupFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupFile {
    #[serde(rename = "fileInfo")]
    pub file_info: Option<FileInfo>,
    #[serde(rename = "folderInfo")]
    pub folder_info: Option<FolderInfo>,
    #[serde(rename = "peerId")]
    pub peer_id: String,
    pub r#type: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FileInfo {
    #[serde(rename = "busId")]
    pub bus_id: i64,
    #[serde(rename = "deadTime")]
    pub dead_time: i64,
    #[serde(rename = "downloadTimes")]
    pub download_times: i64,
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "fileModelId")]
    pub file_model_id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: String,
    #[serde(rename = "isFolder")]
    pub is_folder: bool,
    #[serde(rename = "localPath")]
    pub local_path: String,
    #[serde(rename = "md5")]
    pub md5: String,
    #[serde(rename = "modifyTime")]
    pub modify_time: i64,
    #[serde(rename = "parentFolderId")]
    pub parent_folder_id: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "sha3")]
    pub sha3: String,
    #[serde(rename = "transStatus")]
    pub trans_status: i64,
    #[serde(rename = "transType")]
    pub strans_type: i64,
    #[serde(rename = "uploadTime")]
    pub upload_time: i64,
    #[serde(rename = "uploadedSize")]
    pub uploaded_size: String,
    #[serde(rename = "uploaderLocalPath")]
    pub uploader_local_path: String,
    #[serde(rename = "uploaderName")]
    pub uploader_name: String,
    #[serde(rename = "uploaderUin")]
    pub uploader_uin: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FolderInfo {
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "createUin")]
    pub create_uin: String,
    #[serde(rename = "creatorName")]
    pub creator_name: String,
    #[serde(rename = "folderId")]
    pub folder_id: String,
    #[serde(rename = "folderName")]
    pub folder_name: String,
    #[serde(rename = "modifyName")]
    pub modify_name: String,
    #[serde(rename = "modifyTime")]
    pub modify_time: i64,
    #[serde(rename = "modifyUin")]
    pub modify_uin: String,
    #[serde(rename = "parentFolderId")]
    pub parent_folder_id: String,
    #[serde(rename = "totalFileCount")]
    pub total_file_count: i64,
    #[serde(rename = "usedSpace")]
    pub used_space: String,
}

// 创建群文件夹
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGroupFileFolderResponse {
    #[serde(rename = "groupItem")]
    pub group_item: GroupFile,
    pub result: CommonClientResponseResult,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CommonClientResponseResult {
    #[serde(rename = "clientWording")]
    pub client_wording: String,
    #[serde(rename = "retCode")]
    pub ret_code: i64,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
}

// 删除群文件
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DelGroupFileResponse {
    #[serde(rename = "errMsg")]
    pub error_msg: String,
    pub result: i64,
    #[serde(rename = "transGroupFileResult")]
    pub trans_group_file_result: TransGroupFileResult,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TransGroupFileResult {
    #[serde(rename = "failFileIdList")]
    pub fail_file_id_list: Vec<String>,
    pub result: CommonClientResponseResult,
    #[serde(rename = "successFileIdList")]
    pub success_file_id_list: Vec<String>,
}

// NapCat Gocq 扩展

// 向群发送合并转发消息
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendGroupForwardMsgResponse {
    pub message_id: i64,
}

// 向私聊发送合并转发消息
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendPrivateForwardMsgResponse {
    pub message_id: i64,
}
