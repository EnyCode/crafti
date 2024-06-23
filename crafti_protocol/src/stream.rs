use anyhow::Error;
use async_std::{
    io::{Read, ReadExt, Write},
    task::block_on,
};
use async_trait::async_trait;
use std::{fmt::Debug, pin::Pin, task::Poll};

use crate::read::{MinecraftReadable, MinecraftReadableVar};

pub enum PacketDirection {
    Clientbound,
    Serverbound,
}

pub enum NetworkStatus {
    Handshake,
    Status,
    Login,
    Play,
}

#[derive(Debug)]
pub struct Cursor(async_std::io::Cursor<Vec<u8>>);

impl std::io::Read for Cursor {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        block_on(async { self.0.read(buf).await })
    }
}

impl Read for Cursor {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut async_std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.get_mut().0).poll_read(cx, buf)
    }
}

impl Cursor {
    pub fn new(inner: Vec<u8>) -> Cursor {
        Self {
            0: async_std::io::Cursor::new(inner),
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0.into_inner()
    }

    pub fn get_ref(&self) -> &Vec<u8> {
        self.0.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {
        self.0.get_mut()
    }

    pub fn position(&self) -> u64 {
        self.0.position()
    }

    pub fn set_position(&mut self, pos: u64) {
        self.0.set_position(pos);
    }
}

pub trait MinecraftPacket: MinecraftReadable<Cursor> {
    // + MinecraftWriteable<Cursor> {
    fn get_id() -> i32;
    fn get_direction() -> PacketDirection;
    fn get_status() -> NetworkStatus;
}

#[async_trait]
pub trait MinecraftStream<S: Read + Write + Send + Sync + Unpin>: MinecraftStreamRead<S> {}

#[async_trait]
pub trait MinecraftStreamRead<S: Read + Send + Sync + Unpin> {
    async fn read_packet<R: MinecraftPacket + Send>(&mut self) -> Result<R, Error>;
}

#[async_trait]
impl<S: Read + Send + Sync + Unpin + Debug> MinecraftStreamRead<S> for S {
    async fn read_packet<R: MinecraftPacket + Send>(&mut self) -> Result<R, Error> {
        let length = i32::read_var_from(self).await?;

        let mut buffer = vec![0u8; length as usize];
        self.read_exact(&mut buffer).await?;
        let mut cursor = Cursor::new(buffer);

        let id = i32::read_var_from(&mut cursor).await?;

        if id != R::get_id() {
            return Err(Error::msg(format!(
                "Expected packet id {}, got {}",
                R::get_id(),
                id
            )));
        }

        R::read_from(&mut cursor).await
    }
}
