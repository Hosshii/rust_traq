/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// WebRtcUserState : WebRTC状態

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebRtcUserState {
    /// ユーザーUUID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// チャンネルUUID
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// セッションの配列
    #[serde(rename = "sessions")]
    pub sessions: Vec<crate::models::WebRtcUserStateSessions>,
}

impl WebRtcUserState {
    /// WebRTC状態
    pub fn new(
        user_id: String,
        channel_id: String,
        sessions: Vec<crate::models::WebRtcUserStateSessions>,
    ) -> WebRtcUserState {
        WebRtcUserState {
            user_id,
            channel_id,
            sessions,
        }
    }
}
