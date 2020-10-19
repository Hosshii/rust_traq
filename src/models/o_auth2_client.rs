/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// OAuth2Client : OAuth2クライアント情報



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2Client {
    /// クライアントUUID
    #[serde(rename = "id")]
    pub id: String,
    /// クライアント名
    #[serde(rename = "name")]
    pub name: String,
    /// 説明
    #[serde(rename = "description")]
    pub description: String,
    /// クライアント開発者UUID
    #[serde(rename = "developerId")]
    pub developer_id: String,
    /// 要求スコープの配列
    #[serde(rename = "scopes")]
    pub scopes: Vec<crate::models::OAuth2Scope>,
}

impl OAuth2Client {
    /// OAuth2クライアント情報
    pub fn new(id: String, name: String, description: String, developer_id: String, scopes: Vec<crate::models::OAuth2Scope>) -> OAuth2Client {
        OAuth2Client {
            id,
            name,
            description,
            developer_id,
            scopes,
        }
    }
}

