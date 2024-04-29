use std::result;
use tokio::io::Error;
use crate::topic_db::IncomingData;

#[derive(Debug)]
pub enum CommonErrors {
    CannotParseMessage { err: serde_json::Error, message: String },
    ReadFailed { err: Error },
    UnexpectedEOF,
    WriteFailed { err: Error },
    PeerAddressUnavailable { err: Error },
    ChannelSendFailed {err: tokio::sync::broadcast::error::SendError<IncomingData>}
}

pub type Result<T> = result::Result<T, CommonErrors>;
