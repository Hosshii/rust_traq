/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PostClipFolderRequest : クリップフォルダ作成リクエスト



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostClipFolderRequest {
    /// フォルダ名
    #[serde(rename = "name")]
    pub name: String,
    /// 説明
    #[serde(rename = "description")]
    pub description: String,
}

impl PostClipFolderRequest {
    /// クリップフォルダ作成リクエスト
    pub fn new(name: String, description: String) -> PostClipFolderRequest {
        PostClipFolderRequest {
            name,
            description,
        }
    }
}


