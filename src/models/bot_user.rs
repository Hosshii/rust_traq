/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// BotUser : BOTユーザー対

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotUser {
    /// BOT UUID
    #[serde(rename = "id")]
    pub id: String,
    /// BOTユーザーUUID
    #[serde(rename = "botUserId")]
    pub bot_user_id: String,
}

impl BotUser {
    /// BOTユーザー対
    pub fn new(id: String, bot_user_id: String) -> BotUser {
        BotUser { id, bot_user_id }
    }
}
