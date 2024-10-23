use super::FromParts;
use crate::packets::types::*;

pub struct ForcedDisconnectPacket {
    ban: bool,
    timestamp: i64,
}

impl FromParts for ForcedDisconnectPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let ban = iter.next().unwrap().parse::<bool>().unwrap();
        let timestamp = iter.next().unwrap().parse::<i64>().unwrap();
        Ok(ForcedDisconnectPacket { ban, timestamp })
    }
}
