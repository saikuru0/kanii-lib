#![allow(dead_code)]
use std::str::FromStr;

trait Parsable {
    fn to_str(&self) -> String;
    fn from_str(packet: String) -> Self;
}

enum ClientPacket {
    Ping { user_id: String },
    Authentication {
        method: String,
        authkey: String },
    Message {
        user_id: String,
        message: String },
}

enum BadAuthReason {
    AuthFail,
    UserFail,
    SockFail,
    JoinFail,
}

impl FromStr for BadAuthReason {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authfail" => Ok(BadAuthReason::AuthFail),
            "joinfail" => Ok(BadAuthReason::JoinFail),
            "sockfail" => Ok(BadAuthReason::SockFail),
            "userfail" => Ok(BadAuthReason::UserFail),
            _ => Err(ParsePacketError::WrongFormat)
        }
    }
}

enum JoinAuthPacket {
    Join {
        timestamp: i64,
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        sequence_id: String },
    GoodAuth {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        channel_name: String,
        max_msg_length: i64 },
    BadAuth {
        reason: BadAuthReason,
        timestamp: i64 },
}

enum ChannelEventPacket {
    Creation {
        channel_name: String,
        is_protected: bool,
        is_temporary: bool },
    Update {
        channel_name: String,
        new_name: String,
        is_protected: bool,
        is_temporary: bool },
    Deletion { channel_name: String },
}

struct Color {
    content: String
}

impl Color {
    fn as_hex() -> String {
        todo!()
    }
    fn as_shex() -> String {
        todo!()
    }
    fn raw() -> String {
        todo!()
    }
    fn as_rgb() -> (i8, i8, i8) {
        todo!()
    }
    fn as_hsl() -> (i8, f32, f32) {
        todo!()
    }
}

impl FromStr for Color {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct UserPermissions {
    rank: u8,
    can_moderate: bool,
    can_logs: bool,
    can_nickname: bool,
    channel_permissions: u8,
}

impl FromStr for UserPermissions {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

enum ChannelSwitchPacket {
    Join {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        sequence_id: String },
    Departure {
        user_id: String,
        sequence_id: String },
    ForcedSwitch { channel_name: String },
}

enum DisconnectReason {
    Leave,
    Timeout,
    Kick,
    Flood,
}

impl FromStr for DisconnectReason {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "leave" => Ok(DisconnectReason::Leave),
            "kick" => Ok(DisconnectReason::Kick),
            "flood" => Ok(DisconnectReason::Flood),
            "timeout" => Ok(DisconnectReason::Timeout),
            _ => Err(ParsePacketError::WrongFormat)
        }
    }
}

struct UserContext {
    user_id: String,
    username: String,
    color: Color,
    user_permissions: UserPermissions,
    visible: bool,
}

struct MessageFlags {
    bold: bool,
    cursive: bool,
    underlined: bool,
    colon: bool,
    private: bool,
}

impl FromStr for MessageFlags {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct ChannelContext {
    channel_name: String,
    password_protected: bool,
    temporary: bool,
}

enum ContextInformationPacket {
    ExistingUsers {
        count: i64,
        contexts: Vec<UserContext> },
    ExistingMessage {
        timestamp: i64,
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        message: String,
        sequence_id: String,
        notify: bool,
        message_flags: MessageFlags },
    Channels {
        count: i64,
        contexts: Vec<ChannelContext> },
}

struct ClearedContext {
    message_list: bool,
    user_list: bool,
    channel_list: bool,
}

enum ServerPacket {
    Pong { text: String },
    JoinAuth(JoinAuthPacket),
    ChatMessage {
        timestamp: i64,
        user_id: String,
        message: String,
        sequence_id: String,
        message_flags: MessageFlags },
    UserDisconnect {
        user_id: String,
        username: String,
        reason: DisconnectReason,
        timestamp: i64,
        sequence_id: String },
    ChannelEvent(ChannelEventPacket),
    ChannelSwitch(ChannelSwitchPacket),
    MessageDeletion{ sequence_id: String },
    ContextInformation(ContextInformationPacket),
    ContextClearing(ClearedContext),
    ForcedDisconnect {
        ban: bool,
        timestamp: i64 },
    UserUpdate {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions },
}

enum Packet {
    Client(ClientPacket),
    Server(ServerPacket)
}

enum ParsePacketError {
    NonexistentType,
    FieldCount(usize),
    WrongFormat,
    Empty,
    FieldParsingFail,
}

impl FromStr for ClientPacket {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('\t').collect();
        if parts.is_empty() {
            return Err(ParsePacketError::Empty);
        }

        match parts[0] {
            "0" => {
                if parts.len() != 2 {
                    return Err(ParsePacketError::FieldCount(parts.len()-2));
                }
                Ok(
                    ClientPacket::Ping {
                        user_id: parts[1].to_string(),
                    }
                )
            },
            
            "1" => {
                if parts.len() != 3 {
                    return Err(ParsePacketError::FieldCount(parts.len()-3));
                }
                Ok(
                    ClientPacket::Authentication {
                        method: parts[1].to_string(),
                        authkey: parts[2].to_string(),
                    }
                )
            },
            
            "2" => {
                if parts.len() != 3 {
                    return Err(ParsePacketError::FieldCount(parts.len()-3));
                }
                Ok(
                    ClientPacket::Message {
                        user_id: parts[1].to_string(),
                        message: parts[2].to_string(),
                    }
                )
            }
            
            _ => Err(ParsePacketError::NonexistentType)
        }
    }
}

