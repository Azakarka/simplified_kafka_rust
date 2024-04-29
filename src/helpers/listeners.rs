use std::net::IpAddr;
use log::info;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::errors;
use crate::errors::CommonErrors;
use crate::helpers::io::{read_message, write_message};
use crate::helpers::message_parsers::{get_error_message, parse_new_message};

pub struct ListenersMeta {
    pub(crate) address: IpAddr,
    pub(crate) topic: String,
}

pub async fn publisher_listen(
    mut buffer_reader: BufReader<TcpStream>,
    sender: Sender<crate::topic_db::IncomingData>,
    meta: ListenersMeta,
) -> errors::Result<()> {
    loop {
        let mut buffer = vec![];
        read_message(&mut buffer_reader, &mut buffer).await?;

        let parsed_message = parse_new_message(buffer.as_slice()).await;
        match parsed_message {
            Ok(message) => {
                info!(
                    "Got new message from {:?} on topic {} data: {:?}",
                    meta.address, meta.topic, &message
                );
                sender.send(message.message).
                    map_err(|err| CommonErrors::ChannelSendFailed {err})?;
            }
            Err(err) => {
                let error_message = get_error_message().await.as_bytes().to_owned();
                buffer_reader.write_all(error_message.as_slice()).await
                    .map_err(|err| CommonErrors::WriteFailed {err})?;
                return Err(err);
            }
        }
    }
}

pub async fn subscriber_listen(
    mut stream: TcpStream,
    mut receiver: Receiver<crate::topic_db::IncomingData>,
    meta: ListenersMeta,
) -> errors::Result<()> {
    loop {
        let message = receiver.recv().await.unwrap();
        info!(
            "Writing to {} on topic {} message: {}",
            meta.address,
            meta.topic,
            message.as_str()
        );
        let mut bytes = message.as_bytes().to_owned();
        write_message(&mut stream, bytes.as_mut()).await?;
    }
}
