/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutMyPasswordRequest : パスワード変更リクエスト



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutMyPasswordRequest {
    /// 現在のパスワード
    #[serde(rename = "password")]
    pub password: String,
    /// 新しいパスワード
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

impl PutMyPasswordRequest {
    /// パスワード変更リクエスト
    pub fn new(password: String, new_password: String) -> PutMyPasswordRequest {
        PutMyPasswordRequest {
            password,
            new_password,
        }
    }
}

