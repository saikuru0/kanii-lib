use crate::packets::types::*;

#[derive(Debug)]
pub enum ContextInformationPacket {
    ExistingUsers {
        count: i64,
        contexts: Vec<UserContext>,
    },
    ExistingMessage {
        timestamp: i64,
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        message: String,
        sequence_id: String,
        notify: bool,
        message_flags: MessageFlags,
    },
    Channels {
        count: i64,
        contexts: Vec<ChannelContext>,
    },
}

impl FromParts for ContextInformationPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        match iter.next().unwrap().as_str() {
            "0" => {
                let count = iter
                    .next()
                    .unwrap_or("default_count".to_string())
                    .parse::<i64>()
                    .unwrap_or(0);
                let mut contexts: Vec<UserContext> = Vec::new();
                for _ in 0..count {
                    let user_id = iter.next().unwrap_or("default_user_id".to_string());
                    let username = iter.next().unwrap_or("default_username".to_string());
                    let color = iter
                        .next()
                        .unwrap_or("default_color".to_string())
                        .parse::<Color>()
                        .unwrap_or_default();
                    let user_permissions = iter
                        .next()
                        .unwrap_or("default_user_permissions".to_string())
                        .parse::<UserPermissions>()
                        .unwrap_or_default();
                    let visible = iter
                        .next()
                        .unwrap_or("default_visible".to_string())
                        .parse_sockbool()
                        .unwrap_or(false);
                    contexts.push(UserContext {
                        user_id,
                        username,
                        color,
                        user_permissions,
                        visible,
                    });
                }
                Ok(Self::ExistingUsers { count, contexts })
            }

            "1" => {
                let timestamp = iter
                    .next()
                    .unwrap_or("default_timestamp".to_string())
                    .parse::<i64>()
                    .unwrap_or(444);
                let user_id = iter.next().unwrap_or("default_user_id".to_string());
                let username = iter.next().unwrap_or("default_username".to_string());
                let color = iter
                    .next()
                    .unwrap_or("default_color".to_string())
                    .parse::<Color>()
                    .unwrap_or_default();
                let user_permissions = iter
                    .next()
                    .unwrap_or("default_user_permissions".to_string())
                    .parse::<UserPermissions>()
                    .unwrap_or_default();
                let message = iter.next().unwrap_or("default_message".to_string());
                let sequence_id = iter.next().unwrap_or("default_sequence_id".to_string());
                let notify = iter
                    .next()
                    .unwrap_or("default_notify".to_string())
                    .parse_sockbool()
                    .unwrap_or(false);
                let message_flags = iter
                    .next()
                    .unwrap_or("default_message_flags".to_string())
                    .parse::<MessageFlags>()
                    .unwrap_or_default();

                Ok(Self::ExistingMessage {
                    timestamp,
                    user_id,
                    username,
                    color,
                    user_permissions,
                    message,
                    sequence_id,
                    notify,
                    message_flags,
                })
            }

            "2" => {
                let count = iter
                    .next()
                    .unwrap_or("default_count".to_string())
                    .parse::<i64>()
                    .unwrap_or(0);
                let mut contexts: Vec<ChannelContext> = Vec::new();
                for _ in 0..count {
                    let channel_name = iter.next().unwrap_or("default_channel_name".to_string());
                    let password_protected = iter
                        .next()
                        .unwrap_or("default_password_protected".to_string())
                        .parse_sockbool()
                        .unwrap_or(false);
                    let temporary = iter
                        .next()
                        .unwrap_or("default_temporary".to_string())
                        .parse_sockbool()
                        .unwrap_or(false);
                    contexts.push(ChannelContext {
                        channel_name,
                        password_protected,
                        temporary,
                    });
                }
                Ok(Self::Channels { count, contexts })
            }

            _ => Err(ParsePacketError::WrongFormat),
        }
    }
}

impl Sockchatable for ContextInformationPacket {
    fn to_sockstr(&self) -> String {
        match self {
            Self::ExistingUsers { count, contexts } => {
                let mut output = String::new();
                output.push_str("0\t");
                output.push_str(count.to_string().as_str());
                for context in contexts {
                    output.push_str("\t");
                    output.push_str(context.to_sockstr().as_str());
                }
                output
            }

            Self::ExistingMessage {
                timestamp,
                user_id,
                username,
                color,
                user_permissions,
                message,
                sequence_id,
                notify,
                message_flags,
            } => vec![
                "1",
                timestamp.to_string().as_str(),
                user_id.as_str(),
                username.as_str(),
                color.to_sockstr().as_str(),
                user_permissions.to_sockstr().as_str(),
                message.as_str(),
                sequence_id.as_str(),
                notify.to_sockstr().as_str(),
                message_flags.to_sockstr().as_str(),
            ]
            .join("\t"),

            Self::Channels { count, contexts } => {
                let mut output = String::new();
                output.push_str("2\t");
                output.push_str(count.to_string().as_str());
                for context in contexts {
                    output.push_str("\t");
                    output.push_str(context.to_sockstr().as_str());
                }
                output
            }
        }
    }
}
