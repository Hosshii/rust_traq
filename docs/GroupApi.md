# \GroupApi

All URIs are relative to *https://q.trap.jp/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_group_admin**](GroupApi.md#add_user_group_admin) | **post** /groups/{groupId}/admins | グループ管理者を追加
[**add_user_group_member**](GroupApi.md#add_user_group_member) | **post** /groups/{groupId}/members | グループメンバーを追加
[**create_user_group**](GroupApi.md#create_user_group) | **post** /groups | ユーザーグループを作成
[**delete_user_group**](GroupApi.md#delete_user_group) | **delete** /groups/{groupId} | ユーザーグループを削除
[**edit_user_group**](GroupApi.md#edit_user_group) | **patch** /groups/{groupId} | ユーザーグループを編集
[**edit_user_group_member**](GroupApi.md#edit_user_group_member) | **patch** /groups/{groupId}/members/{userId} | ユーザーグループメンバーを編集
[**get_user_group**](GroupApi.md#get_user_group) | **get** /groups/{groupId} | ユーザーグループを取得
[**get_user_group_admins**](GroupApi.md#get_user_group_admins) | **get** /groups/{groupId}/admins | グループ管理者を取得
[**get_user_group_members**](GroupApi.md#get_user_group_members) | **get** /groups/{groupId}/members | グループメンバーを取得
[**get_user_groups**](GroupApi.md#get_user_groups) | **get** /groups | ユーザーグループのリストを取得
[**remove_user_group_admin**](GroupApi.md#remove_user_group_admin) | **delete** /groups/{groupId}/admins/{userId} | グループ管理者を削除
[**remove_user_group_member**](GroupApi.md#remove_user_group_member) | **delete** /groups/{groupId}/members/{userId} | ユーザーグループから削除



## add_user_group_admin

> add_user_group_admin(group_id, post_user_group_admin_request)
グループ管理者を追加

指定したグループに管理者を追加します。 対象のユーザーグループの管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |
**post_user_group_admin_request** | Option<[**PostUserGroupAdminRequest**](PostUserGroupAdminRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_group_member

> add_user_group_member(group_id, user_group_member)
グループメンバーを追加

指定したグループにメンバーを追加します。 対象のユーザーグループの管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |
**user_group_member** | Option<[**UserGroupMember**](UserGroupMember.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_group

> crate::models::UserGroup create_user_group(post_user_group_request)
ユーザーグループを作成

ユーザーグループを作成します。 作成者は自動的にグループ管理者になります。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_user_group_request** | Option<[**PostUserGroupRequest**](PostUserGroupRequest.md)> |  |  |

### Return type

[**crate::models::UserGroup**](UserGroup.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_group

> delete_user_group(group_id)
ユーザーグループを削除

指定したユーザーグループを削除します。 対象のユーザーグループの管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user_group

> edit_user_group(group_id, patch_user_group_request)
ユーザーグループを編集

指定したユーザーグループの情報を編集します。 対象のユーザーグループの管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |
**patch_user_group_request** | Option<[**PatchUserGroupRequest**](PatchUserGroupRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user_group_member

> edit_user_group_member(group_id, user_id, patch_group_member_request)
ユーザーグループメンバーを編集

指定したユーザーグループ内の指定したユーザーの属性を編集します。 対象のユーザーグループの管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |
**user_id** | [**String**](.md) | ユーザーUUID | [required] |
**patch_group_member_request** | Option<[**PatchGroupMemberRequest**](PatchGroupMemberRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group

> crate::models::UserGroup get_user_group(group_id)
ユーザーグループを取得

指定したユーザーグループの情報を取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |

### Return type

[**crate::models::UserGroup**](UserGroup.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group_admins

> Vec<String> get_user_group_admins(group_id)
グループ管理者を取得

指定したグループの管理者のリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |

### Return type

**Vec<String>**

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group_members

> Vec<crate::models::UserGroupMember> get_user_group_members(group_id)
グループメンバーを取得

指定したグループのメンバーのリストを取得します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |

### Return type

[**Vec<crate::models::UserGroupMember>**](UserGroupMember.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_groups

> Vec<crate::models::UserGroup> get_user_groups()
ユーザーグループのリストを取得

ユーザーグループのリストを取得します。

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::UserGroup>**](UserGroup.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_group_admin

> remove_user_group_admin(group_id, user_id)
グループ管理者を削除

指定したユーザーグループから指定した管理者を削除します。 対象のユーザーグループの管理者権限が必要です。 グループから管理者が存在しなくなる場合は400エラーを返します。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |
**user_id** | [**String**](.md) | ユーザーUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_group_member

> remove_user_group_member(group_id, user_id)
ユーザーグループから削除

指定したユーザーグループから指定したユーザーを削除します。 対象のユーザーグループの管理者権限が必要です。

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**](.md) | ユーザーグループUUID | [required] |
**user_id** | [**String**](.md) | ユーザーUUID | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

