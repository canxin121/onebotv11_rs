use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum MessageSegment {
    // 文本
    #[serde(rename = "text")]
    Text { data: TextData },
    // qq表情
    #[serde(rename = "face")]
    Face { data: FaceData },
    // qq商店表情
    #[serde(rename = "mface")]
    Mface { data: MfaceData },
    // @
    #[serde(rename = "at")]
    At { data: AtData },
    // 图片
    #[serde(rename = "image")]
    Image { data: ImageData },
    // 语音
    #[serde(rename = "record")]
    Record { data: RecordData },
    // 视频
    #[serde(rename = "video")]
    Video { data: VideoData },
    // 文件
    #[serde(rename = "file")]
    File { data: File },
    // 猜拳魔法表情
    #[serde(rename = "rps")]
    Rps {
        // 发: {}
        #[serde(default = "default_value")]
        data: Value,
    },
    // 掷骰子魔法表情
    #[serde(rename = "dice")]
    Dice {
        // 发: {}
        #[serde(default = "default_value")]
        data: Value,
    },
    // 窗口抖动（戳一戳）
    #[serde(rename = "shake")]
    Shake {
        // 发: {}
        #[serde(default = "default_value")]
        data: Value,
    },
    // 戳一戳
    #[serde(rename = "poke")]
    Poke { data: PokeData },
    // 匿名发消息
    #[serde(rename = "anonymous")]
    Anonymous { data: AnonymousData },
    // 分享
    #[serde(rename = "share")]
    Share { data: ShareData },
    // 推荐好友或群
    #[serde(rename = "contact")]
    Contact { data: ContactData },
    // 位置
    #[serde(rename = "location")]
    Location { data: LocationData },
    // 音乐分享
    #[serde(rename = "music")]
    Music { data: MusicData },
    // 自定义音乐分享
    #[serde(rename = "custom_music")]
    CustomMusic { data: CustomMusicData },
    // 回复
    #[serde(rename = "reply")]
    Reply { data: ReplyData },
    // 合并转发
    #[serde(rename = "forward")]
    Forward { data: ForwardData },
    // 合并转发节点
    #[serde(rename = "node")]
    Node { data: NodeData },
    // 合并转发自定义节点
    #[serde(rename = "node")]
    CustomNode { data: CustomNodeData },
    // xml消息
    #[serde(rename = "xml")]
    Xml { data: XmlData },
    // json消息
    #[serde(rename = "json")]
    Json { data: JsonData },
}

fn default_value() -> Value {
    json!({})
}

impl MessageSegment {
    // 文本信息
    pub fn text(text: impl Into<String>) -> Self {
        MessageSegment::Text {
            data: TextData { text: text.into() },
        }
    }
    // qq表情
    pub fn face(id: impl Into<String>) -> Self {
        MessageSegment::Face {
            data: FaceData { id: id.into() },
        }
    }
    pub fn mface(
        summary: impl Into<String>,
        url: impl Into<String>,
        emoji_id: impl Into<String>,
        emoji_package_id: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        MessageSegment::Mface {
            data: MfaceData {
                summary: summary.into(),
                url: url.into(),
                emoji_id: emoji_id.into(),
                emoji_package_id: emoji_package_id.into(),
                key: key.into(),
            },
        }
    }
    // @
    pub fn at(qq: impl Into<String>) -> Self {
        MessageSegment::At {
            data: AtData { qq: qq.into() },
        }
    }

    pub fn easy_image(file: impl Into<String>, summary: Option<impl Into<String>>) -> Self {
        MessageSegment::Image {
            data: ImageData {
                file: file.into(),
                r#type: None,
                url: None,
                cache: None,
                proxy: None,
                timeout: None,
                summary: summary.map(|s| s.into()),
            },
        }
    }

    pub fn image(
        file: impl Into<String>,
        summary: Option<impl Into<String>>,
        r#type: Option<impl Into<String>>,
        cache: Option<bool>,
        proxy: Option<bool>,
        timeout: Option<u32>,
    ) -> Self {
        MessageSegment::Image {
            data: ImageData {
                file: file.into(),
                r#type: r#type.map(|t| t.into()),
                url: None,
                cache: cache.map(|c| if c { 1 } else { 0 }),
                proxy: proxy.map(|p| if p { 1 } else { 0 }),
                timeout,
                summary: summary.map(|s| s.into()),
            },
        }
    }

    pub fn record(
        file: impl Into<String>,
        magic: Option<bool>,
        url: Option<impl Into<String>>,
        cache: Option<bool>,
        proxy: Option<bool>,
        timeout: Option<u32>,
    ) -> Self {
        MessageSegment::Record {
            data: RecordData {
                file: file.into(),
                magic: magic.map(|m| if m { 1 } else { 0 }),
                url: url.map(|u| u.into()),
                cache: cache.map(|c| if c { 1 } else { 0 }),
                proxy: proxy.map(|p| if p { 1 } else { 0 }),
                timeout,
            },
        }
    }

