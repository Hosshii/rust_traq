/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// StampHistoryEntry : スタンプ履歴の１項目

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StampHistoryEntry {
    /// スタンプUUID
    #[serde(rename = "stampId")]
    pub stamp_id: std::path::PathBuf,
    /// 使用日時
    #[serde(rename = "datetime")]
    pub datetime: String,
}

impl StampHistoryEntry {
    /// スタンプ履歴の１項目
    pub fn new(stamp_id: std::path::PathBuf, datetime: String) -> StampHistoryEntry {
        StampHistoryEntry { stamp_id, datetime }
    }
}
