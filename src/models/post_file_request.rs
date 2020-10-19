/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostFileRequest : ファイルアップロードリクエスト



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostFileRequest {
    /// ファイル本体
    #[serde(rename = "file")]
    pub file: std::path::PathBuf,
    /// アップロード先チャンネルUUID
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

impl PostFileRequest {
    /// ファイルアップロードリクエスト
    pub fn new(file: std::path::PathBuf, channel_id: String) -> PostFileRequest {
        PostFileRequest {
            file,
            channel_id,
        }
    }
}


