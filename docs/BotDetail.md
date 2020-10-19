# BotDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | BOT UUID | 
**updated_at** | **String** | 更新日時 | 
**created_at** | **String** | 作成日時 | 
**state** | [**crate::models::BotState**](BotState.md) |  | 
**subscribe_events** | **Vec<String>** | BOTが購読しているイベントの配列 | 
**developer_id** | **String** | BOT開発者UUID | 
**description** | **String** | 説明 | 
**bot_user_id** | **String** | BOTユーザーUUID | 
**tokens** | [**crate::models::BotTokens**](BotTokens.md) |  | 
**endpoint** | **String** | BOTサーバーエンドポイント | 
**privileged** | **bool** | 特権BOTかどうか | 
**channels** | **Vec<String>** | BOTが参加しているチャンネルのUUID配列 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


