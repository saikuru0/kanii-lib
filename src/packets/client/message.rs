use crate::packets::types::{FromParts, ParsePacketError, Sockchatable};

#[derive(Debug)]
pub struct MessagePacket {
    pub user_id: String,
    pub message: String,
}

impl FromParts for MessagePacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap_or("default_user_id".to_string());
        let message = iter.next().unwrap_or("default_message".to_string());
        Ok(MessagePacket { user_id, message })
    }
}
impl Sockchatable for MessagePacket {
    fn to_sockstr(&self) -> String {
        vec![self.user_id.as_str(), self.message.as_str()].join("\t")
    }
}
