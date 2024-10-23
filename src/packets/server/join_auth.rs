use crate::packets::types::*;

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
                let user_id = iter.next().unwrap();
                let username = iter.next().unwrap();
                let color = iter.next().unwrap().parse::<Color>().unwrap();
                let user_permissions = iter.next().unwrap().parse::<UserPermissions>().unwrap();
                let channel_name = iter.next().unwrap();
                let max_msg_length = iter.next().unwrap().parse::<i64>().unwrap();
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
                let reason = iter.next().unwrap().parse::<BadAuthReason>().unwrap();
                let timestamp = iter.next().unwrap().parse::<i64>().unwrap();
                Ok(Self::BadAuth { reason, timestamp })
            }

            _ => {
                let timestamp = iter.next().unwrap().parse::<i64>().unwrap();
                let user_id = iter.next().unwrap();
                let username = iter.next().unwrap();
                let color = iter.next().unwrap().parse::<Color>().unwrap();
                let user_permissions = iter.next().unwrap().parse::<UserPermissions>().unwrap();
                let sequence_id = iter.next().unwrap();
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
