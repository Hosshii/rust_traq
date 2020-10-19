/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserAccountState : ユーザーアカウント状態 0: 停止 1: 有効 2: 一時停止

/// ユーザーアカウント状態 0: 停止 1: 有効 2: 一時停止
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserAccountState {
    #[serde(rename = "0")]
    deactivated,
    #[serde(rename = "1")]
    active,
    #[serde(rename = "2")]
    suspended,

}




