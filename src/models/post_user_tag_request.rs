/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostUserTagRequest : ユーザータグ追加リクエスト



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUserTagRequest {
    /// タグ文字列
    #[serde(rename = "tag")]
    pub tag: String,
}

impl PostUserTagRequest {
    /// ユーザータグ追加リクエスト
    pub fn new(tag: String) -> PostUserTagRequest {
        PostUserTagRequest {
            tag,
        }
    }
}


