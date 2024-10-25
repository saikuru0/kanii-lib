use super::FromParts;
use crate::packets::types::*;

#[derive(Debug)]
pub struct ForcedDisconnectPacket {
    ban: bool,
    timestamp: i64,
}

impl FromParts for ForcedDisconnectPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let ban = iter.next().unwrap_or("default_ban".to_string()).parse_sockbool().unwrap_or(false);
        let timestamp = iter.next().unwrap_or("default_timestamp".to_string()).parse::<i64>().unwrap_or(444);
        Ok(ForcedDisconnectPacket { ban, timestamp })
    }
}

impl Sockchatable for ForcedDisconnectPacket {
    fn to_sockstr(&self) -> String {
        vec![
            self.ban.to_sockstr().as_str(),
            self.timestamp.to_string().as_str(),
        ]
        .join("\t")
    }
}
