use super::{FromParts, Sockchatable};

pub struct MessageDeletionPacket {
    sequence_id: String,
}

impl FromParts for MessageDeletionPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, super::ParsePacketError> {
        let mut iter = parts.into_iter();
        let sequence_id = iter.next().unwrap();
        Ok(MessageDeletionPacket { sequence_id })
    }
}

impl Sockchatable for MessageDeletionPacket {
    fn to_sockstr(&self) -> String {
        self.sequence_id.clone()
    }
}
