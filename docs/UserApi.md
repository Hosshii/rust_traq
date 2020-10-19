# \UserApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_tag**](UserApi.md#add_user_tag) | **post** /users/{userId}/tags | ユーザーにタグを追加
[**change_user_icon**](UserApi.md#change_user_icon) | **put** /users/{userId}/icon | ユーザーのアイコン画像を変更します
[**change_user_password**](UserApi.md#change_user_password) | **put** /users/{userId}/password | ユーザーのパスワードを変更
[**create_user**](UserApi.md#create_user) | **post** /users | ユーザーを登録
[**edit_user**](UserApi.md#edit_user) | **patch** /users/{userId} | ユーザー情報を変更
[**edit_user_tag**](UserApi.md#edit_user_tag) | **patch** /users/{userId}/tags/{tagId} | ユーザーのタグを編集
[**get_direct_messages**](UserApi.md#get_direct_messages) | **get** /users/{userId}/messages | ダイレクトメッセージのリストを取得
[**get_user**](UserApi.md#get_user) | **get** /users/{userId} | ユーザー詳細情報を取得
[**get_user_dm_channel**](UserApi.md#get_user_dm_channel) | **get** /users/{userId}/dm-channel | DMチャンネル情報を取得
[**get_user_icon**](UserApi.md#get_user_icon) | **get** /users/{userId}/icon | ユーザーのアイコン画像を取得
[**get_user_tags**](UserApi.md#get_user_tags) | **get** /users/{userId}/tags | ユーザーのタグリストを取得
[**get_users**](UserApi.md#get_users) | **get** /users | ユーザーのリストを取得
[**post_direct_message**](UserApi.md#post_direct_message) | **post** /users/{userId}/messages | ダイレクトメッセージを送信
[**remove_user_tag**](UserApi.md#remove_user_tag) | **delete** /users/{userId}/tags/{tagId} | ユーザーからタグを削除します



## add_user_tag

> crate::models::UserTag add_user_tag(user_id, post_user_tag_request)
ユーザーにタグを追加

指定したユーザーに指定したタグを追加します。 Webhookユーザーにタグを追加することは出来ません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**post_user_tag_request** | Option<[**PostUserTagRequest**](PostUserTagRequest.md)> |  |  |

### Return type

[**crate::models::UserTag**](UserTag.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_user_icon

> change_user_icon(user_id, file)
ユーザーのアイコン画像を変更します

指定したユーザーのアイコン画像を変更します。 管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**file** | **std::path::PathBuf** | アイコン画像(1MBまでのpng, jpeg, gif) | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_user_password

> change_user_password(user_id, put_user_password_request)
ユーザーのパスワードを変更

指定したユーザーのパスワードを変更します。 管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**put_user_password_request** | Option<[**PutUserPasswordRequest**](PutUserPasswordRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::UserDetail create_user(post_user_request)
ユーザーを登録

ユーザーを登録します。 管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_user_request** | Option<[**PostUserRequest**](PostUserRequest.md)> |  |  |

### Return type

[**crate::models::UserDetail**](UserDetail.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user

> edit_user(user_id, patch_user_request)
ユーザー情報を変更

指定したユーザーの情報を変更します。 管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**patch_user_request** | Option<[**PatchUserRequest**](PatchUserRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user_tag

> edit_user_tag(user_id, tag_id, patch_user_tag_request)
ユーザーのタグを編集

指定したユーザーの指定したタグの状態を変更します。 他人の状態は変更できません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**tag_id** | [**String**](.md) | タグUUID | [required] |
**patch_user_tag_request** | Option<[**PatchUserTagRequest**](PatchUserTagRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

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


## get_user

> crate::models::UserDetail get_user(user_id)
ユーザー詳細情報を取得

指定したユーザーの詳細情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |

### Return type

[**crate::models::UserDetail**](UserDetail.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_dm_channel

> crate::models::DmChannel get_user_dm_channel(user_id)
DMチャンネル情報を取得

指定したユーザーとのダイレクトメッセージチャンネルの情報を返します。 ダイレクトメッセージチャンネルが存在しなかった場合、自動的に作成されます。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::DmChannel**](DMChannel.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_icon

> std::path::PathBuf get_user_icon(user_id)
ユーザーのアイコン画像を取得

指定したユーザーのアイコン画像を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/jpeg, image/gif, image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_tags

> Vec<crate::models::UserTag> get_user_tags(user_id)
ユーザーのタグリストを取得

指定したユーザーのタグリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |

### Return type

[**Vec<crate::models::UserTag>**](UserTag.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> Vec<crate::models::User> get_users(include_suspended)
ユーザーのリストを取得

ユーザーのリストを取得します。 `include-suspended`を指定しない場合、レスポンスに非アクティブユーザーは含まれません。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_suspended** | Option<**bool**> | アカウントがアクティブでないユーザーを含めるかどうか |  |[default to false]

### Return type

[**Vec<crate::models::User>**](User.md)

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


## remove_user_tag

> remove_user_tag(user_id, tag_id)
ユーザーからタグを削除します

既に存在しないタグを削除しようとした場合は204を返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**tag_id** | [**String**](.md) | タグUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

