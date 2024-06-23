use anyhow::Error;
use async_std::io::{Read, Write};
use async_trait::async_trait;
use protocol_derive::MinecraftPacket;

use crate::protocol::{
    read::{MinecraftReadError, MinecraftReadable, MinecraftReadableVar},
    write::{MinecraftWriteable, MinecraftWriteableVar},
};

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 0, direction = PacketDirection::Serverbound, status = NetworkStatus::Handshake)]
pub struct HandshakePacket {
    #[var]
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

#[derive(Copy, Clone, Debug)]
pub enum NextState {
    Status = 1,
    Login = 2,
}

impl Default for NextState {
    fn default() -> Self {
        NextState::Status
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for NextState {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        match self {
            NextState::Status => 1_i32.write_var_to(buffer).await?,
            NextState::Login => 2_i32.write_var_to(buffer).await?,
        }

        Ok(())
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for NextState {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let value = i32::read_var_from(buffer).await?;

        Ok(match value {
            0 => NextState::Status,
            1 => NextState::Login,
            _ => return Err(MinecraftReadError::UnexpectedEnumVariant.into()),
        })
    }
}
