use crate::packets::types::*;

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
                let count = iter.next().unwrap().parse::<i64>().unwrap();
                let mut contexts: Vec<UserContext> = Vec::new();
                for _ in 0..count {
                    let user_id = iter.next().unwrap();
                    let username = iter.next().unwrap();
                    let color = iter.next().unwrap().parse::<Color>().unwrap();
                    let user_permissions = iter.next().unwrap().parse::<UserPermissions>().unwrap();
                    let visible = iter.next().unwrap().parse::<bool>().unwrap();
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
                let timestamp = iter.next().unwrap().parse::<i64>().unwrap();
                let user_id = iter.next().unwrap();
                let username = iter.next().unwrap();
                let color = iter.next().unwrap().parse::<Color>().unwrap();
                let user_permissions = iter.next().unwrap().parse::<UserPermissions>().unwrap();
                let message = iter.next().unwrap();
                let sequence_id = iter.next().unwrap();
                let notify = iter.next().unwrap().parse::<bool>().unwrap();
                let message_flags = iter.next().unwrap().parse::<MessageFlags>().unwrap();

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
                let count = iter.next().unwrap().parse::<i64>().unwrap();
                let mut contexts: Vec<ChannelContext> = Vec::new();
                for _ in 0..count {
                    let channel_name = iter.next().unwrap();
                    let password_protected = iter.next().unwrap().parse::<bool>().unwrap();
                    let temporary = iter.next().unwrap().parse::<bool>().unwrap();
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
                let mut output = count.to_string();
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
                timestamp.to_string().as_str(),
                user_id.as_str(),
                username.as_str(),
                color.to_sockstr().as_str(),
                user_permissions.to_sockstr().as_str(),
                message.as_str(),
                sequence_id.as_str(),
                notify.to_string().as_str(),
                message_flags.to_sockstr().as_str(),
            ]
            .join("\t"),
            Self::Channels { count, contexts } => {
                let mut output = count.to_string();
                for context in contexts {
                    output.push_str("\t");
                    output.push_str(context.to_sockstr().as_str());
                }
                output
            }
        }
    }
}
