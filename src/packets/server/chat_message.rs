use crate::packets::types::*;

pub struct ChatMessagePacket {
    timestamp: i64,
    user_id: String,
    message: String,
    sequence_id: String,
    message_flags: MessageFlags,
}

impl FromParts for ChatMessagePacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let timestamp = iter.next().unwrap().parse::<i64>().unwrap();
        let user_id = iter.next().unwrap();
        let message = iter.next().unwrap();
        let sequence_id = iter.next().unwrap();
        let message_flags = iter.next().unwrap().parse::<MessageFlags>().unwrap();
        Ok(ChatMessagePacket { timestamp, user_id, message, sequence_id, message_flags })
    }
}
