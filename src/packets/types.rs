use std::str::FromStr;

#[derive(Debug)]
pub enum ParsePacketError {
    WrongFormat,
    Empty,
    FieldParsingFail,
}

#[derive(Debug)]
pub enum BadAuthReason {
    AuthFail,
    UserFail,
    SockFail,
    JoinFail,
}

impl std::fmt::Display for BadAuthReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadAuthReason::AuthFail => {
                write!(f, "AuthFail")
            }
            BadAuthReason::UserFail => {
                write!(f, "UserFail")
            }
            BadAuthReason::SockFail => {
                write!(f, "SockFail")
            }
            BadAuthReason::JoinFail => {
                write!(f, "JoinFail")
            }
        }
    }
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
        match self {
            Self::AuthFail => "authfail".to_string(),
            Self::JoinFail => "joinfail".to_string(),
            Self::SockFail => "sockfail".to_string(),
            Self::UserFail => "userfail".to_string(),
        }
    }
}

impl Default for BadAuthReason {
    fn default() -> Self {
        BadAuthReason::SockFail
    }
}

#[derive(Debug)]
pub struct UserPermissions {
    pub rank: u8,
    pub can_moderate: bool,
    pub can_logs: bool,
    pub can_nickname: bool,
    pub channel_permissions: u8,
}

impl FromStr for UserPermissions {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .split(|c| c == 0x0C as char || c == 0x20 as char)
            .map(str::to_string);

        let rank = iter.next().and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);
        let can_moderate = iter
            .next()
            .and_then(|s| s.parse_sockbool().ok())
            .unwrap_or(false);
        let can_logs = iter
            .next()
            .and_then(|s| s.parse_sockbool().ok())
            .unwrap_or(false);
        let can_nickname = iter
            .next()
            .and_then(|s| s.parse_sockbool().ok())
            .unwrap_or(false);
        let channel_permissions = iter.next().and_then(|s| s.parse::<u8>().ok()).unwrap_or(0);

        Ok(UserPermissions {
            rank,
            can_moderate,
            can_logs,
            can_nickname,
            channel_permissions,
        })
    }
}

impl Sockchatable for UserPermissions {
    fn to_sockstr(&self) -> String {
        vec![
            self.rank.to_string().as_str(),
            self.can_moderate.to_sockstr().as_str(),
            self.can_logs.to_sockstr().as_str(),
            self.can_nickname.to_sockstr().as_str(),
            self.channel_permissions.to_string().as_str(),
        ]
        .join(" ")
    }
}

impl Default for UserPermissions {
    fn default() -> Self {
        UserPermissions {
            rank: 0,
            can_moderate: false,
            can_logs: false,
            can_nickname: false,
            channel_permissions: 0,
        }
    }
}

#[derive(Debug)]
pub struct MessageFlags {
    pub bold: bool,
    pub cursive: bool,
    pub underlined: bool,
    pub colon: bool,
    pub private: bool,
}

impl FromStr for MessageFlags {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.to_string();
        Ok(MessageFlags {
            bold: input
                .remove(0)
                .to_string()
                .parse_sockbool()
                .unwrap_or(false),
            cursive: input
                .remove(0)
                .to_string()
                .parse_sockbool()
                .unwrap_or(false),
            underlined: input
                .remove(0)
                .to_string()
                .parse_sockbool()
                .unwrap_or(false),
            colon: input
                .remove(0)
                .to_string()
                .parse_sockbool()
                .unwrap_or(false),
            private: input
                .remove(0)
                .to_string()
                .parse_sockbool()
                .unwrap_or(false),
        })
    }
}

impl Sockchatable for MessageFlags {
    fn to_sockstr(&self) -> String {
        let mut output = String::new();
        for flag in [
            self.bold,
            self.cursive,
            self.underlined,
            self.colon,
            self.private,
        ] {
            if flag {
                output.push_str("1");
            } else {
                output.push_str("0");
            }
        }
        output
    }
}

impl Default for MessageFlags {
    fn default() -> Self {
        MessageFlags {
            bold: false,
            cursive: false,
            underlined: false,
            colon: false,
            private: false,
        }
    }
}

#[derive(Debug)]
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
        match self {
            Self::Leave => "leave".to_string(),
            Self::Kick => "kick".to_string(),
            Self::Flood => "flood".to_string(),
            Self::Timeout => "timeout".to_string(),
        }
    }
}

impl Default for DisconnectReason {
    fn default() -> Self {
        DisconnectReason::Leave
    }
}

#[derive(Debug)]
pub struct UserContext {
    pub user_id: String,
    pub username: String,
    pub color: Color,
    pub user_permissions: UserPermissions,
    pub visible: bool,
}

impl Sockchatable for UserContext {
    fn to_sockstr(&self) -> String {
        vec![
            self.user_id.as_str(),
            self.username.as_str(),
            self.color.to_sockstr().as_str(),
            self.user_permissions.to_sockstr().as_str(),
            self.visible.to_sockstr().as_str(),
        ]
        .join("\t")
    }
}

impl Default for UserContext {
    fn default() -> Self {
        UserContext {
            user_id: "default_user_id".to_string(),
            username: "default_username".to_string(),
            color: Color::default(),
            user_permissions: UserPermissions::default(),
            visible: false,
        }
    }
}

#[derive(Debug)]
pub struct Color {
    value: String,
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.to_string();
        Ok(Color { value })
    }
}

impl Sockchatable for Color {
    fn to_sockstr(&self) -> String {
        self.value.clone()
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            value: "FFF".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ChannelContext {
    pub channel_name: String,
    pub password_protected: bool,
    pub temporary: bool,
}

impl Sockchatable for ChannelContext {
    fn to_sockstr(&self) -> String {
        vec![
            self.channel_name.as_str(),
            self.password_protected.to_sockstr().as_str(),
            self.temporary.to_sockstr().as_str(),
        ]
        .join("\t")
    }
}

impl Default for ChannelContext {
    fn default() -> Self {
        ChannelContext {
            channel_name: "default_channel_name".to_string(),
            password_protected: false,
            temporary: false,
        }
    }
}

pub trait ParseSockBool {
    fn parse_sockbool(&self) -> Result<bool, ParsePacketError>;
}

impl ParseSockBool for String {
    fn parse_sockbool(&self) -> Result<bool, ParsePacketError> {
        match self.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(ParsePacketError::FieldParsingFail),
        }
    }
}

impl Sockchatable for bool {
    fn to_sockstr(&self) -> String {
        match self {
            false => "0".to_string(),
            true => "1".to_string(),
        }
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
