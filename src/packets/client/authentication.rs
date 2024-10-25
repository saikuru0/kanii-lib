use crate::packets::types::{FromParts, ParsePacketError, Sockchatable};

#[derive(Debug)]
pub struct AuthenticationPacket {
    pub method: String,
    pub authkey: String,
}

impl FromParts for AuthenticationPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let method = iter.next().unwrap_or("default_method".to_string());
        let authkey = iter.next().unwrap_or("default_authkey".to_string());
        Ok(AuthenticationPacket { method, authkey })
    }
}

impl Sockchatable for AuthenticationPacket {
    fn to_sockstr(&self) -> String {
        vec![self.method.as_str(), self.authkey.as_str()].join("\t")
    }
}
