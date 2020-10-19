/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MessageClip : メッセージクリップ



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageClip {
    /// クリップされているフォルダのID
    #[serde(rename = "folderId")]
    pub folder_id: String,
    /// クリップされた日時
    #[serde(rename = "clippedAt")]
    pub clipped_at: String,
}

impl MessageClip {
    /// メッセージクリップ
    pub fn new(folder_id: String, clipped_at: String) -> MessageClip {
        MessageClip {
            folder_id,
            clipped_at,
        }
    }
}


