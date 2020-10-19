/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2Decide {
    /// 承諾する場合は\"approve\"
    #[serde(rename = "submit")]
    pub submit: String,
}

impl OAuth2Decide {
    pub fn new(submit: String) -> OAuth2Decide {
        OAuth2Decide { submit }
    }
}
