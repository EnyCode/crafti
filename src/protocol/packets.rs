use anyhow::Error;
use async_std::io::{Read, Write};
use async_trait::async_trait;
use protocol_derive::MinecraftPacket;

use crate::protocol::read::MinecraftReadable;
use crate::protocol::read::MinecraftReadableVar;

use super::read::MinecraftReadError;
use super::write::MinecraftWriteable;
use super::write::MinecraftWriteableVar;

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 0)]
pub struct HandshakePacket {
    #[var]
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for NextState {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let value = i32::read_var_from(buffer).await?;

        Ok(match value {
            1 => NextState::Status,
            2 => NextState::Login,
            _ => return Err(MinecraftReadError::UnexpectedEnumVariant.into()),
        })
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

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 0)]
pub struct StatusRequestPacket {}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 1)]
pub struct PingRequestPacket {
    pub payload: i64,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 0)]
pub struct StatusResponsePacket {
    pub response: String,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 1)]
pub struct PongResponsePacket {
    pub payload: i64,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 0)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: u128,
}
