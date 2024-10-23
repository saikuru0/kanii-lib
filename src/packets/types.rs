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
        match self {
            Self::AuthFail => "authfail".to_string(),
            Self::JoinFail => "joinfail".to_string(),
            Self::SockFail => "sockfail".to_string(),
            Self::UserFail => "userfail".to_string(),
        }
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(|c| {
                c == char::from_u32(0x0C).expect("feed split failed")
                    || c == char::from_u32(0x20).expect("space split failed")
            })
            .map(str::to_string)
            .collect::<Vec<String>>();
        let mut iter = parts.into_iter();
        Ok(UserPermissions {
            rank: iter.next().unwrap().parse::<u8>().unwrap(),
            can_moderate: iter.next().unwrap().parse::<bool>().unwrap(),
            can_logs: iter.next().unwrap().parse::<bool>().unwrap(),
            can_nickname: iter.next().unwrap().parse::<bool>().unwrap(),
            channel_permissions: iter.next().unwrap().parse::<u8>().unwrap(),
        })
    }
}

impl Sockchatable for UserPermissions {
    fn to_sockstr(&self) -> String {
        vec![
            self.rank.to_string().as_str(),
            self.can_moderate.to_string().as_str(),
            self.can_logs.to_string().as_str(),
            self.can_nickname.to_string().as_str(),
            self.channel_permissions.to_string().as_str(),
        ]
        .join("\t")
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.to_string();
        Ok(MessageFlags {
            bold: input.remove(0).to_string().parse::<bool>().unwrap(),
            cursive: input.remove(0).to_string().parse::<bool>().unwrap(),
            underlined: input.remove(0).to_string().parse::<bool>().unwrap(),
            colon: input.remove(0).to_string().parse::<bool>().unwrap(),
            private: input.remove(0).to_string().parse::<bool>().unwrap(),
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
            self.visible.to_string().as_str(),
        ]
        .join("\t")
    }
}

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

pub struct ChannelContext {
    pub channel_name: String,
    pub password_protected: bool,
    pub temporary: bool,
}

impl Sockchatable for ChannelContext {
    fn to_sockstr(&self) -> String {
        vec![
            self.channel_name.as_str(),
            self.password_protected.to_string().as_str(),
            self.temporary.to_string().as_str(),
        ]
        .join("\t")
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
