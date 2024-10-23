use super::{FromParts, ParsePacketError};

pub struct ContextClearingPacket {
    message_history: bool,
    user_list: bool,
    channel_list: bool,
}

impl FromParts for ContextClearingPacket {
    fn from_parts(parts: Vec<String>) -> Result<Self, ParsePacketError> {
        let num = parts[0].as_str();
        let message_history = ["0", "3", "4"].contains(&num);
        let user_list = ["1", "3"].contains(&num);
        let channel_list = ["2", "4"].contains(&num);
        if message_history || user_list || channel_list {
            Ok(ContextClearingPacket {
                message_history,
                user_list,
                channel_list,
            })
        } else {
            Err(ParsePacketError::WrongFormat)
        }
    }
}
