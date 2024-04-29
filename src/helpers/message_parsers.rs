use std::str;
use serde::{Deserialize, Serialize};
use crate::errors::CommonErrors;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Publish,
    Subscribe,
}
#[derive(Deserialize, Debug)]
pub struct AuthMessage {
    pub method: ClientType,
    pub topic: String,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message: String,
}

#[derive(Serialize)]
struct ErrorMessage {
    error: &'static str,
}
const ERROR_MESSAGE_TEXT: &'static str = "received not valid json";

pub async fn parse_auth_message(message: &[u8]) -> Result<AuthMessage, CommonErrors> {
    serde_json::from_slice(message)
        .map_err(|err| CommonErrors::CannotParseMessage {
            err,
            message: str::from_utf8(message).unwrap().to_string()
        })
}

pub async fn parse_new_message(message: &[u8]) -> Result<Message, CommonErrors> {
    serde_json::from_slice(message)
        .map_err(|err| CommonErrors::CannotParseMessage {
            err,
            message: str::from_utf8(message).unwrap().to_string()
        })
}

pub async fn get_error_message() -> String {
    let error_message = ErrorMessage {
        error: ERROR_MESSAGE_TEXT,
    };
    serde_json::to_string(&error_message).unwrap()
}
