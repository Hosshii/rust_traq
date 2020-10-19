/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUserGroupAdminRequest : グループ管理者追加リクエスト



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUserGroupAdminRequest {
    /// 追加するユーザーのUUID
    #[serde(rename = "id")]
    pub id: String,
}

impl PostUserGroupAdminRequest {
    /// グループ管理者追加リクエスト
    pub fn new(id: String) -> PostUserGroupAdminRequest {
        PostUserGroupAdminRequest {
            id,
        }
    }
}


