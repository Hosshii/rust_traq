/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PinRemovedEvent : ピン削除イベント

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PinRemovedEvent {
    /// 変更者UUID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// メッセージUUID
    #[serde(rename = "messageId")]
    pub message_id: String,
}

impl PinRemovedEvent {
    /// ピン削除イベント
    pub fn new(user_id: String, message_id: String) -> PinRemovedEvent {
        PinRemovedEvent {
            user_id,
            message_id,
        }
    }
}
