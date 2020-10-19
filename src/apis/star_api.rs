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

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `add_my_star`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddMyStarError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_my_stars`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyStarsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `remove_my_star`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveMyStarError {
    UnknownValue(serde_json::Value),
}


/// 指定したチャンネルをスターチャンネルに追加します。 不正なチャンネルIDを指定した場合、400を返します。
pub async fn add_my_star(configuration: &configuration::Configuration, post_star_request: Option<crate::models::PostStarRequest>) -> Result<(), Error<AddMyStarError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/stars", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_star_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<AddMyStarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自分がスターしているチャンネルのUUIDの配列を取得します。
pub async fn get_my_stars(configuration: &configuration::Configuration, ) -> Result<Vec<String>, Error<GetMyStarsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/stars", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<GetMyStarsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 既にスターから削除されているチャンネルを指定した場合は204を返します。
pub async fn remove_my_star(configuration: &configuration::Configuration, channel_id: &str) -> Result<(), Error<RemoveMyStarError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/stars/{channelId}", configuration.base_path, channelId=channel_id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<RemoveMyStarError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
