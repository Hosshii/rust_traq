/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FileInfoThumbnail : サムネイル情報 サムネイルが存在しない場合はnullになります

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileInfoThumbnail {
    /// MIMEタイプ
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    /// サムネイル幅
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// サムネイル高さ
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

impl FileInfoThumbnail {
    /// サムネイル情報 サムネイルが存在しない場合はnullになります
    pub fn new() -> FileInfoThumbnail {
        FileInfoThumbnail {
            mime: None,
            width: None,
            height: None,
        }
    }
}
