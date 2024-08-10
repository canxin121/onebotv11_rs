use serde_json::Value;

use super::{payload::ApiPayload, resp::ApiRespData};

impl ApiPayload {
    pub fn to_resp_type(&self) -> u8 {
        match self {
            ApiPayload::SendPrivateMsg(_) => 1,
            ApiPayload::SendGroupMsg(_) => 2,
            ApiPayload::SendMsg(_) => 3,
            ApiPayload::DeleteMsg(_) => 4,
            ApiPayload::GetMsg(_) => 5,
            ApiPayload::GetForwardMsg(_) => 6,
            ApiPayload::SendLike(_) => 7,
            ApiPayload::SetGroupKick(_) => 8,
            ApiPayload::SetGroupBan(_) => 9,
            ApiPayload::SetGroupAnonymousBan(_) => 10,
            ApiPayload::SetGroupWholeBan(_) => 11,
            ApiPayload::SetGroupAdmin(_) => 12,
            ApiPayload::SetGroupAnonymous(_) => 13,
            ApiPayload::SetGroupCard(_) => 14,
            ApiPayload::SetGroupName(_) => 15,
            ApiPayload::SetGroupLeave(_) => 16,
            ApiPayload::SetGroupSpecialTitle(_) => 17,
            ApiPayload::SetFriendAddRequest(_) => 18,
            ApiPayload::SetGroupAddRequest(_) => 19,
            ApiPayload::GetLoginInfo(_) => 20,
            ApiPayload::GetStrangerInfo(_) => 21,
            ApiPayload::GetFriendList(_) => 22,
            ApiPayload::GetGroupInfo(_) => 23,
            ApiPayload::GetGroupList(_) => 24,
            ApiPayload::GetGroupMemberInfo(_) => 25,
            ApiPayload::GetGroupMemberList(_) => 26,
            ApiPayload::GetGroupHonorInfo(_) => 27,
            ApiPayload::GetCookies(_) => 28,
            ApiPayload::GetCsrfToken(_) => 29,
            ApiPayload::GetCredentials(_) => 30,
            ApiPayload::GetRecord(_) => 31,
            ApiPayload::GetImage(_) => 32,
            ApiPayload::CanSendImage(_) => 33,
            ApiPayload::CanSendRecord(_) => 34,
            ApiPayload::GetStatus(_) => 35,
            ApiPayload::GetVersionInfo(_) => 36,
            ApiPayload::SetRestart(_) => 37,
            ApiPayload::CleanCache(_) => 38,
            ApiPayload::SetQQAvatar(_) => 39,
            ApiPayload::GetGroupSystemMsg(_) => 40,
            ApiPayload::GetFile(_) => 41,
            ApiPayload::ForwardFriendSingleMsg(_) => 42,
            ApiPayload::ForwardGroupSingleMsg(_) => 43,
            ApiPayload::SetMsgEmojiLike(_) => 44,
            ApiPayload::MarkPrivateMsgAsRead(_) => 45,
            ApiPayload::MarkGroupMsgAsRead(_) => 46,
            ApiPayload::GetRobotUinRange(_) => 47,
            ApiPayload::SetOnlineStatus(_) => 48,
            ApiPayload::GetFriendsWithCategory(_) => 49,
            ApiPayload::GetGroupFileCount(_) => 50,
            ApiPayload::GetGroupFileList(_) => 51,
            ApiPayload::SetGroupFileFolder(_) => 52,
            ApiPayload::DelGroupFile(_) => 53,
            ApiPayload::DelGroupFileFolder(_) => 54,
        }
    }
}

impl ApiRespData {
    pub fn from_resp_type(resp_type: u8, data: Value) -> Result<ApiRespData, anyhow::Error> {
        match resp_type {
            1 => Ok(ApiRespData::SendPrivateMsgResponse(serde_json::from_value(
                data,
            )?)),
            2 => Ok(ApiRespData::SendGroupMsgResponse(serde_json::from_value(
                data,
            )?)),
            3 => Ok(ApiRespData::SendMsgResponse(serde_json::from_value(data)?)),
            4 => Ok(ApiRespData::DeleteMsgResponse(serde_json::from_value(
                data,
            )?)),
            5 => Ok(ApiRespData::GetMsgResponse(serde_json::from_value(data)?)),
            6 => Ok(ApiRespData::GetForwardMsgResponse(serde_json::from_value(
                data,
            )?)),
            20 => Ok(ApiRespData::GetLoginInfoResponse(serde_json::from_value(
                data,
            )?)),
            21 => Ok(ApiRespData::GetStrangerInfoResponse(
                serde_json::from_value(data)?,
            )),
            22 => Ok(ApiRespData::GetFriendListResponse(serde_json::from_value(
                data,
            )?)),
            23 => Ok(ApiRespData::GetGroupInfoResponse(serde_json::from_value(
                data,
            )?)),
            24 => Ok(ApiRespData::GetGroupListResponse(serde_json::from_value(
                data,
            )?)),
            25 => Ok(ApiRespData::GetGroupMemberInfoResponse(
                serde_json::from_value(data)?,
            )),
            26 => Ok(ApiRespData::GetGroupMemberListResponse(
                serde_json::from_value(data)?,
            )),
            27 => Ok(ApiRespData::GetGroupHonorInfoResponse(
                serde_json::from_value(data)?,
            )),
            28 => Ok(ApiRespData::GetCookiesResponse(serde_json::from_value(
                data,
            )?)),
            29 => Ok(ApiRespData::GetCsrfTokenResponse(serde_json::from_value(
                data,
            )?)),
            30 => Ok(ApiRespData::GetCredentialsResponse(serde_json::from_value(
                data,
            )?)),
            31 => Ok(ApiRespData::GetRecordResponse(serde_json::from_value(
                data,
            )?)),
            32 => Ok(ApiRespData::GetImageResponse(serde_json::from_value(data)?)),
            33 => Ok(ApiRespData::CanSendImageResponse(serde_json::from_value(
                data,
            )?)),
            34 => Ok(ApiRespData::CanSendRecordResponse(serde_json::from_value(
                data,
            )?)),
            35 => Ok(ApiRespData::GetStatusResponse(serde_json::from_value(
                data,
            )?)),
            36 => Ok(ApiRespData::GetVersionInfoResponse(serde_json::from_value(
                data,
            )?)),
            40 => Ok(ApiRespData::GetGroupSystemMsgResponse(
                serde_json::from_value(data)?,
            )),
            41 => Ok(ApiRespData::GetFileResponse(serde_json::from_value(data)?)),
            47 => Ok(ApiRespData::GetRobotUinRangeResponse(
                serde_json::from_value(data)?,
            )),
            48 => Ok(ApiRespData::NoResponse(
                serde_json::from_value(data)?,
            )),
            49 => Ok(ApiRespData::GetFriendsWithCategoryResponse(
                serde_json::from_value(data)?,
            )),
            50 => Ok(ApiRespData::GetGroupFileCountResponse(
                serde_json::from_value(data)?,
            )),
            51 => Ok(ApiRespData::GetGroupFileListResponse(
                serde_json::from_value(data)?,
            )),
            52 => Ok(ApiRespData::SetGroupFileFolderResponse(
                serde_json::from_value(data)?,
            )),
            53 => Ok(ApiRespData::DelGroupFileResponse(serde_json::from_value(
                data,
            )?)),
            54 => Ok(ApiRespData::DelGroupFileFolderResponse(
                serde_json::from_value(data)?,
            )),
            _ => Ok(ApiRespData::NoResponse(Some(()))),
        }
    }
}
