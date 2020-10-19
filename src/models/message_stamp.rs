/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MessageStamp : メッセージに押されたスタンプ



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageStamp {
    /// ユーザーUUID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// スタンプUUID
    #[serde(rename = "stampId")]
    pub stamp_id: String,
    /// スタンプ数
    #[serde(rename = "count")]
    pub count: i32,
    /// スタンプが最初に押された日時
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// スタンプが最後に押された日時
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl MessageStamp {
    /// メッセージに押されたスタンプ
    pub fn new(user_id: String, stamp_id: String, count: i32, created_at: String, updated_at: String) -> MessageStamp {
        MessageStamp {
            user_id,
            stamp_id,
            count,
            created_at,
            updated_at,
        }
    }
}


