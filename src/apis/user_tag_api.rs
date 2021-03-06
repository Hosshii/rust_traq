/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `add_my_user_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddMyUserTagError {
    Status400(),
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `add_user_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddUserTagError {
    Status400(),
    Status403(),
    Status404(),
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `edit_my_user_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditMyUserTagError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `edit_user_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditUserTagError {
    Status400(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_my_user_tags`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyUserTagsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTagError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_user_tags`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserTagsError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `remove_my_user_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveMyUserTagError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `remove_user_tag`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUserTagError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// 自分に新しくタグを追加します。
pub async fn add_my_user_tag(
    configuration: &configuration::Configuration,
    post_user_tag_request: Option<crate::models::PostUserTagRequest>,
) -> Result<crate::models::UserTag, Error<AddMyUserTagError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/tags", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_user_tag_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddMyUserTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したユーザーに指定したタグを追加します。 Webhookユーザーにタグを追加することは出来ません。
pub async fn add_user_tag(
    configuration: &configuration::Configuration,
    user_id: &str,
    post_user_tag_request: Option<crate::models::PostUserTagRequest>,
) -> Result<crate::models::UserTag, Error<AddUserTagError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/{userId}/tags",
        configuration.base_path,
        userId = user_id
    );
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_user_tag_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddUserTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自分の指定したタグの状態を変更します。
pub async fn edit_my_user_tag(
    configuration: &configuration::Configuration,
    tag_id: &str,
    patch_user_tag_request: Option<crate::models::PatchUserTagRequest>,
) -> Result<(), Error<EditMyUserTagError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/me/tags/{tagId}",
        configuration.base_path,
        tagId = tag_id
    );
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&patch_user_tag_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<EditMyUserTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したユーザーの指定したタグの状態を変更します。 他人の状態は変更できません。
pub async fn edit_user_tag(
    configuration: &configuration::Configuration,
    user_id: &str,
    tag_id: &str,
    patch_user_tag_request: Option<crate::models::PatchUserTagRequest>,
) -> Result<(), Error<EditUserTagError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/{userId}/tags/{tagId}",
        configuration.base_path,
        userId = user_id,
        tagId = tag_id
    );
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&patch_user_tag_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<EditUserTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自分に付けられているタグの配列を取得します。
pub async fn get_my_user_tags(
    configuration: &configuration::Configuration,
) -> Result<Vec<crate::models::UserTag>, Error<GetMyUserTagsError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/tags", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMyUserTagsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したタグの情報を取得します。
pub async fn get_tag(
    configuration: &configuration::Configuration,
    tag_id: &str,
) -> Result<crate::models::Tag, Error<GetTagError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/tags/{tagId}", configuration.base_path, tagId = tag_id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTagError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したユーザーのタグリストを取得します。
pub async fn get_user_tags(
    configuration: &configuration::Configuration,
    user_id: &str,
) -> Result<Vec<crate::models::UserTag>, Error<GetUserTagsError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/{userId}/tags",
        configuration.base_path,
        userId = user_id
    );
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUserTagsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 既に存在しないタグを削除しようとした場合は204を返します。
pub async fn remove_my_user_tag(
    configuration: &configuration::Configuration,
    tag_id: &str,
) -> Result<(), Error<RemoveMyUserTagError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/me/tags/{tagId}",
        configuration.base_path,
        tagId = tag_id
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<RemoveMyUserTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 既に存在しないタグを削除しようとした場合は204を返します。
pub async fn remove_user_tag(
    configuration: &configuration::Configuration,
    user_id: &str,
    tag_id: &str,
) -> Result<(), Error<RemoveUserTagError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/{userId}/tags/{tagId}",
        configuration.base_path,
        userId = user_id,
        tagId = tag_id
    );
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<RemoveUserTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
