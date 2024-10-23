use crate::packets::types::{FromParts, ParsePacketError};

pub struct AuthenticationPacket {
    method: String,
    authkey: String,
}

impl FromParts for AuthenticationPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let method = iter.next().unwrap();
        let authkey = iter.next().unwrap();
        Ok(AuthenticationPacket { method, authkey })
    }
}
