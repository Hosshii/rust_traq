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

/// struct for typed errors of method `edit_channel_subscribers`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditChannelSubscribersError {
    Status400(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_channel_subscribers`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChannelSubscribersError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_my_channel_subscriptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyChannelSubscriptionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_my_unread_channels`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyUnreadChannelsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `read_channel`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadChannelError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `register_fcm_device`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterFcmDeviceError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_channel_subscribe_level`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChannelSubscribeLevelError {
    Status400(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `set_channel_subscribers`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetChannelSubscribersError {
    Status400(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `ws`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WsError {
    UnknownValue(serde_json::Value),
}

/// 指定したチャンネルの通知購読者を編集します。 リクエストに含めなかったユーザーの通知購読状態は変更しません。 また、存在しないユーザーを指定した場合は無視されます。
pub async fn edit_channel_subscribers(
    configuration: &configuration::Configuration,
    channel_id: &str,
    patch_channel_subscribers_request: Option<crate::models::PatchChannelSubscribersRequest>,
) -> Result<(), Error<EditChannelSubscribersError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/channels/{channelId}/subscribers",
        configuration.base_path,
        channelId = channel_id
    );
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&patch_channel_subscribers_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<EditChannelSubscribersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したチャンネルを通知購読しているユーザーのUUIDのリストを取得します。
pub async fn get_channel_subscribers(
    configuration: &configuration::Configuration,
    channel_id: &str,
) -> Result<Vec<String>, Error<GetChannelSubscribersError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/channels/{channelId}/subscribers",
        configuration.base_path,
        channelId = channel_id
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
        let local_var_entity: Option<GetChannelSubscribersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自身のチャンネル購読状態を取得します。
pub async fn get_my_channel_subscriptions(
    configuration: &configuration::Configuration,
) -> Result<Vec<crate::models::UserSubscribeState>, Error<GetMyChannelSubscriptionsError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/subscriptions", configuration.base_path);
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
        let local_var_entity: Option<GetMyChannelSubscriptionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自分が現在未読のチャンネルの未読情報を取得します。
pub async fn get_my_unread_channels(
    configuration: &configuration::Configuration,
) -> Result<Vec<crate::models::UnreadChannel>, Error<GetMyUnreadChannelsError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/unread", configuration.base_path);
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
        let local_var_entity: Option<GetMyUnreadChannelsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自分が未読のチャンネルを既読にします。
pub async fn read_channel(
    configuration: &configuration::Configuration,
    channel_id: &str,
) -> Result<(), Error<ReadChannelError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/me/unread/{channelId}",
        configuration.base_path,
        channelId = channel_id
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
        let local_var_entity: Option<ReadChannelError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自身のFCMデバイスを登録します。
pub async fn register_fcm_device(
    configuration: &configuration::Configuration,
    post_my_fcm_device_request: Option<crate::models::PostMyFcmDeviceRequest>,
) -> Result<(), Error<RegisterFcmDeviceError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users/me/fcm-device", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&post_my_fcm_device_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<RegisterFcmDeviceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 自身の指定したチャンネルの購読レベルを設定します。
pub async fn set_channel_subscribe_level(
    configuration: &configuration::Configuration,
    channel_id: &str,
    put_channel_subscribe_level_request: Option<crate::models::PutChannelSubscribeLevelRequest>,
) -> Result<(), Error<SetChannelSubscribeLevelError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/users/me/subscriptions/{channelId}",
        configuration.base_path,
        channelId = channel_id
    );
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_channel_subscribe_level_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<SetChannelSubscribeLevelError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// 指定したチャンネルの通知購読者を設定します。 リクエストに含めなかったユーザーの通知購読状態はオフになります。 また、存在しないユーザーを指定した場合は無視されます。
pub async fn set_channel_subscribers(
    configuration: &configuration::Configuration,
    channel_id: &str,
    put_channel_subscribers_request: Option<crate::models::PutChannelSubscribersRequest>,
) -> Result<(), Error<SetChannelSubscribersError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!(
        "{}/channels/{channelId}/subscribers",
        configuration.base_path,
        channelId = channel_id
    );
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&put_channel_subscribers_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<SetChannelSubscribersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// # WebSocketプロトコル ## 送信 `コマンド:引数1:引数2:...`のような形式のTextMessageをサーバーに送信することで、このWebSocketセッションに対する設定が実行できる。 ### `viewstate`コマンド このWebSocketセッションが見ているチャンネル(イベントを受け取るチャンネル)を設定する。 現時点では1つのセッションに対して1つのチャンネルしか設定できない。  `viewstate:{チャンネルID}:{閲覧状態}` + チャンネルID: 対象のチャンネルID + 閲覧状態: `none`, `monitoring`, `editing`  最初の`viewstate`コマンドを送る前、または`viewstate:null`, `viewstate:`を送信した後は、このセッションはどこのチャンネルも見ていないことになる。  ### `rtcstate`コマンド 自分のWebRTC状態を変更する。 他のコネクションが既に状態を保持している場合、変更することができません。  `rtcstate:{チャンネルID}:({状態}:{セッションID})*`  コネクションが切断された場合、自分のWebRTC状態はリセットされます。  ### `timeline_streaming`コマンド 全てのパブリックチャンネルの`MESSAGE_CREATED`イベントを受け取るかどうかを設定する。 初期状態は`off`です。  `timeline_streaming:(on|off|true|false)`  ## 受信 TextMessageとして各種イベントが`type`と`body`を持つJSONとして非同期に送られます。  例:  ```json {\"type\":\"USER_ONLINE\",\"body\":{\"id\":\"7dd8e07f-7f5d-4331-9176-b56a4299768b\"}} ```  ## イベント一覧  ### `USER_JOINED` ユーザーが新規登録された。  対象: 全員  + `id`: 登録されたユーザーのId  ### `USER_UPDATED` ユーザーの情報が更新された。  対象: 全員  + `id`: 情報が更新されたユーザーのId  ### `USER_TAGS_UPDATED` ユーザーのタグが更新された。  対象: 全員  + `id`: タグが更新されたユーザーのId  ### `USER_ICON_UPDATED` ユーザーのアイコンが更新された。  対象: 全員  + `id`: アイコンが更新されたユーザーのId  ### `USER_WEBRTC_STATE_CHANGED` ユーザーのWebRTCの状態が変化した  対象: 全員  + `user_id`: 変更があったユーザーのId + `channel_id`: ユーザーの変更後の接続チャンネルのId + `sessions`: ユーザーの変更後の状態(配列)   + `state`: 状態   + `sessionId`: セッションID  ### `USER_ONLINE` ユーザーがオンラインになった。  対象: 全員  + `id`: オンラインになったユーザーのId  ### `USER_OFFLINE` ユーザーがオフラインになった。  対象: 全員  + `id`: オフラインになったユーザーのId  ### `USER_GROUP_CREATED` ユーザーグループが作成された  対象: 全員  + `id`: 作成されたユーザーグループのId  ### `USER_GROUP_UPDATED` ユーザーグループが更新された  対象: 全員  + `id`: 作成されたユーザーグループのId  ### `USER_GROUP_DELETED` ユーザーグループが削除された  対象: 全員  + `id`: 削除されたユーザーグループのId  ### `CHANNEL_CREATED` チャンネルが新規作成された。  対象: 全員  + `id`: 作成されたチャンネルのId  ### `CHANNEL_UPDATED` チャンネルの情報が変更された。  対象: 全員  + `id`: 変更があったチャンネルのId  ### `CHANNEL_DELETED` チャンネルが削除された。  対象: 全員  + `id`: 削除されたチャンネルのId  ### `CHANNEL_STARED` 自分がチャンネルをスターした。  対象: 自分  + `id`: スターしたチャンネルのId  ### `CHANNEL_UNSTARED` 自分がチャンネルのスターを解除した。  対象: 自分  + `id`: スターしたチャンネルのId  ### `CHANNEL_SUBSCRIBERS_CHANGED` チャンネルの購読者が変化した。  対象: 該当チャンネルを閲覧しているユーザー  + `id`: 変化したチャンネルのId  ### `MESSAGE_CREATED` メッセージが投稿された。  対象: 投稿チャンネルを閲覧しているユーザー・投稿チャンネルに通知をつけているユーザー・メンションを受けたユーザー  + `id`: 投稿されたメッセージのId  ### `MESSAGE_UPDATED` メッセージが更新された。  対象: 投稿チャンネルを閲覧しているユーザー  + `id`: 更新されたメッセージのId  ### `MESSAGE_DELETED` メッセージが削除された。  対象: 投稿チャンネルを閲覧しているユーザー  + `id`: 削除されたメッセージのId  ### `MESSAGE_STAMPED` メッセージにスタンプが押された。  対象: 投稿チャンネルを閲覧しているユーザー  + `message_id`: メッセージId + `user_id`: スタンプを押したユーザーのId + `stamp_id`: スタンプのId + `count`: そのユーザーが押した数 + `created_at`: そのユーザーがそのスタンプをそのメッセージに最初に押した日時  ### `MESSAGE_UNSTAMPED` メッセージからスタンプが外された。  対象: 投稿チャンネルを閲覧しているユーザー  + `message_id`: メッセージId + `user_id`: スタンプを押したユーザーのId + `stamp_id`: スタンプのId  ### `MESSAGE_PINNED` メッセージがピン留めされた。  対象: 投稿チャンネルを閲覧しているユーザー  + `message_id`: ピンされたメッセージのID + `channel_id`: ピンされたメッセージのチャンネルID  ### `MESSAGE_UNPINNED` ピン留めされたメッセージのピンが外された。  対象: 投稿チャンネルを閲覧しているユーザー  + `message_id`: ピンが外されたメッセージのID + `channel_id`: ピンが外されたメッセージのチャンネルID  ### `MESSAGE_READ` 自分があるチャンネルのメッセージを読んだ。  対象: 自分  + `id`: 読んだチャンネルId  ### `STAMP_CREATED` スタンプが新しく追加された。  対象: 全員  + `id`: 作成されたスタンプのId  ### `STAMP_UPDATED` スタンプが修正された。  対象: 全員  + `id`: 修正されたスタンプのId  ### `STAMP_DELETED` スタンプが削除された。  対象: 全員  + `id`: 削除されたスタンプのId  ### `STAMP_PALETTE_CREATED` スタンプパレットが新しく追加された。  対象: 自分  + `id`: 作成されたスタンプパレットのId  ### `STAMP_PALETTE_UPDATED` スタンプパレットが修正された。  対象: 自分  + `id`: 修正されたスタンプパレットのId  ### `STAMP_PALETTE_DELETED` スタンプパレットが削除された。  対象: 自分  + `id`: 削除されたスタンプパレットのId  ### `CLIP_FOLDER_CREATED` クリップフォルダーが作成された。  対象：自分  + `id`: 作成されたクリップフォルダーのId  ### `CLIP_FOLDER_UPDATED` クリップフォルダーが修正された。  対象: 自分  + `id`: 更新されたクリップフォルダーのId  ### `CLIP_FOLDER_DELETED` クリップフォルダーが削除された。  対象: 自分  + `id`: 削除されたクリップフォルダーのId  ### `CLIP_FOLDER_MESSAGE_DELETED` クリップフォルダーからメッセージが除外された。  対象: 自分  + `folder_id`: メッセージが除外されたクリップフォルダーのId + `message_id`: クリップフォルダーから除外されたメッセージのId  ### `CLIP_FOLDER_MESSAGE_ADDED` クリップフォルダーにメッセージが追加された。  対象: 自分  + `folder_id`: メッセージが追加されたクリップフォルダーのId + `message_id`: クリップフォルダーに追加されたメッセージのId
pub async fn ws(configuration: &configuration::Configuration) -> Result<(), Error<WsError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/ws", configuration.base_path);
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
        Ok(())
    } else {
        let local_var_entity: Option<WsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
