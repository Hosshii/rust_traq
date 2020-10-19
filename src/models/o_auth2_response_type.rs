/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OAuth2ResponseType {
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "none")]
    None,

}



