/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Stamp : スタンプ情報

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stamp {
    /// スタンプUUID
    #[serde(rename = "id")]
    pub id: String,
    /// スタンプ名
    #[serde(rename = "name")]
    pub name: String,
    /// 作成者UUID
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    /// 作成日時
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// 更新日時
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// ファイルUUID
    #[serde(rename = "fileId")]
    pub file_id: String,
    /// Unicode絵文字か
    #[serde(rename = "isUnicode")]
    pub is_unicode: bool,
}

impl Stamp {
    /// スタンプ情報
    pub fn new(
        id: String,
        name: String,
        creator_id: String,
        created_at: String,
        updated_at: String,
        file_id: String,
        is_unicode: bool,
    ) -> Stamp {
        Stamp {
            id,
            name,
            creator_id,
            created_at,
            updated_at,
            file_id,
            is_unicode,
        }
    }
}
