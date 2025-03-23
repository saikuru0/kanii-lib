use super::{FromParts, Sockchatable};

#[derive(Debug)]
pub struct PongPacket {
    pub text: String,
}

impl FromParts for PongPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, super::ParsePacketError> {
        let mut iter = parts.into_iter();
        let text = iter.next().unwrap_or("default_text".to_string());
        Ok(PongPacket { text })
    }
}

impl Sockchatable for PongPacket {
    fn to_sockstr(&self) -> String {
        self.text.clone()
    }
}
