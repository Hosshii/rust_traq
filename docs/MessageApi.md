# \MessageApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_message_stamp**](MessageApi.md#add_message_stamp) | **post** /messages/{messageId}/stamps/{stampId} | スタンプを押す
[**create_pin**](MessageApi.md#create_pin) | **post** /messages/{messageId}/pin | ピン留めする
[**delete_message**](MessageApi.md#delete_message) | **delete** /messages/{messageId} | メッセージを削除
[**edit_message**](MessageApi.md#edit_message) | **put** /messages/{messageId} | メッセージを編集
[**get_direct_messages**](MessageApi.md#get_direct_messages) | **get** /users/{userId}/messages | ダイレクトメッセージのリストを取得
[**get_message**](MessageApi.md#get_message) | **get** /messages/{messageId} | メッセージを取得
[**get_message_clips**](MessageApi.md#get_message_clips) | **get** /messages/{messageId}/clips | 自分のクリップを取得
[**get_message_stamps**](MessageApi.md#get_message_stamps) | **get** /messages/{messageId}/stamps | メッセージのスタンプリストを取得
[**get_messages**](MessageApi.md#get_messages) | **get** /channels/{channelId}/messages | チャンネルメッセージのリストを取得
[**get_pin**](MessageApi.md#get_pin) | **get** /messages/{messageId}/pin | ピン留めを取得
[**post_direct_message**](MessageApi.md#post_direct_message) | **post** /users/{userId}/messages | ダイレクトメッセージを送信
[**post_message**](MessageApi.md#post_message) | **post** /channels/{channelId}/messages | チャンネルにメッセージを投稿
[**remove_message_stamp**](MessageApi.md#remove_message_stamp) | **delete** /messages/{messageId}/stamps/{stampId} | スタンプを消す
[**remove_pin**](MessageApi.md#remove_pin) | **delete** /messages/{messageId}/pin | ピン留めを外す



## add_message_stamp

> add_message_stamp(message_id, stamp_id, post_message_stamp_request)
スタンプを押す

指定したメッセージに指定したスタンプを押します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |
**stamp_id** | [**String**](.md) | スタンプUUID | [required] |
**post_message_stamp_request** | Option<[**PostMessageStampRequest**](PostMessageStampRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pin

> crate::models::MessagePin create_pin(message_id)
ピン留めする

指定したメッセージをピン留めします。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |

### Return type

[**crate::models::MessagePin**](MessagePin.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message

> delete_message(message_id)
メッセージを削除

指定したメッセージを削除します。 自身が投稿したメッセージと自身が管理権限を持つWebhookとBOTが投稿したメッセージのみ削除することができます。 アーカイブされているチャンネルのメッセージを編集することは出来ません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_message

> crate::models::Message edit_message(message_id, post_message_request)
メッセージを編集

指定したメッセージを編集します。 自身が投稿したメッセージと自身が管理権限を持つWebhookとBOTが投稿したメッセージのみ編集することができます。 アーカイブされているチャンネルのメッセージを編集することは出来ません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |
**post_message_request** | Option<[**PostMessageRequest**](PostMessageRequest.md)> |  |  |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_direct_messages

> Vec<crate::models::Message> get_direct_messages(user_id, limit, offset, since, until, inclusive, order)
ダイレクトメッセージのリストを取得

指定したユーザーとのダイレクトメッセージのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**limit** | Option<**i32**> | 取得する件数 |  |[default to 0]
**offset** | Option<**i32**> | 取得するオフセット |  |[default to 0]
**since** | Option<**String**> | 取得する時間範囲の開始日時 |  |[default to 0000-01-01T00:00Z]
**until** | Option<**String**> | 取得する時間範囲の終了日時 |  |
**inclusive** | Option<**bool**> | 範囲の端を含めるかどうか |  |[default to false]
**order** | Option<**String**> | 昇順か降順か |  |[default to desc]

### Return type

[**Vec<crate::models::Message>**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message

> crate::models::Message get_message(message_id)
メッセージを取得

指定したメッセージを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_clips

> Vec<crate::models::MessageClip> get_message_clips(message_id)
自分のクリップを取得

対象のメッセージの自分のクリップの一覧を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |

### Return type

[**Vec<crate::models::MessageClip>**](MessageClip.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_stamps

> Vec<crate::models::MessageStamp> get_message_stamps(message_id)
メッセージのスタンプリストを取得

指定したメッセージに押されているスタンプのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |

### Return type

[**Vec<crate::models::MessageStamp>**](MessageStamp.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_messages

> Vec<crate::models::Message> get_messages(channel_id, limit, offset, since, until, inclusive, order)
チャンネルメッセージのリストを取得

指定したチャンネルのメッセージのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | [**String**](.md) | チャンネルUUID | [required] |
**limit** | Option<**i32**> | 取得する件数 |  |[default to 0]
**offset** | Option<**i32**> | 取得するオフセット |  |[default to 0]
**since** | Option<**String**> | 取得する時間範囲の開始日時 |  |[default to 0000-01-01T00:00Z]
**until** | Option<**String**> | 取得する時間範囲の終了日時 |  |
**inclusive** | Option<**bool**> | 範囲の端を含めるかどうか |  |[default to false]
**order** | Option<**String**> | 昇順か降順か |  |[default to desc]

### Return type

[**Vec<crate::models::Message>**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pin

> crate::models::MessagePin get_pin(message_id)
ピン留めを取得

指定したメッセージのピン留め情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |

### Return type

[**crate::models::MessagePin**](MessagePin.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_direct_message

> crate::models::Message post_direct_message(user_id, post_message_request)
ダイレクトメッセージを送信

指定したユーザーにダイレクトメッセージを送信します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**post_message_request** | Option<[**PostMessageRequest**](PostMessageRequest.md)> |  |  |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_message

> crate::models::Message post_message(channel_id, post_message_request)
チャンネルにメッセージを投稿

指定したチャンネルにメッセージを投稿します。 embedをtrueに指定すると、メッセージ埋め込みが自動で行われます。 アーカイブされているチャンネルに投稿することはできません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | [**String**](.md) | チャンネルUUID | [required] |
**post_message_request** | Option<[**PostMessageRequest**](PostMessageRequest.md)> |  |  |

### Return type

[**crate::models::Message**](Message.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_message_stamp

> remove_message_stamp(message_id, stamp_id)
スタンプを消す

指定したメッセージから指定した自身が押したスタンプを削除します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |
**stamp_id** | [**String**](.md) | スタンプUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_pin

> remove_pin(message_id)
ピン留めを外す

指定したメッセージのピン留めを外します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | [**String**](.md) | メッセージUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

