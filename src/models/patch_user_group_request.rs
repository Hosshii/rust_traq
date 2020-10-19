/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// PatchUserGroupRequest : ユーザーグループ編集リクエスト

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchUserGroupRequest {
    /// グループ名
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// グループ説明
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// グループタイプ
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl PatchUserGroupRequest {
    /// ユーザーグループ編集リクエスト
    pub fn new() -> PatchUserGroupRequest {
        PatchUserGroupRequest {
            name: None,
            description: None,
            _type: None,
        }
    }
}
