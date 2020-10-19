/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PinAddedEvent : ピン追加イベント



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PinAddedEvent {
    /// 変更者UUID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// メッセージUUID
    #[serde(rename = "messageId")]
    pub message_id: String,
}

impl PinAddedEvent {
    /// ピン追加イベント
    pub fn new(user_id: String, message_id: String) -> PinAddedEvent {
        PinAddedEvent {
            user_id,
            message_id,
        }
    }
}