impl FromStr for ServerPacket {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('\t').collect();
        if parts.is_empty() {
            return Err(ParsePacketError::Empty);
        }

        match parts[0] {
            "0" => {
                if parts.len() != 2 {
                    return Err(ParsePacketError::FieldCount(parts.len()-2));
                }

                Ok(
                    ServerPacket::Pong {
                        text: parts[1].to_string(),
                    }
                )
            },

            "1" => {
                if parts.len() < 2 {
                    return Err(ParsePacketError::FieldCount(parts.len()-2));
                }

                match parts[1] {
                    "y" => {
                        if parts.len() != 8 {
                            return Err(ParsePacketError::FieldCount(parts.len()-8));
                        }

                        let color: Color;
                        match parts[4].parse::<Color>() {
                            Ok(content) => color = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }

                        let user_permissions: UserPermissions;
                        match parts[5].parse::<UserPermissions>() {
                            Ok(content) => user_permissions = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }

                        let max_msg_length: i64;
                        match parts[7].parse::<i64>() {
                            Ok(content) => max_msg_length = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }

                        Ok(
                            ServerPacket::JoinAuth(
                                JoinAuthPacket::GoodAuth {
                                    user_id: parts[2].to_string(),
                                    username: parts[3].to_string(),
                                    color,
                                    user_permissions,
                                    channel_name: parts[6].to_string(),
                                    max_msg_length
                                }
                            )
                        )
                    },

                    "n" => {
                        if parts.len() != 4 {
                            return Err(ParsePacketError::FieldCount(parts.len()-8));
                        }
                        let reason: BadAuthReason;
                        match parts[3].parse::<BadAuthReason>() {
                            Ok(content) => reason = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }
                        let timestamp: i64;
                        match parts[3].parse::<i64>() {
                            Ok(content) => timestamp = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }
                        Ok(
                            ServerPacket::JoinAuth(
                                JoinAuthPacket::BadAuth {
                                    reason,
                                    timestamp
                                }
                            )
                        )
                    },

                    _ => {
                        if parts.len() != 8 {
                            return Err(ParsePacketError::FieldCount(parts.len()-8));
                        }

                        let timestamp: i64;
                        match parts[2].parse::<i64>() {
                            Ok(content) => timestamp = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }

                        let color: Color;
                        match parts[5].parse::<Color>() {
                            Ok(content) => color = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }

                        let user_permissions: UserPermissions;
                        match parts[6].parse::<UserPermissions>() {
                            Ok(content) => user_permissions = content,
                            Err(..) => return Err(ParsePacketError::FieldParsingFail)
                        }

                        Ok(
                            ServerPacket::JoinAuth(
                                JoinAuthPacket::Join {
                                    timestamp,
                                    user_id: parts[3].to_string(),
                                    username: parts[4].to_string(),
                                    color,
                                    user_permissions,
                                    sequence_id: parts[7].to_string()
                                }
                            )
                        ) 
                    }
                }
            },

            "2" => {
                if parts.len() != 6 {
                    return Err(ParsePacketError::FieldCount(parts.len()-6));
                }

                let timestamp: i64;
                match parts[1].parse::<i64>() {
                    Ok(content) => timestamp = content,
                    Err(..) => return Err(ParsePacketError::FieldParsingFail)
                }

                let message_flags: MessageFlags;
                match parts[5].parse::<MessageFlags>() {
                    Ok(content) => message_flags = content,
                    Err(..) => return Err(ParsePacketError::FieldParsingFail)
                }

                Ok(
                    ServerPacket::ChatMessage {
                        timestamp,
                        user_id: parts[2].to_string(),
                        message: parts[3].to_string(),
                        sequence_id: parts[4].to_string(),
                        message_flags
                    }
                )
            },

            "3" => {
                if parts.len() != 6 {
                    return Err(ParsePacketError::FieldCount(parts.len()-6));
                }

                let reason: DisconnectReason;
                match parts[3].parse::<DisconnectReason>() {
                    Ok(content) => reason = content,
                    Err(..) => return Err(ParsePacketError::FieldParsingFail)
                }

                let timestamp: i64;
                match parts[4].parse::<i64>() {
                    Ok(content) => timestamp = content,
                    Err(..) => return Err(ParsePacketError::FieldParsingFail)
                }
                
                Ok(
                    ServerPacket::UserDisconnect {
                        user_id: parts[1].to_string(),
                        username: parts[2].to_string(),
                        reason,
                        timestamp,
                        sequence_id: parts[5].to_string()
                    }
                )
            },

            _ => Err(ParsePacketError::NonexistentType)
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// trait Transmission
// struct Client <T:transmision> {
//     connection
//     set_receive_handler
// }
