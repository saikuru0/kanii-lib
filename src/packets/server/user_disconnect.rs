use crate::packets::types::*;

#[derive(Debug)]
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
        let user_id = iter.next().unwrap_or("default_user_id".to_string());
        let username = iter.next().unwrap_or("default_username".to_string());
        let reason = iter.next().unwrap_or("default_reason".to_string()).parse::<DisconnectReason>().unwrap_or_default();
        let timestamp = iter.next().unwrap_or("default_timestamp".to_string()).parse::<i64>().unwrap_or(444);
        let sequence_id = iter.next().unwrap_or("default_sequence_id".to_string());
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
