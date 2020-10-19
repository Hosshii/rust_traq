/*
 * traQ v3
 *
 * traQ v3 API
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserPermission : ユーザー権限

/// ユーザー権限
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserPermission {
    #[serde(rename = "get_webhook")]
    GetWebhook,
    #[serde(rename = "create_webhook")]
    CreateWebhook,
    #[serde(rename = "edit_webhook")]
    EditWebhook,
    #[serde(rename = "delete_webhook")]
    DeleteWebhook,
    #[serde(rename = "access_others_webhook")]
    AccessOthersWebhook,
    #[serde(rename = "get_bot")]
    GetBot,
    #[serde(rename = "create_bot")]
    CreateBot,
    #[serde(rename = "edit_bot")]
    EditBot,
    #[serde(rename = "delete_bot")]
    DeleteBot,
    #[serde(rename = "access_others_bot")]
    AccessOthersBot,
    #[serde(rename = "bot_action_join_channel")]
    BotActionJoinChannel,
    #[serde(rename = "bot_action_leave_channel")]
    BotActionLeaveChannel,
    #[serde(rename = "create_channel")]
    CreateChannel,
    #[serde(rename = "get_channel")]
    GetChannel,
    #[serde(rename = "edit_channel")]
    EditChannel,
    #[serde(rename = "delete_channel")]
    DeleteChannel,
    #[serde(rename = "change_parent_channel")]
    ChangeParentChannel,
    #[serde(rename = "edit_channel_topic")]
    EditChannelTopic,
    #[serde(rename = "get_channel_star")]
    GetChannelStar,
    #[serde(rename = "edit_channel_star")]
    EditChannelStar,
    #[serde(rename = "get_my_tokens")]
    GetMyTokens,
    #[serde(rename = "revoke_my_token")]
    RevokeMyToken,
    #[serde(rename = "get_clients")]
    GetClients,
    #[serde(rename = "create_client")]
    CreateClient,
    #[serde(rename = "edit_my_client")]
    EditMyClient,
    #[serde(rename = "delete_my_client")]
    DeleteMyClient,
    #[serde(rename = "manage_others_client")]
    ManageOthersClient,
    #[serde(rename = "upload_file")]
    UploadFile,
    #[serde(rename = "download_file")]
    DownloadFile,
    #[serde(rename = "delete_file")]
    DeleteFile,
    #[serde(rename = "get_message")]
    GetMessage,
    #[serde(rename = "post_message")]
    PostMessage,
    #[serde(rename = "edit_message")]
    EditMessage,
    #[serde(rename = "delete_message")]
    DeleteMessage,
    #[serde(rename = "report_message")]
    ReportMessage,
    #[serde(rename = "get_message_reports")]
    GetMessageReports,
    #[serde(rename = "create_message_pin")]
    CreateMessagePin,
    #[serde(rename = "delete_message_pin")]
    DeleteMessagePin,
    #[serde(rename = "get_channel_subscription")]
    GetChannelSubscription,
    #[serde(rename = "edit_channel_subscription")]
    EditChannelSubscription,
    #[serde(rename = "connect_notification_stream")]
    ConnectNotificationStream,
    #[serde(rename = "register_fcm_device")]
    RegisterFCMDevice,
    #[serde(rename = "get_stamp")]
    GetStamp,
    #[serde(rename = "create_stamp")]
    CreateStamp,
    #[serde(rename = "edit_stamp")]
    EditStamp,
    #[serde(rename = "edit_stamp_created_by_others")]
    EditStampCreatedByOthers,
    #[serde(rename = "delete_stamp")]
    DeleteStamp,
    #[serde(rename = "add_message_stamp")]
    AddMessageStamp,
    #[serde(rename = "remove_message_stamp")]
    RemoveMessageStamp,
    #[serde(rename = "get_my_stamp_history")]
    GetMyStampHistory,
    #[serde(rename = "get_stamp_palette")]
    GetStampPalette,
    #[serde(rename = "create_stamp_palette")]
    CreateStampPalette,
    #[serde(rename = "edit_stamp_palette")]
    EditStampPalette,
    #[serde(rename = "delete_stamp_palette")]
    DeleteStampPalette,
    #[serde(rename = "get_user")]
    GetUser,
    #[serde(rename = "register_user")]
    RegisterUser,
    #[serde(rename = "get_me")]
    GetMe,
    #[serde(rename = "edit_me")]
    EditMe,
    #[serde(rename = "change_my_icon")]
    ChangeMyIcon,
    #[serde(rename = "change_my_password")]
    ChangeMyPassword,
    #[serde(rename = "edit_other_users")]
    EditOtherUsers,
    #[serde(rename = "get_user_qr_code")]
    GetUserQRCode,
    #[serde(rename = "get_user_tag")]
    GetUserTag,
    #[serde(rename = "edit_user_tag")]
    EditUserTag,
    #[serde(rename = "get_user_group")]
    GetUserGroup,
    #[serde(rename = "create_user_group")]
    CreateUserGroup,
    #[serde(rename = "create_special_user_group")]
    CreateSpecialUserGroup,
    #[serde(rename = "edit_user_group")]
    EditUserGroup,
    #[serde(rename = "delete_user_group")]
    DeleteUserGroup,
    #[serde(rename = "web_rtc")]
    WebRTC,
    #[serde(rename = "get_my_sessions")]
    GetMySessions,
    #[serde(rename = "delete_my_sessions")]
    DeleteMySessions,
    #[serde(rename = "get_my_external_account")]
    GetMyExternalAccount,
    #[serde(rename = "edit_my_external_account")]
    EditMyExternalAccount,
    #[serde(rename = "get_unread")]
    GetUnread,
    #[serde(rename = "delete_unread")]
    DeleteUnread,
    #[serde(rename = "get_clip_folder")]
    GetClipFolder,
    #[serde(rename = "create_clip_folder")]
    CreateClipFolder,
    #[serde(rename = "edit_clip_folder")]
    EditClipFolder,
    #[serde(rename = "delete_clip_folder")]
    DeleteClipFolder,

}




