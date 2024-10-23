use super::{FromParts, ParsePacketError, Sockchatable};

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

impl Sockchatable for ContextClearingPacket {
    fn to_sockstr(&self) -> String {
        match (self.message_history, self.user_list, self.channel_list) {
            (true, false, false) => String::from("0"),
            (false, true, false) => String::from("1"),
            (false, false, true) => String::from("2"),
            (true, true, false) => String::from("3"),
            (true, true, true) => String::from("4"),
            _ => String::from("i havent read the docs"),
        }
    }
}
