use std::str::FromStr;

#[derive(Debug)]
pub enum ParsePacketError {
    WrongFormat,
    Empty,
    FieldParsingFail,
}

pub enum BadAuthReason {
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
            _ => Err(ParsePacketError::WrongFormat),
        }
    }
}

impl Sockchatable for BadAuthReason {
    fn to_sockstr(&self) -> String {
        todo!()
    }
}

pub struct UserPermissions {
    pub rank: u8,
    pub can_moderate: bool,
    pub can_logs: bool,
    pub can_nickname: bool,
    pub channel_permissions: u8,
}

impl FromStr for UserPermissions {
    type Err = ParsePacketError;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Sockchatable for UserPermissions {
    fn to_sockstr(&self) -> String {
        todo!()
    }
}

pub struct MessageFlags {
    pub bold: bool,
    pub cursive: bool,
    pub underlined: bool,
    pub colon: bool,
    pub private: bool,
}

impl FromStr for MessageFlags {
    type Err = ParsePacketError;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Sockchatable for MessageFlags {
    fn to_sockstr(&self) -> String {
        todo!()
    }
}

pub enum DisconnectReason {
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
            _ => Err(ParsePacketError::WrongFormat),
        }
    }
}

impl Sockchatable for DisconnectReason {
    fn to_sockstr(&self) -> String {
        todo!()
    }
}

pub struct UserContext {
    pub user_id: String,
    pub username: String,
    pub color: Color,
    pub user_permissions: UserPermissions,
    pub visible: bool,
}

impl Sockchatable for UserContext {
    fn to_sockstr(&self) -> String {
        todo!()
    }
}

pub struct Color {
    content: String,
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
    fn as_hsl() -> (i16, f32, f32) {
        todo!()
    }
}

impl FromStr for Color {
    type Err = ParsePacketError;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Sockchatable for Color {
    fn to_sockstr(&self) -> String {
        todo!()
    }
}

pub struct ChannelContext {
    pub channel_name: String,
    pub password_protected: bool,
    pub temporary: bool,
}

impl Sockchatable for ChannelContext {
    fn to_sockstr(&self) -> String {
        todo!()
    }
}

pub trait FromParts {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError>
    where
        Self: Sized;
}

pub trait Sockchatable {
    fn to_sockstr(&self) -> String;
}
