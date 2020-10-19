/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DmChannel : ダイレクトメッセージチャンネル



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DmChannel {
    /// チャンネルUUID
    #[serde(rename = "id")]
    pub id: String,
    /// 送信先相手のUUID
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl DmChannel {
    /// ダイレクトメッセージチャンネル
    pub fn new(id: String, user_id: String) -> DmChannel {
        DmChannel {
            id,
            user_id,
        }
    }
}


