/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// BotEventLog : BOTイベントログ

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotEventLog {
    /// BOT UUID
    #[serde(rename = "botId")]
    pub bot_id: String,
    /// リクエストUUID
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// イベントタイプ
    #[serde(rename = "event")]
    pub event: String,
    /// ステータスコード
    #[serde(rename = "code")]
    pub code: i32,
    /// イベント日時
    #[serde(rename = "datetime")]
    pub datetime: String,
}

impl BotEventLog {
    /// BOTイベントログ
    pub fn new(
        bot_id: String,
        request_id: String,
        event: String,
        code: i32,
        datetime: String,
    ) -> BotEventLog {
        BotEventLog {
            bot_id,
            request_id,
            event,
            code,
            datetime,
        }
    }
}
