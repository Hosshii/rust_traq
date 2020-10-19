# \BotApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_bot**](BotApi.md#activate_bot) | **post** /bots/{botId}/actions/activate | BOTをアクティベート
[**change_bot_icon**](BotApi.md#change_bot_icon) | **put** /bots/{botId}/icon | BOTのアイコン画像を変更
[**create_bot**](BotApi.md#create_bot) | **post** /bots | BOTを作成
[**delete_bot**](BotApi.md#delete_bot) | **delete** /bots/{botId} | BOTを削除
[**edit_bot**](BotApi.md#edit_bot) | **patch** /bots/{botId} | BOT情報を変更
[**get_bot**](BotApi.md#get_bot) | **get** /bots/{botId} | BOT情報を取得
[**get_bot_icon**](BotApi.md#get_bot_icon) | **get** /bots/{botId}/icon | BOTのアイコン画像を取得
[**get_bot_logs**](BotApi.md#get_bot_logs) | **get** /bots/{botId}/logs | BOTのイベントログを取得
[**get_bots**](BotApi.md#get_bots) | **get** /bots | BOTリストを取得
[**get_channel_bots**](BotApi.md#get_channel_bots) | **get** /channels/{channelId}/bots | チャンネル参加中のBOTのリストを取得
[**inactivate_bot**](BotApi.md#inactivate_bot) | **post** /bots/{botId}/actions/inactivate | BOTをインアクティベート
[**let_bot_join_channel**](BotApi.md#let_bot_join_channel) | **post** /bots/{botId}/actions/join | BOTをチャンネルに参加させる
[**let_bot_leave_channel**](BotApi.md#let_bot_leave_channel) | **post** /bots/{botId}/actions/leave | BOTをチャンネルから退出させる
[**reissue_bot**](BotApi.md#reissue_bot) | **post** /bots/{botId}/actions/reissue | BOTのトークンを再発行



## activate_bot

> activate_bot(bot_id)
BOTをアクティベート

指定したBOTを有効化します。 対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_bot_icon

> change_bot_icon(bot_id, file)
BOTのアイコン画像を変更

指定したBOTのアイコン画像を変更を変更します。 対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |
**file** | **std::path::PathBuf** | アイコン画像(1MBまでのpng, jpeg, gif) | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_bot

> crate::models::BotDetail create_bot(post_bot_request)
BOTを作成

BOTを作成します。 作成後にアクティベーション・購読イベントの設定を行う必要があります。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_bot_request** | Option<[**PostBotRequest**](PostBotRequest.md)> |  |  |

### Return type

[**crate::models::BotDetail**](BotDetail.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bot

> delete_bot(bot_id)
BOTを削除

指定したBOTを削除します。 対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_bot

> edit_bot(bot_id, patch_bot_request)
BOT情報を変更

指定したBOTの情報を変更します。 対象のBOTの管理権限が必要です。 BOT開発者UUIDを変更した場合は、変更先ユーザーにBOT管理権限が移譲され、自分自身は権限を失います。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |
**patch_bot_request** | Option<[**PatchBotRequest**](PatchBotRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bot

> crate::models::OneOfBotBotDetail get_bot(bot_id, detail)
BOT情報を取得

指定したBOTのBOT情報を取得します。 BOT詳細情報を取得する場合は、対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |
**detail** | Option<**bool**> | 詳細情報を含めるかどうか |  |[default to false]

### Return type

[**crate::models::OneOfBotBotDetail**](oneOf<Bot,BotDetail>.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bot_icon

> std::path::PathBuf get_bot_icon(bot_id)
BOTのアイコン画像を取得

指定したBOTのアイコン画像を取得を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/gif, image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bot_logs

> Vec<crate::models::BotEventLog> get_bot_logs(bot_id, limit, offset)
BOTのイベントログを取得

指定したBOTのイベントログを取得します。 対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |
**limit** | Option<**i32**> | 取得する件数 |  |[default to 0]
**offset** | Option<**i32**> | 取得するオフセット |  |[default to 0]

### Return type

[**Vec<crate::models::BotEventLog>**](BotEventLog.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bots

> Vec<crate::models::Bot> get_bots(all)
BOTリストを取得

BOT情報のリストを取得します。 allを指定しない場合、自分が開発者のBOTのみを返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | 全てのBOTを取得するかどうか |  |[default to false]

### Return type

[**Vec<crate::models::Bot>**](Bot.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_bots

> Vec<crate::models::BotUser> get_channel_bots(channel_id)
チャンネル参加中のBOTのリストを取得

指定したチャンネルに参加しているBOTのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | [**String**](.md) | チャンネルUUID | [required] |

### Return type

[**Vec<crate::models::BotUser>**](BotUser.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inactivate_bot

> inactivate_bot(bot_id)
BOTをインアクティベート

指定したBOTを無効化します。対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## let_bot_join_channel

> let_bot_join_channel(bot_id, post_bot_action_join_request)
BOTをチャンネルに参加させる

指定したBOTを指定したチャンネルに参加させます。 チャンネルに参加したBOTは、そのチャンネルの各種イベントを受け取るようになります。 対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |
**post_bot_action_join_request** | Option<[**PostBotActionJoinRequest**](PostBotActionJoinRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## let_bot_leave_channel

> let_bot_leave_channel(bot_id, post_bot_action_leave_request)
BOTをチャンネルから退出させる

指定したBOTを指定したチャンネルから退出させます。 対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |
**post_bot_action_leave_request** | Option<[**PostBotActionLeaveRequest**](PostBotActionLeaveRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reissue_bot

> crate::models::BotTokens reissue_bot(bot_id)
BOTのトークンを再発行

指定したBOTの現在の各種トークンを無効化し、再発行を行います。 対象のBOTの管理権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | [**String**](.md) | BOTUUID | [required] |

### Return type

[**crate::models::BotTokens**](BotTokens.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

