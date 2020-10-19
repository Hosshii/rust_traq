# UserDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ユーザーUUID | 
**state** | [**crate::models::UserAccountState**](UserAccountState.md) |  | 
**bot** | **bool** | BOTかどうか | 
**icon_file_id** | **String** | アイコンファイルUUID | 
**display_name** | **String** | ユーザー表示名 | 
**name** | **String** | ユーザー名 | 
**twitter_id** | **String** | Twitter ID | 
**last_online** | Option<**String**> | 最終オンライン日時 | 
**updated_at** | **String** | 更新日時 | 
**tags** | [**Vec<crate::models::UserTag>**](UserTag.md) | タグリスト | 
**groups** | **Vec<String>** | 所属グループのUUIDの配列 | 
**bio** | **String** | 自己紹介(biography) | 
**home_channel** | Option<**String**> | ホームチャンネル | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


