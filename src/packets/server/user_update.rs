use crate::packets::types::*;

#[derive(Debug)]
pub struct UserUpdatePacket {
    pub user_id: String,
    pub username: String,
    pub color: Color,
    pub user_permissions: UserPermissions,
}

impl FromParts for UserUpdatePacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        let user_id = iter.next().unwrap_or("default_user_id".to_string());
        let username = iter.next().unwrap_or("default_username".to_string());
        let color = iter.next().unwrap_or("default_color".to_string()).parse::<Color>().unwrap_or_default();
        let user_permissions = iter.next().unwrap_or("default_user_permissions".to_string()).parse::<UserPermissions>().unwrap_or_default();
        Ok(UserUpdatePacket {
            user_id,
            username,
            color,
            user_permissions,
        })
    }
}

impl Sockchatable for UserUpdatePacket {
    fn to_sockstr(&self) -> String {
        vec![
            self.user_id.as_str(),
            self.username.as_str(),
            self.color.to_sockstr().as_str(),
            self.user_permissions.to_sockstr().as_str(),
        ]
        .join("\t")
    }
}
