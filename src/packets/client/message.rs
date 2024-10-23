use crate::packets::types::{FromParts, ParsePacketError};

pub struct MessagePacket {
    user_id: String,
    message: String,
}

impl FromParts for MessagePacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap();
        let message = iter.next().unwrap();
        Ok(MessagePacket { user_id, message })
    }
}
