use std::net::Ipv4Addr;

use crate::bgp_type::AutonomousSystemNumber;
use crate::packets::header::{Header, MessageType};
use crate::packets::keepalive::KeepaliveMessage;

use bytes::BytesMut;

use crate::error::{
    ConvertBgpMessageToBytesError, ConvertBytesToBgpMessageError,
};
use crate::packets::open::OpenMessage;

use super::{header, keepalive};

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Message {
    Open(OpenMessage),
    Keepalive(KeepaliveMessage),
}

impl TryFrom<BytesMut> for Message {
    type Error = ConvertBytesToBgpMessageError;

    fn try_from(bytes: BytesMut) -> Result<Self, Self::Error> {
        let header_bytes_length = 19;

        if bytes.len() < header_bytes_length {
            return Err(Self::Error::from(anyhow::anyhow!(
                "BytesからMessageに変換できませんでした。Bytesの長さが最小の長さより短いです。"
            )));
        };

        let header = Header::try_from(BytesMut::from(&bytes[0..header_bytes_length]))?;
        match header.type_ {
            MessageType::Open => {
                Ok(Message::Open(OpenMessage::try_from(bytes)?))
            }
            MessageType::Keepalive => {
                Ok(Message::Keepalive(KeepaliveMessage::try_from(bytes)?))
            }
        }
    }
}

impl From<Message> for BytesMut {
    fn from(message: Message) -> BytesMut {
        match message {
            Message::Open(open) => open.into(),
            Message::Keepalive(keepalive) => keepalive.into(),
        }
    }
}

impl Message {
    pub fn new_open(
        my_as_number: AutonomousSystemNumber,
        my_ip_addr: Ipv4Addr,
    ) -> Self {
        Self::Open(OpenMessage::new(my_as_number, my_ip_addr))
    }

    pub fn new_keepalive() -> Self {
        Self::Keepalive(KeepaliveMessage::new())
    }
}

