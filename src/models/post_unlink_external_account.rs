/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUnlinkExternalAccount : POST /users/me/ex-accounts/unlink 用リクエストボディ



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUnlinkExternalAccount {
    /// 外部サービス名
    #[serde(rename = "providerName")]
    pub provider_name: String,
}

impl PostUnlinkExternalAccount {
    /// POST /users/me/ex-accounts/unlink 用リクエストボディ
    pub fn new(provider_name: String) -> PostUnlinkExternalAccount {
        PostUnlinkExternalAccount {
            provider_name,
        }
    }
}


