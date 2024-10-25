use super::{FromParts, Sockchatable};

#[derive(Debug)]
pub struct MessageDeletionPacket {
    sequence_id: String,
}

impl FromParts for MessageDeletionPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, super::ParsePacketError> {
        let mut iter = parts.into_iter();
        let sequence_id = iter.next().unwrap_or("default_sequence_id".to_string());
        Ok(MessageDeletionPacket { sequence_id })
    }
}

impl Sockchatable for MessageDeletionPacket {
    fn to_sockstr(&self) -> String {
        self.sequence_id.clone()
    }
}
