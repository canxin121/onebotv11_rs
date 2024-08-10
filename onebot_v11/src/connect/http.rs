use crate::{
    api::{payload::ApiPayload, resp::ApiRespData},
    traits::UrlSuffix,
};
use reqwest::StatusCode;
use serde_json::Value;
use tracing::warn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16,
    pub access_token: Option<String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        HttpConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            access_token: None,
        }
    }
}

/// `status` 字段表示请求的状态：
/// - `ok` 表示操作成功, 同时 `retcode` （返回码）会等于 0
/// - `async` 表示请求已提交异步处理, 此时 `retcode` 为 1, 具体成功或失败将无法获知
/// - `failed` 表示操作失败, 此时 `retcode` 既不是 0 也不是 1, 具体错误信息应参考 OneBot 实现的日志
#[derive(Debug)]
pub struct HttpCallApiResp {
    pub status: String,
    pub retcode: String,
    pub data: ApiRespData,
}

#[derive(Debug)]
pub enum HttpCallApiError {
    Unauthorized,
    InvaildToken,
    ContentTypeNotSupported,
    InvalidRequestBody,
    ApiNotFound,
    UnKnown,
    AnyhowError(anyhow::Error),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for HttpCallApiError {
    fn from(err: reqwest::Error) -> Self {
        HttpCallApiError::ReqwestError(err)
    }
}
impl From<anyhow::Error> for HttpCallApiError {
    fn from(err: anyhow::Error) -> Self {
        HttpCallApiError::AnyhowError(err)
    }
}

impl From<StatusCode> for HttpCallApiError {
    fn from(value: StatusCode) -> Self {
        match value {
            StatusCode::UNAUTHORIZED => HttpCallApiError::Unauthorized,
            StatusCode::FORBIDDEN => HttpCallApiError::InvaildToken,
            StatusCode::NOT_ACCEPTABLE => HttpCallApiError::ContentTypeNotSupported,
            StatusCode::BAD_REQUEST => HttpCallApiError::InvalidRequestBody,
            StatusCode::NOT_FOUND => HttpCallApiError::ApiNotFound,
            _ => HttpCallApiError::UnKnown,
        }
    }
}

pub struct HttpConnect {
    pub config: HttpConfig,
    pub client: reqwest::Client,
}

impl HttpConnect {
    pub fn new(config: HttpConfig) -> Self {
        HttpConnect {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn call_api(
        &self,
        api_data: ApiPayload,
    ) -> Result<HttpCallApiResp, HttpCallApiError> {
        let resp_type = api_data.to_resp_type();
        let url = format!(
            "http://{}:{}/{}",
            self.config.host,
            self.config.port,
            api_data.url_suffix()
        );

        let response_builder = self.client.post(url).json(&api_data);
        let response = match &self.config.access_token {
            Some(token) => response_builder.bearer_auth(token).send().await?,
            _ => response_builder.send().await?,
        };
        let status = response.status();
        match status {
            StatusCode::OK => {
                let json: Value = response.json().await?;
                let status = json["status"].as_str().unwrap_or("failed").to_string();
                let retcode = json["retcode"].as_str().unwrap_or("0").to_string();
                if status == "failed" {
                    return Err(anyhow::anyhow!(
                        "http call api unknown error, status: 'failed', raw resp: {}",
                        json.to_string()
                    )
                    .into());
                }
                let data = match status.as_str() {
                    "ok" => match ApiRespData::from_resp_type(resp_type, json["data"].clone()) {
                        Ok(resp) => Ok(resp),
                        Err(e) => {
                            warn!(
                                "http call api failed, raw resp: {}, raw req: {}",
                                json,
                                serde_json::to_string(&api_data)
                                    .unwrap_or("Serialize Failed".to_string())
                            );
                            Err(e)
                        }
                    }?,
                    _ => ApiRespData::NoResponse(None),
                };
                Ok(HttpCallApiResp {
                    status,
                    retcode,
                    data,
                })
            }
            _ => {
                if let Ok(resp_str) = response.text().await {
                    warn!(
                        "http call api failed, raw resp: {}, raw req: {}",
                        resp_str,
                        serde_json::to_string(&api_data).unwrap_or("Serialize Failed".to_string())
                    );
                }
                return Err(HttpCallApiError::from(status));
            }
        }
    }
}
