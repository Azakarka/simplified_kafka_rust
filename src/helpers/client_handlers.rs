use log::info;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use crate::errors;
use crate::errors::CommonErrors;
use crate::helpers::listeners::{ListenersMeta, publisher_listen, subscriber_listen};
use crate::helpers::message_parsers::{AuthMessage, ClientType};
use crate::topic_db::{get_receiver, get_sender, TopicMap};

pub async fn add_new_client(
    buffer_reader: BufReader<TcpStream>,
    auth_message: AuthMessage,
    mut topic_db: TopicMap,
) -> errors::Result<()> {
    let listeners_meta = ListenersMeta {
        address: buffer_reader.get_ref().peer_addr().map_err(|err| CommonErrors::PeerAddressUnavailable {err})?.ip(),
        topic: auth_message.topic.clone(),
    };
    match auth_message.method {
        ClientType::Publish => {
            let sender = get_sender(&mut topic_db, auth_message.topic.as_str()).await;
            info!("For topic {} connected publisher with ip {}",
                auth_message.topic.as_str(),
                listeners_meta.address);
            publisher_listen(buffer_reader, sender, listeners_meta).await
        }
        ClientType::Subscribe => {
            let stream = buffer_reader.into_inner();
            let receiver = get_receiver(&mut topic_db, auth_message.topic.as_str()).await;
            info!(
                "For topic {} connected subscriber with ip {}",
                auth_message.topic.as_str(),
                listeners_meta.address
            );
            subscriber_listen(stream, receiver, listeners_meta).await
        }
    }
}