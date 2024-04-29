use std::str;
use std::net::{IpAddr, SocketAddr};
use clap::Parser;
use log::{error, info, warn};
use tokio::io::BufReader;
use tokio::net::{TcpListener, TcpStream};
use crate::errors::CommonErrors;
use crate::helpers::client_handlers::add_new_client;
use crate::helpers::io::read_message;
use crate::helpers::message_parsers::parse_auth_message;
use crate::topic_db::{init_topic_db, TopicMap};

mod topic_db;
mod errors;
mod helpers;

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Arguments {
    #[clap(short, long)]
    pub address: IpAddr,

    #[clap(short, long)]
    pub port: u16,
}

async fn handle_connection(
    stream: TcpStream,
    client_address: SocketAddr,
    topic_db: TopicMap,
) -> errors::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    let address = stream
        .peer_addr()
        .map_err(|err| CommonErrors::PeerAddressUnavailable { err })?;
    let mut buffer_reader = BufReader::new(stream);

    read_message(&mut buffer_reader, &mut buffer).await?;
    info!(
        "Read auth message from {}, raw: {:?}",
        client_address,
        str::from_utf8(&buffer)
    );

    let auth_message = match parse_auth_message(buffer.as_slice()).await {
        Ok(message) => message,
        Err(err) => {
            warn!(
                "Error parsing auth message from {}, error: {:?}",
                address, err
            );
            return Err(err);
        }
    };
    info!(
        "Parsed auth message from {}: {:?}",
        client_address, auth_message
    );

    add_new_client(buffer_reader, auth_message, topic_db).await
}

pub async fn run(arguments: Arguments) -> Result<(), std::io::Error> {
    let server_socket = SocketAddr::new(arguments.address, arguments.port);
    let listener = TcpListener::bind(server_socket).await?;
    info!("Start kafka server on address {}", server_socket);
    let topic_db = init_topic_db();
    loop {
        let topic_db_clone = topic_db.clone();
        let (stream, address) = listener.accept().await?;
        tokio::spawn(async move {
            info!("New connection from addr: {}", address);
            match handle_connection(stream, address.clone(), topic_db_clone).await {
                Ok(_) => info!("Closing connection with {}", address),
                Err(err) => {
                    error!("Connection with {} ended with error: {:?}", address, err);
                }
            }
        });
    }
}
