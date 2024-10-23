use crate::packets::types::*;

pub struct UserUpdatePacket {
    user_id: String,
    username: String,
    color: Color,
    user_permissions: UserPermissions,
}

impl FromParts for UserUpdatePacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap();
        let username = iter.next().unwrap();
        let color = iter.next().unwrap().parse::<Color>().unwrap();
        let user_permissions = iter.next().unwrap().parse::<UserPermissions>().unwrap();
        Ok(UserUpdatePacket {
            user_id,
            username,
            color,
            user_permissions,
        })
    }
}
