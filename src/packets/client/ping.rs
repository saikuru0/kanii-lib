use crate::packets::types::{FromParts, ParsePacketError};

pub struct PingPacket {
    user_id: String,
}

impl FromParts for PingPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap();
        Ok(PingPacket { user_id })
    }
}
