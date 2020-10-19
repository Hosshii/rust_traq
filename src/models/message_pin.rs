/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// MessagePin : ピン情報

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagePin {
    /// ピン留めしたユーザーUUID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// ピン留めされた日時
    #[serde(rename = "pinnedAt")]
    pub pinned_at: String,
}

impl MessagePin {
    /// ピン情報
    pub fn new(user_id: String, pinned_at: String) -> MessagePin {
        MessagePin { user_id, pinned_at }
    }
}
