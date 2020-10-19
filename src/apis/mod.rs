use reqwest;
use serde_json;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod activity_api;
pub mod authentication_api;
pub mod bot_api;
pub mod channel_api;
pub mod clip_api;
pub mod default_api;
pub mod file_api;
pub mod group_api;
pub mod me_api;
pub mod message_api;
pub mod notification_api;
pub mod oauth2_api;
pub mod pin_api;
pub mod public_api;
pub mod stamp_api;
pub mod star_api;
pub mod user_api;
pub mod user_tag_api;
pub mod webhook_api;
pub mod webrtc_api;

pub mod configuration;
