use crate::packets::types::*;

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
                let user_id = iter.next().unwrap();
                let username = iter.next().unwrap();
                let color = iter.next().unwrap().parse::<Color>().unwrap();
                let user_permissions = iter.next().unwrap().parse::<UserPermissions>().unwrap();
                let sequence_id = iter.next().unwrap();
                Ok(ChannelSwitchingPacket::Join {
                    user_id,
                    username,
                    color,
                    user_permissions,
                    sequence_id,
                })
            }

            "1" => {
                let user_id = iter.next().unwrap();
                let sequence_id = iter.next().unwrap();
                Ok(ChannelSwitchingPacket::Departure {
                    user_id,
                    sequence_id,
                })
            }

            "2" => {
                let channel_name = iter.next().unwrap();
                Ok(ChannelSwitchingPacket::ForcedSwitch { channel_name })
            }

            _ => Err(ParsePacketError::WrongFormat)
        }
    }
}