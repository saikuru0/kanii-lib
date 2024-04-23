trait Parsable {
    fn to_str(&self) -> String;
    fn from_str(packet: String) -> Self;
}

enum ClientPacket {
    Ping { user_id: String },
    Authentication {
        user_id: String,
        magic: String,
        authkey: String },
    Message {
        user_id: String,
        message: String },
}

enum BadAuthReason {
    AuthFail,
    UserFail,
    SockFail,
    JoinFail,
}

enum JoinAuthPacket {
    Join {
        timestamp: i64,
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        sequence_id: String },
    GoodAuth {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        channel_name: String,
        max_msg_length: i64 },
    BadAuth {
        reason: BadAuthReason,
        timestamp: i64 },
}

enum ChannelEventPacket {
    Creation {
        channel_name: String,
        is_protected: bool,
        is_temporary: bool },
    Update {
        channel_name: String,
        new_name: String,
        is_protected: bool,
        is_temporary: bool },
    Deletion { channel_name: String },
}

enum Color {
    Hex(i8),
    ShortHex(i8),
    Name(String),
    Rgb(i8, i8, i8),
    Hsl(i8, f32, f32),
}

struct UserPermissions {
    rank: u8,
    can_moderate: bool,
    can_logs: bool,
    can_nickname: bool,
    channel_permissions: u8,
}

enum ChannelSwitchPacket {
    Join {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        sequence_id: String },
    Departure {
        user_id: String,
        sequence_id: String },
    ForcedSwitch { channel_name: String },
}

enum DisconnectReason {
    Leave,
    Timeout,
    Kick,
    Flood,
}

struct UserContext {
    user_id: String,
    username: String,
    color: Color,
    user_permissions: UserPermissions,
    visible: bool,
}

struct MessageFlags {
    bold: bool,
    cursive: bool,
    underlined: bool,
    colon: bool,
    private: bool,
}

struct ChannelContext {
    channel_name: String,
    password_protected: bool,
    temporary: bool,
}

enum ContextInformationPacket {
    ExistingUsers {
        count: i64,
        contexts: Vec<UserContext> },
    ExistingMessage {
        timestamp: i64,
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions,
        message: String,
        sequence_id: String,
        notify: bool,
        message_flags: MessageFlags },
    Channels {
        count: i64,
        contexts: Vec<ChannelContext> },
}

struct ClearedContext {
    message_list: bool,
    user_list: bool,
    channel_list: bool,
}

enum ServerPacket {
    Pong { timestamp: i64 },
    JoinAuth(JoinAuthPacket),
    ChatMessage {
        timestamp: i64,
        user_id: String,
        message: String,
        sequence_id: String,
        message_flags: MessageFlags },
    UserDisconnect {
        user_id: String,
        username: String,
        reason: DisconnectReason,
        timestamp: i64,
        sequence_id: String },
    ChannelEvent(ChannelEventPacket),
    ChannelSwitch(ChannelSwitchPacket),
    MessageDeletion{ sequence_id: String },
    ContextInformation(ContextInformationPacket),
    ContextClearing(ClearedContext),
    ForcedDisconnect {
        ban: bool,
        timestamp: i64 },
    UserUpdate {
        user_id: String,
        username: String,
        color: Color,
        user_permissions: UserPermissions },
}

enum Packet {
    Client(ClientPacket),
    Server(ServerPacket)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// enum ClientPacket
// enum ServerPacket
// struct Parser {
//     fn ptostr
//     fn strtop
// }
// trait Transmission
// struct Client <T:transmision> {
//     connection
//     set_receive_handler
// }
