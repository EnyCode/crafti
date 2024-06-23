use crafti_protocol_derive::{MinecraftPacket, MinecraftReadable, MinecraftWriteable};

use crate as crafti_protocol;

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 0, direction = PacketDirection::Serverbound, status = NetworkStatus::Login)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Option<u128>,
}

// TODO: Move to Chat
#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 0, direction = PacketDirection::Clientbound, status = NetworkStatus::Login)]
pub struct DisconnectLoginPacket {
    pub reason: String,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 1, direction = PacketDirection::Serverbound, status = NetworkStatus::Login)]
pub struct EncryptionResponsePacket {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 1, direction = PacketDirection::Clientbound, status = NetworkStatus::Login)]
pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 2, direction = PacketDirection::Serverbound, status = NetworkStatus::Login)]
pub struct LoginPluginResponse {
    #[var]
    pub message_id: i32,
    pub data: Option<Vec<u8>>,
}

// TODO: move to uuid
#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 2, direction = PacketDirection::Clientbound, status = NetworkStatus::Login)]
pub struct LoginSuccess {
    pub uuid: u128,
    pub username: String,
    pub properties: Vec<LoginSuccessProperty>,
}

// TODO: derive for types like this
#[derive(MinecraftReadable, MinecraftWriteable, Debug, Default)]
pub struct LoginSuccessProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 3, direction = PacketDirection::Clientbound, status = NetworkStatus::Login)]
pub struct SetCompression {
    #[var]
    pub threshold: i32,
}

// TODO: Identifier
#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 4, direction = PacketDirection::Clientbound, status = NetworkStatus::Login)]
pub struct LoginPluginRequest {
    #[var]
    pub message_id: i32,
    pub channel: String,
    pub data: Vec<u8>,
}
