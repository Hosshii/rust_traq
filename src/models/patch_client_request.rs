/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchClientRequest : OAuth2クライアント情報変更リクエスト



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchClientRequest {
    /// クライアント名
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 説明
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// コールバックURL
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// クライアント開発者UUID
    #[serde(rename = "developerId", skip_serializing_if = "Option::is_none")]
    pub developer_id: Option<String>,
}

impl PatchClientRequest {
    /// OAuth2クライアント情報変更リクエスト
    pub fn new() -> PatchClientRequest {
        PatchClientRequest {
            name: None,
            description: None,
            callback_url: None,
            developer_id: None,
        }
    }
}

