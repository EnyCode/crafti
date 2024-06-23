use anyhow::Error;
use async_std::io::{Write, WriteExt};
use async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinecraftWriteError {
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
pub trait MinecraftWriteable<W: Write + Unpin + Send + Sync>: Debug + Send + Sync {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error>;
}

#[async_trait]
pub trait MinecraftWriteableVar<W: Write + Unpin + Send + Sync>: Debug + Send + Sync {
    async fn write_var_to(&self, buffer: &mut W) -> Result<(), Error>;
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteableVar<W> for i32 {
    async fn write_var_to(&self, buffer: &mut W) -> Result<(), Error> {
        if self == &0i32 {
            buffer.write(&[0u8; 1]).await?;
            return Ok(());
        }

        let mut buf = [0];
        let mut value = self.clone();

        while value != 0 {
            buf[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i32::max_value() >> 6);

            if value != 0 {
                buf[0] |= 0b1000_0000;
            }

            buffer.write(&mut buf).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteableVar<W> for i64 {
    async fn write_var_to(&self, buffer: &mut W) -> Result<(), Error> {
        let mut buf = [0];
        let mut value = self.clone();

        while value != 0 {
            buf[0] = (value & 0b0111_1111) as u8;
            value = (value >> 7) & (i64::max_value() >> 6);

            if value != 0 {
                buf[0] |= 0b1000_0000;
            }

            buffer.write(&mut buf).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for bool {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        u8::write_to(if *self { &1 } else { &0 }, buffer).await?;

        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for i8 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for u8 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for i16 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for u16 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for i32 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for u32 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for i64 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for u64 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for i128 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for u128 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for f32 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for f64 {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        buffer.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync> MinecraftWriteable<W> for String {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        if self.len() > 32767 {
            return Err(MinecraftWriteError::StringTooLong {
                length: self.len() as u32,
                max_length: 32767,
            }
            .into());
        }
        i32::write_var_to(&(self.len() as i32), buffer).await?;
        buffer.write_all(self.as_bytes()).await?;

        Ok(())
    }
}

// TODO: identifier, entity metadata, slot, nbt, position, angle, uuid

#[async_trait]
impl<W: Write + Unpin + Send + Sync, T: MinecraftWriteable<W>> MinecraftWriteable<W> for Option<T> {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        bool::write_to(&self.is_some(), buffer).await?;

        if let Some(value) = self {
            value.write_to(buffer).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<W: Write + Unpin + Send + Sync, T: MinecraftWriteable<W>> MinecraftWriteable<W> for Vec<T> {
    async fn write_to(&self, buffer: &mut W) -> Result<(), Error> {
        i32::write_var_to(&(self.len() as i32), buffer).await?;

        for value in self {
            value.write_to(buffer).await?;
        }

        Ok(())
    }
}
