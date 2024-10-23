use crate::packets::types::*;

pub enum ChannelEventPacket {
    Creation {
        channel_name: String,
        is_protected: bool,
        is_temporary: bool,
    },
    Update {
        channel_name: String,
        new_name: String,
        is_protected: bool,
        is_temporary: bool,
    },
    Deletion {
        channel_name: String,
    },
}

impl FromParts for ChannelEventPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let mut iter = parts.into_iter();
        match iter.next().unwrap().as_str() {
            "0" => {
                let channel_name = iter.next().unwrap();
                let is_protected = iter.next().unwrap().parse::<bool>().unwrap();
                let is_temporary = iter.next().unwrap().parse::<bool>().unwrap();
                Ok(ChannelEventPacket::Creation {
                    channel_name,
                    is_protected,
                    is_temporary,
                })
            }

            "1" => {
                let channel_name = iter.next().unwrap();
                let new_name = iter.next().unwrap();
                let is_protected = iter.next().unwrap().parse::<bool>().unwrap();
                let is_temporary = iter.next().unwrap().parse::<bool>().unwrap();
                Ok(ChannelEventPacket::Update {
                    channel_name,
                    new_name,
                    is_protected,
                    is_temporary,
                })
            }

            "2" => {
                let channel_name = iter.next().unwrap();
                Ok(ChannelEventPacket::Deletion { channel_name })
            }

            _ => Err(ParsePacketError::WrongFormat),
        }
    }
}

impl Sockchatable for ChannelEventPacket {
    fn to_sockstr(&self) -> String {
        match self {
            Self::Creation {
                channel_name,
                is_protected,
                is_temporary,
            } => vec![
                channel_name.as_str(),
                is_protected.to_string().as_str(),
                is_temporary.to_string().as_str(),
            ]
            .join("\t"),

            Self::Update {
                channel_name,
                new_name,
                is_protected,
                is_temporary,
            } => vec![
                channel_name.as_str(),
                new_name.as_str(),
                is_protected.to_string().as_str(),
                is_temporary.to_string().as_str(),
            ]
            .join("\t"),

            Self::Deletion { channel_name } => channel_name.clone(),
        }
    }
}
