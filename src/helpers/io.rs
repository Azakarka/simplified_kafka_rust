use crate::errors::CommonErrors;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

const SOH: u8 = '\n' as u8;

pub async fn read_message(
    buffer_reader: &mut BufReader<TcpStream>,
    buffer: &mut Vec<u8>,
) -> Result<(), CommonErrors> {
    match buffer_reader.read_until(SOH, buffer).await {
        Ok(n) if n == 0 => {
            Err(CommonErrors::UnexpectedEOF)
        },
        Ok(_) => Ok(()),
        Err(err) => Err(CommonErrors::ReadFailed { err }),
    }
}

pub async fn write_message(
    stream: &mut TcpStream,
    buffer: &mut Vec<u8>,
) -> Result<(), CommonErrors> {
    match stream.write_all(buffer).await {
        Ok(_) => Ok(()),
        Err(err) => Err(CommonErrors::WriteFailed { err }),
    }
}
