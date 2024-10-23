pub mod client;
pub mod server;
pub mod types;

pub enum Packet {
    Client(client::ClientPacket),
    Server(server::ServerPacket),
}
