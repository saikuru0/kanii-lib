use crate::packets::types::*;

#[derive(Debug)]
pub enum JoinAuthPacket {
    GoodAuth {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        channel_name: String,
        max_msg_length: i64,
    },
    BadAuth {
        reason: BadAuthReason,
        timestamp: i64,
    },
    Join {
        timestamp: i64,
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        sequence_id: String,
    },
}

impl FromParts for JoinAuthPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        match iter.next().unwrap().as_str() {
            "y" => {
                let user_id = iter.next().unwrap_or("default_user_id".to_string());
                let username = iter.next().unwrap_or("default_username".to_string());
                let color = iter.next().unwrap_or("default_color".to_string()).parse::<Color>().unwrap_or_default();
                let user_permissions = iter.next().unwrap_or("default_user_permissions".to_string()).parse::<UserPermissions>().unwrap_or_default();
                let channel_name = iter.next().unwrap_or("default_channel_name".to_string());
                let max_msg_length = iter.next().unwrap_or("default_max_msg_length".to_string()).parse::<i64>().unwrap_or(444);
                Ok(Self::GoodAuth {
                    user_id,
                    username,
                    color,
                    user_permissions,
                    channel_name,
                    max_msg_length,
                })
            }

            "n" => {
                let reason = iter.next().unwrap_or("default_reason".to_string()).parse::<BadAuthReason>().unwrap_or_default();
                let timestamp = iter.next().unwrap_or("default_timestamp".to_string()).parse::<i64>().unwrap_or(444);
                Ok(Self::BadAuth { reason, timestamp })
            }

            _ => {
                let timestamp = iter.next().unwrap_or("default_timestamp".to_string()).parse::<i64>().unwrap_or(444);
                let user_id = iter.next().unwrap_or("default_user_id".to_string());
                let username = iter.next().unwrap_or("default_username".to_string());
                let color = iter.next().unwrap_or("default_color".to_string()).parse::<Color>().unwrap_or_default();
                let user_permissions = iter.next().unwrap_or("default_user_permissions".to_string()).parse::<UserPermissions>().unwrap_or_default();
                let sequence_id = iter.next().unwrap_or("default_sequence_id".to_string());
                Ok(Self::Join {
                    timestamp,
                    user_id,
                    username,
                    color,
                    user_permissions,
                    sequence_id,
                })
            }
        }
    }
}

impl Sockchatable for JoinAuthPacket {
    fn to_sockstr(&self) -> String {
        match self {
            Self::GoodAuth {
                user_id,
                username,
                color,
                user_permissions,
                channel_name,
                max_msg_length,
            } => vec![
                "y",
                user_id.as_str(),
                username.as_str(),
                color.to_sockstr().as_str(),
                user_permissions.to_sockstr().as_str(),
                channel_name.as_str(),
                max_msg_length.to_string().as_str(),
            ]
            .join("\t"),

            Self::BadAuth { reason, timestamp } => vec![
                "n",
                reason.to_sockstr().as_str(),
                timestamp.to_string().as_str(),
            ]
            .join("\t"),

            Self::Join {
                timestamp,
                user_id,
                username,
                color,
                user_permissions,
                sequence_id,
            } => vec![
                timestamp.to_string().as_str(),
                user_id.as_str(),
                username.as_str(),
                color.to_sockstr().as_str(),
                user_permissions.to_sockstr().as_str(),
                sequence_id.as_str(),
            ]
            .join("\t"),
        }
    }
}
