use super::{FromParts, Sockchatable};

pub struct PongPacket {
    text: String,
}

impl FromParts for PongPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, super::ParsePacketError> {
        let mut iter = parts.into_iter();
        let text = iter.next().unwrap();
        Ok(PongPacket { text })
    }
}

impl Sockchatable for PongPacket {
    fn to_sockstr(&self) -> String {
        self.text.clone()
    }
}
