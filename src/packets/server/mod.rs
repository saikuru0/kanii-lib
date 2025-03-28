pub mod channel_event;
pub mod channel_switching;
pub mod chat_message;
pub mod context_clearing;
pub mod context_information;
pub mod forced_disconnect;
pub mod join_auth;
pub mod message_deletion;
pub mod pong;
pub mod user_disconnect;
pub mod user_update;

use crate::packets::types::*;

pub use channel_event::ChannelEventPacket;
pub use channel_switching::ChannelSwitchingPacket;
pub use chat_message::ChatMessagePacket;
pub use context_clearing::ContextClearingPacket;
pub use context_information::ContextInformationPacket;
pub use forced_disconnect::ForcedDisconnectPacket;
pub use join_auth::JoinAuthPacket;
pub use message_deletion::MessageDeletionPacket;
pub use pong::PongPacket;
pub use user_disconnect::UserDisconnectPacket;
pub use user_update::UserUpdatePacket;

use std::str::FromStr;

#[derive(Debug)]
pub enum ServerPacket {
    Pong(PongPacket),
    JoinAuth(JoinAuthPacket),
    ChatMessage(ChatMessagePacket),
    UserDisconnect(UserDisconnectPacket),
    ChannelEvent(ChannelEventPacket),
    ChannelSwitching(ChannelSwitchingPacket),
    MessageDeletion(MessageDeletionPacket),
    ContextInformation(ContextInformationPacket),
    ContextClearing(ContextClearingPacket),
    ForcedDisconnect(ForcedDisconnectPacket),
    UserUpdate(UserUpdatePacket),
}

impl FromStr for ServerPacket {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\t').map(str::to_string).collect::<Vec<String>>();
        if parts.is_empty() {
            return Err(ParsePacketError::Empty);
        }

        let first_part = parts.remove(0);
        match first_part.as_str() {
            "0" => Ok(ServerPacket::Pong(PongPacket::from_parts(parts).unwrap())),
            "1" => Ok(ServerPacket::JoinAuth(
                JoinAuthPacket::from_parts(parts).unwrap(),
            )),
            "2" => Ok(ServerPacket::ChatMessage(
                ChatMessagePacket::from_parts(parts).unwrap(),
            )),
            "3" => Ok(ServerPacket::UserDisconnect(
                UserDisconnectPacket::from_parts(parts).unwrap(),
            )),
            "4" => Ok(ServerPacket::ChannelEvent(
                ChannelEventPacket::from_parts(parts).unwrap(),
            )),
            "5" => Ok(ServerPacket::ChannelSwitching(
                ChannelSwitchingPacket::from_parts(parts).unwrap(),
            )),
            "6" => Ok(ServerPacket::MessageDeletion(
                MessageDeletionPacket::from_parts(parts).unwrap(),
            )),
            "7" => Ok(ServerPacket::ContextInformation(
                ContextInformationPacket::from_parts(parts).unwrap(),
            )),
            "8" => Ok(ServerPacket::ContextClearing(
                ContextClearingPacket::from_parts(parts).unwrap(),
            )),
            "9" => Ok(ServerPacket::ForcedDisconnect(
                ForcedDisconnectPacket::from_parts(parts).unwrap(),
            )),
            "10" => Ok(ServerPacket::UserUpdate(
                UserUpdatePacket::from_parts(parts).unwrap(),
            )),
            _ => Err(ParsePacketError::WrongFormat),
        }
    }
}

impl Sockchatable for ServerPacket {
    fn to_sockstr(&self) -> String {
        match self {
            Self::Pong(packet) => vec!["0", packet.to_sockstr().as_str()].join("\t"),
            Self::JoinAuth(packet) => vec!["1", packet.to_sockstr().as_str()].join("\t"),
            Self::ChatMessage(packet) => vec!["2", packet.to_sockstr().as_str()].join("\t"),
            Self::UserDisconnect(packet) => vec!["3", packet.to_sockstr().as_str()].join("\t"),
            Self::ChannelEvent(packet) => vec!["4", packet.to_sockstr().as_str()].join("\t"),
            Self::ChannelSwitching(packet) => vec!["5", packet.to_sockstr().as_str()].join("\t"),
            Self::MessageDeletion(packet) => vec!["6", packet.to_sockstr().as_str()].join("\t"),
            Self::ContextInformation(packet) => vec!["7", packet.to_sockstr().as_str()].join("\t"),
            Self::ContextClearing(packet) => vec!["8", packet.to_sockstr().as_str()].join("\t"),
            Self::ForcedDisconnect(packet) => vec!["9", packet.to_sockstr().as_str()].join("\t"),
            Self::UserUpdate(packet) => vec!["10", packet.to_sockstr().as_str()].join("\t"),
        }
    }
}
