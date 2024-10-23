use crate::packets::types::{FromParts, ParsePacketError, Sockchatable};

pub struct MessagePacket {
    pub user_id: String,
    pub message: String,
}

impl FromParts for MessagePacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap();
        let message = iter.next().unwrap();
        Ok(MessagePacket { user_id, message })
    }
}
impl Sockchatable for MessagePacket {
    fn to_sockstr(&self) -> String {
        vec![self.user_id.as_str(), self.message.as_str()].join("\t")
    }
}