    pub fn video(
        file: impl Into<String>,
        url: Option<impl Into<String>>,
        cache: Option<bool>,
        proxy: Option<bool>,
        timeout: Option<u32>,
    ) -> Self {
        MessageSegment::Video {
            data: VideoData {
                file: file.into(),
                url: url.map(|u| u.into()),
                cache: cache.map(|c| if c { 1 } else { 0 }),
                proxy: proxy.map(|p| if p { 1 } else { 0 }),
                timeout,
            },
        }
    }

    pub fn file(file: impl Into<String>, name: Option<impl Into<String>>) -> Self {
        MessageSegment::File {
            data: File {
                file: file.into(),
                name: name.map(|n| n.into()),
                path: String::with_capacity(0),
                url: None,
                file_id: String::with_capacity(0),
                file_size: String::with_capacity(0),
            },
        }
    }

    pub fn poke(r#type: String, id: impl Into<String>) -> Self {
        MessageSegment::Poke {
            data: PokeData {
                r#type,
                id: id.into(),
            },
        }
    }

    pub fn anonymous(ignore: Option<bool>) -> Self {
        MessageSegment::Anonymous {
            data: AnonymousData {
                ignore: ignore.map(|i| if i { 1 } else { 0 }),
            },
        }
    }

    pub fn contact(r#type: ContactType, id: impl Into<String>) -> Self {
        MessageSegment::Contact {
            data: ContactData {
                r#type,
                id: id.into(),
            },
        }
    }

    pub fn music(r#type: String, id: impl Into<String>) -> Self {
        MessageSegment::Music {
            data: MusicData {
                r#type,
                id: id.into(),
            },
        }
    }

    pub fn music_custom(
        r#type: impl Into<String>,
        url: impl Into<String>,
        audio: impl Into<String>,
        title: impl Into<String>,
        content: Option<impl Into<String>>,
        image: Option<impl Into<String>>,
    ) -> Self {
        MessageSegment::CustomMusic {
            data: CustomMusicData {
                r#type: r#type.into(),
                url: url.into(),
                audio: audio.into(),
                title: title.into(),
                content: content.map(|c| c.into()),
                image: image.map(|i| i.into()),
            },
        }
    }

    pub fn reply(id: impl Into<String>) -> Self {
        MessageSegment::Reply {
            data: ReplyData { id: id.into() },
        }
    }

    pub fn forward(id: impl Into<String>) -> Self {
        MessageSegment::Forward {
            data: ForwardData { id: id.into() },
        }
    }

    pub fn node(id: impl Into<String>) -> Self {
        MessageSegment::Node {
            data: NodeData { id: id.into() },
        }
    }

    pub fn easy_custom_node(content: Vec<MessageSegment>) -> Self {
        MessageSegment::CustomNode {
            data: CustomNodeData {
                name: None,
                uin: None,
                content,
            },
        }
    }

    // llonebot/NapCat 无法自定义昵称和uin, Lagrange可以但是不能发送大于1mb的video
    pub fn custom_node(uin: i64, name: impl Into<String>, content: Vec<MessageSegment>) -> Self {
        MessageSegment::CustomNode {
            data: CustomNodeData {
                name: Some(name.into()),
                uin: Some(uin),
                content,
            },
        }
    }

    pub fn rps() -> Self {
        MessageSegment::Rps {
            data: default_value(),
        }
    }

    pub fn dice() -> Self {
        MessageSegment::Dice {
            data: default_value(),
        }
    }

    pub fn shake() -> Self {
        MessageSegment::Shake {
            data: default_value(),
        }
    }

    pub fn share(
        url: impl Into<String>,
        title: impl Into<String>,
        content: Option<impl Into<String>>,
        image: Option<impl Into<String>>,
    ) -> Self {
        MessageSegment::Share {
            data: ShareData {
                url: url.into(),
                title: title.into(),
                content: content.map(|c| c.into()),
                image: image.map(|i| i.into()),
            },
        }
    }

    pub fn location(
        lat: impl Into<String>,
        lon: impl Into<String>,
        title: Option<impl Into<String>>,
        content: Option<impl Into<String>>,
    ) -> Self {
        MessageSegment::Location {
            data: LocationData {
                lat: lat.into(),
                lon: lon.into(),
                title: title.map(|t| t.into()),
                content: content.map(|c| c.into()),
            },
        }
    }
}

