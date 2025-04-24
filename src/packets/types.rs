use csscolorparser::{Color as CssColor, ParseColorError};
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
    pub value: String,
}

impl Color {
    fn parse(&self) -> Result<CssColor, ParseColorError> {
        CssColor::from_html(&self.value)
    }

    pub fn raw(&self) -> String {
        self.value.clone()
    }

    pub fn as_hex(&self) -> Result<String, ParseColorError> {
        let c = self.parse()?;
        Ok(format!(
            "#{:02X}{:02X}{:02X}",
            (c.r * 255.0).round() as u8,
            (c.g * 255.0).round() as u8,
            (c.b * 255.0).round() as u8
        ))
    }

    pub fn as_hexa(&self) -> Result<String, ParseColorError> {
        let c = self.parse()?;
        Ok(format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (c.r * 255.0).round() as u8,
            (c.g * 255.0).round() as u8,
            (c.b * 255.0).round() as u8,
            (c.a * 255.0).round() as u8
        ))
    }

    pub fn as_shex(&self) -> Result<String, ParseColorError> {
        let hex = self.as_hex()?;
        let rr = &hex[1..3];
        let gg = &hex[3..5];
        let bb = &hex[5..7];
        if rr.chars().nth(0) == rr.chars().nth(1)
            && gg.chars().nth(0) == gg.chars().nth(1)
            && bb.chars().nth(0) == bb.chars().nth(1)
        {
            Ok(format!("#{}{}{}", &rr[0..1], &gg[0..1], &bb[0..1]))
        } else {
            Ok(hex)
        }
    }

    pub fn as_shexa(&self) -> Result<String, ParseColorError> {
        let hexa = self.as_hexa()?;
        let rr = &hexa[1..3];
        let gg = &hexa[3..5];
        let bb = &hexa[5..7];
        let aa = &hexa[7..9];
        if rr.chars().nth(0) == rr.chars().nth(1)
            && gg.chars().nth(0) == gg.chars().nth(1)
            && bb.chars().nth(0) == bb.chars().nth(1)
            && aa.chars().nth(0) == aa.chars().nth(1)
        {
            Ok(format!(
                "#{}{}{}{}",
                &rr[0..1],
                &gg[0..1],
                &bb[0..1],
                &aa[0..1]
            ))
        } else {
            Ok(hexa)
        }
    }

    pub fn as_rgb(&self) -> Result<[u8; 3], ParseColorError> {
        let c = self.parse()?;
        Ok([
            (c.r * 255.0).round() as u8,
            (c.g * 255.0).round() as u8,
            (c.b * 255.0).round() as u8,
        ])
    }

    pub fn as_rgba(&self) -> Result<[u8; 4], ParseColorError> {
        let c = self.parse()?;
        Ok([
            (c.r * 255.0).round() as u8,
            (c.g * 255.0).round() as u8,
            (c.b * 255.0).round() as u8,
            (c.a * 255.0).round() as u8,
        ])
    }

    pub fn as_hsl(&self) -> Result<(i16, f32, f32), ParseColorError> {
        let c = self.parse()?;
        let (h, s, l) = rgb_to_hsl(c.r, c.g, c.b);
        Ok(((h * 360.0).round() as i16, s, l))
    }
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    let s = if d == 0.0 {
        0.0
    } else {
        d / (1.0 - (2.0 * l - 1.0).abs())
    };
    let h = if d == 0.0 {
        0.0
    } else if max == r {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    (h, s, l)
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
