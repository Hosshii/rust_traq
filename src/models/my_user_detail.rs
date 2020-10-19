/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MyUserDetail : 自分のユーザー詳細情報



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyUserDetail {
    /// ユーザーUUID
    #[serde(rename = "id")]
    pub id: String,
    /// 自己紹介(biography)
    #[serde(rename = "bio")]
    pub bio: String,
    /// 所属グループのUUIDの配列
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
    /// タグリスト
    #[serde(rename = "tags")]
    pub tags: Vec<crate::models::UserTag>,
    /// 更新日時
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    /// 最終オンライン日時
    #[serde(rename = "lastOnline")]
    pub last_online: Option<String>,
    /// Twitter ID
    #[serde(rename = "twitterId")]
    pub twitter_id: String,
    /// ユーザー名
    #[serde(rename = "name")]
    pub name: String,
    /// ユーザー表示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// アイコンファイルUUID
    #[serde(rename = "iconFileId")]
    pub icon_file_id: String,
    /// BOTかどうか
    #[serde(rename = "bot")]
    pub bot: bool,
    #[serde(rename = "state")]
    pub state: crate::models::UserAccountState,
    /// 所有している権限の配列
    #[serde(rename = "permissions")]
    pub permissions: Vec<crate::models::UserPermission>,
    /// ホームチャンネル
    #[serde(rename = "homeChannel")]
    pub home_channel: Option<String>,
}

impl MyUserDetail {
    /// 自分のユーザー詳細情報
    pub fn new(id: String, bio: String, groups: Vec<String>, tags: Vec<crate::models::UserTag>, updated_at: String, last_online: Option<String>, twitter_id: String, name: String, display_name: String, icon_file_id: String, bot: bool, state: crate::models::UserAccountState, permissions: Vec<crate::models::UserPermission>, home_channel: Option<String>) -> MyUserDetail {
        MyUserDetail {
            id,
            bio,
            groups,
            tags,
            updated_at,
            last_online,
            twitter_id,
            name,
            display_name,
            icon_file_id,
            bot,
            state,
            permissions,
            home_channel,
        }
    }
}