// 以下未标志收发的，均为收发均可

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TextData {
    // 文本内容
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FaceData {
    // QQ 表情的 ID
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    // 图片文件名, 使用收到的图片文件名直接发送
    // 绝对路径，例如 file:///C:\\Users\Richard\Pictures\1.png，格式使用 file URI
    // 网络 URL，例如 http://i1.piimg.com/567571/fdd6e7b6d93f1ef0.jpg
    // Base64 编码，例如 base64://iVBORw0KGgoAAAANSUhEUgAAABQAAAAVCAIAAADJt1n/AAAAKElEQVQ4EWPk5+RmIBcwkasRpG9UM4mhNxpgowFGMARGEwnBIEJVAAAdBgBNAZf+QAAAAABJRU5ErkJggg==
    pub file: String,

    // 图片类型, flash 表示闪照, 无此参数表示普通图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    // 发: 自定义图片预览信息
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub summary: Option<String>,

    // 收
    // 图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    // 发
    // 可选0 1, 只在通过网络 URL 发送时有效, 表示是否使用已缓存的文件, 默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<u8>,

    // 发
    // 可选0 1, 只在通过网络 URL 发送时有效，表示是否通过代理下载文件（需通过环境变量或配置文件配置代理），默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<u8>,

    // 发
    // 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间，默认不超时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RecordData {
    // 语音文件名, 使用收到的语音文件名直接发送
    // 以下为来自图片消息段的参考
    // 绝对路径，例如 file:///C:\\Users\Richard\Pictures\1.png，格式使用 file URI
    // 网络 URL，例如 http://i1.piimg.com/567571/fdd6e7b6d93f1ef0.jpg
    // Base64 编码，例如 base64://iVBORw0KGgoAAAANSUhEUgAAABQAAAAVCAIAAADJt1n/AAAAKElEQVQ4EWPk5+RmIBcwkasRpG9UM4mhNxpgowFGMARGEwnBIEJVAAAdBgBNAZf+QAAAAABJRU5ErkJggg==
    pub file: String,

    // 发送时可选，默认 0，设置为 1 表示变声
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic: Option<u8>,

    // 收
    // 语音 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    // 发
    // 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<u8>,

    // 发
    // 只在通过网络 URL 发送时有效，表示是否通过代理下载文件（需通过环境变量或配置文件配置代理），默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<u8>,

    // 发
    // 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间 ，默认不超时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoData {
    // 视频文件名, 使用收到的视频文件名直接发送
    // 以下为来自图片消息段的参考
    // 绝对路径，例如 file:///C:\\Users\Richard\Pictures\1.png，格式使用 file URI
    // 网络 URL，例如 http://i1.piimg.com/567571/fdd6e7b6d93f1ef0.jpg
    // Base64 编码，例如 base64://iVBORw0KGgoAAAANSUhEUgAAABQAAAAVCAIAAADJt1n/AAAAKElEQVQ4EWPk5+RmIBcwkasRpG9UM4mhNxpgowFGMARGEwnBIEJVAAAdBgBNAZf+QAAAAABJRU5ErkJggg==
    pub file: String,

    // 收
    // 视频 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    // 发
    // 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<u8>,

    // 发
    // 只在通过网络 URL 发送时有效，表示是否通过代理下载文件（需通过环境变量或配置文件配置代理），默认 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<u8>,

    // 发
    // 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间 ，默认不超时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AtData {
    // @的 QQ 号，all 表示全体成员
    pub qq: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PokeData {
    // 类型，见 Mirai 的 PokeMessage 类
    pub r#type: String,
    // ID
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AnonymousData {
    // 可选，表示无法匿名时是否继续发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ShareData {
    // URL
    pub url: String,
    // 标题
    pub title: String,
    // 发送时可选，内容描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    // 发送时可选，图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ContactType {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "qq")]
    QQ,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ContactData {
    // group 或 qq
    pub r#type: ContactType,
    // 被推荐人的 QQ 号或被推荐群的群号
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LocationData {
    // 纬度
    pub lat: String,
    // 经度
    pub lon: String,
    // 发送时可选，标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    // 发送时可选，内容描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MusicData {
    // 分别表示使用 QQ 音乐、网易云音乐、虾米音乐
    pub r#type: String,
    // 歌曲 ID
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CustomMusicData {
    // 表示音乐自定义分享
    pub r#type: String,
    // 点击后跳转目标 URL
    pub url: String,
    // 音乐 URL
    pub audio: String,
    // 标题
    pub title: String,
    // 发送时可选，内容描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    // 发送时可选，图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReplyData {
    // 回复时引用的消息 ID
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForwardData {
    pub id: String,
}

// 合并转发节点
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NodeData {
    // 转发的消息 ID
    pub id: String,
}

// 自定义合并转发节点
// 非obv11定义，而是适用于lagrand/gocq/llonebot/NapCat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CustomNodeData {
    // 自定义昵称(已失效)
    pub name: Option<String>,
    // 自定义qq号(已失效)
    pub uin: Option<i64>,
    // 自定义内容
    pub content: Vec<MessageSegment>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct XmlData {
    // XML 内容
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct JsonData {
    // JSON 内容
    pub data: String,
}

// onebot_v11某些变体的实现，如llonebot/NapCat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MfaceData {
    pub summary: String,
    pub url: String,
    pub emoji_id: String,
    pub emoji_package_id: String,
    pub key: String,
}

// onebot_v11某些变体的实现，如llonebot/NapCat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct File {
    // 收: 文件名称 发： 文件路径
    pub file: String,
    // 发: 自定义文件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    // 收
    // 文件路径
    #[serde(skip_serializing)]
    pub path: String,
    // 收
    // 文件 URL
    #[serde(skip_serializing)]
    pub url: Option<String>,
    // 收
    // 文件 ID
    #[serde(skip_serializing)]
    pub file_id: String,
    // 收
    // 文件大小
    #[serde(skip_serializing)]
    pub file_size: String,
}
