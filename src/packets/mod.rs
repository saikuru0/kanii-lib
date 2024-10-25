use types::Sockchatable;

pub mod client;
pub mod server;
pub mod types;

#[derive(Debug)]
pub enum Packet {
    Client(client::ClientPacket),
    Server(server::ServerPacket),
}

impl Sockchatable for Packet {
    fn to_sockstr(&self) -> String {
        match self {
            Self::Client(packet) => packet.to_sockstr(),
            Self::Server(packet) => packet.to_sockstr(),
        }
    }
}
