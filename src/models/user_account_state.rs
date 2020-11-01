/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde_repr::{Deserialize_repr, Serialize_repr};

/// UserAccountState : ユーザーアカウント状態 0: 停止 1: 有効 2: 一時停止

/// ユーザーアカウント状態 0: 停止 1: 有効 2: 一時停止
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize_repr, Serialize_repr,
)]
#[repr(u8)]
pub enum UserAccountState {
    #[serde(rename = "0")]
    deactivated = 0,
    #[serde(rename = "1")]
    active = 1,
    #[serde(rename = "2")]
    suspended = 2,
}
