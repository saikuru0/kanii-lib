use crate::packets::types::{FromParts, ParsePacketError, Sockchatable};

#[derive(Debug)]
pub struct PingPacket {
    pub user_id: String,
}

impl FromParts for PingPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap_or("default_user_id".to_string());
        Ok(PingPacket { user_id })
    }
}

impl Sockchatable for PingPacket {
    fn to_sockstr(&self) -> String {
        self.user_id.clone()
    }
}
