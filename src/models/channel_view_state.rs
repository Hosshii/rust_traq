/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChannelViewState : 閲覧状態

/// 閲覧状態
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelViewState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "monitoring")]
    Monitoring,
    #[serde(rename = "editing")]
    Editing,
}
