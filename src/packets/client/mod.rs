pub mod ping;
pub mod authentication;
pub mod message;

use ping::PingPacket;
use authentication::AuthenticationPacket;
use message::MessagePacket;

pub enum ClientPacket {
    Ping(PingPacket),
    Authentication(AuthenticationPacket),
    Message(MessagePacket),
}
