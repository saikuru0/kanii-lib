use crate::packets::types::*;

#[derive(Debug)]
pub struct ChatMessagePacket {
    pub timestamp: i64,
    pub user_id: String,
    pub message: String,
    pub sequence_id: String,
    pub message_flags: MessageFlags,
}

impl FromParts for ChatMessagePacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let timestamp = iter
            .next()
            .unwrap_or("default_timestamp".to_string())
            .parse::<i64>()
            .unwrap_or(444);
        let user_id = iter.next().unwrap_or("default_user_id".to_string());
        let message = iter.next().unwrap_or("default_message".to_string());
        let sequence_id = iter.next().unwrap_or("default_sequence_id".to_string());
        let message_flags = iter
            .next()
            .unwrap_or("default_message_flags".to_string())
            .parse::<MessageFlags>()
            .unwrap_or_default();
        Ok(ChatMessagePacket {
            timestamp,
            user_id,
            message,
            sequence_id,
            message_flags,
        })
    }
}

impl Sockchatable for ChatMessagePacket {
    fn to_sockstr(&self) -> String {
        vec![
            self.timestamp.to_string().as_str(),
            self.user_id.as_str(),
            self.message.as_str(),
            self.sequence_id.as_str(),
            self.message_flags.to_sockstr().as_str(),
        ]
        .join("\t")
    }
}
