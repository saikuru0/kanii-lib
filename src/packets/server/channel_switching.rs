use crate::packets::types::*;

#[derive(Debug)]
pub enum ChannelSwitchingPacket {
    Join {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        sequence_id: String,
    },
    Departure {
        user_id: String,
        sequence_id: String,
    },
    ForcedSwitch {
        channel_name: String,
    },
}

impl FromParts for ChannelSwitchingPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        match iter.next().unwrap().as_str() {
            "0" => {
                let user_id = iter.next().unwrap_or("default_user_id".to_string());
                let username = iter.next().unwrap_or("default_username".to_string());
                let color = iter.next().unwrap_or("default_color".to_string()).parse::<Color>().unwrap_or_default();
                let user_permissions = iter.next().unwrap_or("default_user_permissions".to_string()).parse::<UserPermissions>().unwrap_or_default();
                let sequence_id = iter.next().unwrap_or("default_sequence_id".to_string());
                Ok(ChannelSwitchingPacket::Join {
                    user_id,
                    username,
                    color,
                    user_permissions,
                    sequence_id,
                })
            }

            "1" => {
                let user_id = iter.next().unwrap_or("default_user_id".to_string());
                let sequence_id = iter.next().unwrap_or("default_sequence_id".to_string());
                Ok(ChannelSwitchingPacket::Departure {
                    user_id,
                    sequence_id,
                })
            }

            "2" => {
                let channel_name = iter.next().unwrap_or("default_channel_name".to_string());
                Ok(ChannelSwitchingPacket::ForcedSwitch { channel_name })
            }

            _ => Err(ParsePacketError::WrongFormat),
        }
    }
}

impl Sockchatable for ChannelSwitchingPacket {
    fn to_sockstr(&self) -> String {
        match self {
            Self::Join {
                user_id,
                username,
                color,
                user_permissions,
                sequence_id,
            } => vec![
                user_id.as_str(),
                username.as_str(),
                color.to_sockstr().as_str(),
                user_permissions.to_sockstr().as_str(),
                sequence_id.as_str(),
            ]
            .join("\t"),

            Self::Departure {
                user_id,
                sequence_id,
            } => vec![user_id.as_str(), sequence_id.as_str()].join("\t"),

            Self::ForcedSwitch { channel_name } => channel_name.clone(),
        }
    }
}
