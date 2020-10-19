/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PutUserIconRequest : アイコン画像変更リクエスト



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutUserIconRequest {
    /// アイコン画像(1MBまでのpng, jpeg, gif)
    #[serde(rename = "file")]
    pub file: std::path::PathBuf,
}

impl PutUserIconRequest {
    /// アイコン画像変更リクエスト
    pub fn new(file: std::path::PathBuf) -> PutUserIconRequest {
        PutUserIconRequest {
            file,
        }
    }
}


