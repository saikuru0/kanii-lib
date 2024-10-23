use crate::packets::types::*;

pub struct UserDisconnectPacket {
    user_id: String,
    username: String,
    reason: DisconnectReason,
    timestamp: i64,
    sequence_id: String,
}

impl FromParts for UserDisconnectPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap();
        let username = iter.next().unwrap();
        let reason = iter.next().unwrap().parse::<DisconnectReason>().unwrap();
        let timestamp = iter.next().unwrap().parse::<i64>().unwrap();
        let sequence_id = iter.next().unwrap();
        Ok(UserDisconnectPacket {
            user_id,
            username,
            reason,
            timestamp,
            sequence_id,
        })
    }
}

impl Sockchatable for UserDisconnectPacket {
    fn to_sockstr(&self) -> String {
        vec![
            self.user_id.as_str(),
            self.username.as_str(),
            self.reason.to_sockstr().as_str(),
            self.timestamp.to_string().as_str(),
            self.sequence_id.as_str(),
        ]
        .join("\t")
    }
}
