use anyhow::Error;
use async_std::io::{Read, ReadExt};
use async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinecraftReadError {
    #[error("Invalid VarInt")]
    InvalidVarInt,
    #[error("Invalid VarLong")]
    InvalidVarLong,
    #[error("Invalid String length; max length is {max_length} but got {length}")]
    StringTooLong { length: u32, max_length: u32 },
    #[error("Invalid Enum variant")]
    UnexpectedEnumVariant,
}

#[async_trait]
pub trait MinecraftReadable<R: Read + Unpin + Send + Sync>: Debug + Send + Sync {
    async fn read_from(buffer: &mut R) -> Result<Self, Error>
    where
        Self: Sized;
}

#[async_trait]
pub trait MinecraftReadableVar<R: Read + Unpin + Send + Sync>: Debug + Send + Sync {
    async fn read_var_from(buffer: &mut R) -> Result<Self, Error>
    where
        Self: Sized;
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadableVar<R> for i32 {
    async fn read_var_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        let mut ans = 0;

        for i in 0..4 {
            buffer.read_exact(&mut buf).await?;
            ans |= (buf[0] as i32 & 0x7F) << (7 * i);
            if buf[0] & 0x80 == 0 {
                return Ok(ans);
            }
        }

        Err(MinecraftReadError::InvalidVarInt.into())
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadableVar<R> for i64 {
    async fn read_var_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0];
        let mut ans: i64 = 0;

        for i in 0..8 {
            buffer.read_exact(&mut buf).await?;
            ans |= (buf[0] as i64 & 0x7F) << (7 * i);
            if buf[0] & 0x80 == 0 {
                return Ok(ans);
            }
        }

        Err(MinecraftReadError::InvalidVarLong.into())
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync, T: MinecraftReadableVar<R>> MinecraftReadableVar<R>
    for Option<T>
{
    async fn read_var_from(buffer: &mut R) -> Result<Self, Error> {
        let has_value = bool::read_from(buffer).await?;
        if has_value {
            return Ok(Some(T::read_var_from(buffer).await?));
        }
        Ok(None)
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync, T: MinecraftReadableVar<R> + Send> MinecraftReadableVar<R>
    for Vec<T>
{
    async fn read_var_from(buffer: &mut R) -> Result<Self, Error> {
        let length = i32::read_var_from(buffer).await?;
        let mut contents = Vec::new();
        for _ in 0..length {
            contents.push(T::read_var_from(buffer).await?);
        }
        Ok(contents)
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for bool {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        buffer.read_exact(&mut buf).await?;
        Ok(buf[0] != 0)
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for i8 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        buffer.read_exact(&mut buf).await?;
        Ok(i8::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for u8 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 1];
        buffer.read_exact(&mut buf).await?;
        Ok(u8::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for i16 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        buffer.read_exact(&mut buf).await?;
        Ok(i16::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for u16 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 2];
        buffer.read_exact(&mut buf).await?;
        Ok(u16::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for i32 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        buffer.read_exact(&mut buf).await?;
        Ok(i32::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for u32 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        buffer.read_exact(&mut buf).await?;
        Ok(u32::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for i64 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        buffer.read_exact(&mut buf).await?;
        Ok(i64::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for u64 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        buffer.read_exact(&mut buf).await?;
        Ok(u64::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for i128 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        buffer.read_exact(&mut buf).await?;
        Ok(i128::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for u128 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 16];
        buffer.read_exact(&mut buf).await?;
        Ok(u128::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for f32 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        buffer.read_exact(&mut buf).await?;
        Ok(f32::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for f64 {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 8];
        buffer.read_exact(&mut buf).await?;
        Ok(f64::from_be_bytes(buf))
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync> MinecraftReadable<R> for String {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let length = i32::read_var_from(buffer).await?;

        if length > 32767 * 4 + 3 {
            return Err(MinecraftReadError::StringTooLong {
                length: length as u32,
                max_length: 32767 * 4 + 3,
            }
            .into());
        }
        let mut buf = vec![0; length as usize];
        buffer.read_exact(&mut buf).await?;

        Ok(String::from_utf8(buf)?)
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync, T: MinecraftReadable<R>> MinecraftReadable<R> for Option<T> {
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let has_value = bool::read_from(buffer).await?;
        if has_value {
            return Ok(Some(T::read_from(buffer).await?));
        }
        Ok(None)
    }
}

#[async_trait]
impl<R: Read + Unpin + Send + Sync, T: MinecraftReadable<R> + Send> MinecraftReadable<R>
    for Vec<T>
{
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let length = i32::read_var_from(buffer).await?;
        let mut contents = Vec::new();
        for _ in 0..length {
            contents.push(T::read_from(buffer).await?);
        }
        Ok(contents)
    }
}

#[async_trait]
impl<
        R: Read + Unpin + Send + Sync,
        T: MinecraftReadable<R> + Default + Copy + Send,
        const N: usize,
    > MinecraftReadable<R> for [T; N]
{
    async fn read_from(buffer: &mut R) -> Result<Self, Error> {
        let mut contents = [T::default(); N];
        for i in 0..N {
            contents[i] = T::read_from(buffer).await?;
        }
        Ok(contents)
    }
}
