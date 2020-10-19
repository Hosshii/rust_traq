/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Ogp : OGPの情報



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ogp {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::OgpMedia>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<crate::models::OgpMedia>>,
}

impl Ogp {
    /// OGPの情報
    pub fn new() -> Ogp {
        Ogp {
            _type: None,
            title: None,
            url: None,
            images: None,
            description: None,
            videos: None,
        }
    }
}


