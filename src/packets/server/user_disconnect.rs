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
