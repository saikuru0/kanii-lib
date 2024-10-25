pub mod authentication;
pub mod message;
pub mod ping;

use std::str::FromStr;

use authentication::AuthenticationPacket;
use message::MessagePacket;
use ping::PingPacket;

use super::types::*;

#[derive(Debug)]
pub enum ClientPacket {
    Ping(PingPacket),
    Authentication(AuthenticationPacket),
    Message(MessagePacket),
}

impl FromStr for ClientPacket {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\t').map(str::to_string).collect::<Vec<String>>();
        if parts.is_empty() {
            return Err(ParsePacketError::Empty);
        }

        let first_part = parts.remove(0);
        match first_part.as_str() {
            "0" => Ok(ClientPacket::Ping(PingPacket::from_parts(parts).unwrap())),
            "1" => Ok(ClientPacket::Authentication(
                AuthenticationPacket::from_parts(parts).unwrap(),
            )),
            "2" => Ok(ClientPacket::Message(
                MessagePacket::from_parts(parts).unwrap(),
            )),
            _ => Err(ParsePacketError::WrongFormat),
        }
    }
}

impl Sockchatable for ClientPacket {
    fn to_sockstr(&self) -> String {
        match self {
            Self::Ping(packet) => vec!["0", packet.to_sockstr().as_str()].join("\t"),
            Self::Authentication(packet) => vec!["1", packet.to_sockstr().as_str()].join("\t"),
            Self::Message(packet) => vec!["2", packet.to_sockstr().as_str()].join("\t"),
        }
    }
}
