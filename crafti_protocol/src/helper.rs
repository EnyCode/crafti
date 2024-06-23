use anyhow::Error;
use async_std::io::{Cursor, Read};
use async_std::task::block_on;
use crafti_protocol::{
    read::{MinecraftReadable, MinecraftReadableVar},
    stream::{MinecraftPacket, MinecraftStreamRead, MinecraftStreamWrite},
};

/// Reads a packet from a cursor.
///
/// Designed to be used with the [`crate::events::packet::RecievePacketEvent`]. This is also just a sync wrapper around [`MinecraftReadable::read_from`].
pub fn read_packet<P: MinecraftPacket>(data: &mut Cursor<Vec<u8>>) -> Result<P, Error> {
    data.set_position(0);

    block_on(async { Ok(data.read_packet_unsized().await?) })
}

/// Converts a packets into bytes.
///
/// Designed to be used with the [`crate::events::packet::SendPacketEvent`]. This is also just a sync wrapper around [`MinecraftStreamWrite::write_packet`].
pub fn to_bytes<P: MinecraftPacket>(packet: &mut P) -> Cursor<Vec<u8>> {
    block_on(async {
        let mut cursor = Cursor::new(Vec::new());

        cursor.write_packet(packet).await.unwrap();

        return cursor;
    })
}

// TODO: improve docs
/// Reads a type from a cursor.
pub fn read_type<T: Read + Unpin + Send + Sync, P: MinecraftReadable<T>>(
    data: &mut T,
) -> Result<P, Error> {
    block_on(async { Ok(P::read_from(data).await?) })
}

// TODO: improve docs
/// Reads a var type from a cursor.
///
/// For example, a varint or a varlong.
pub fn read_var_type<T: Read + Unpin + Send + Sync, P: MinecraftReadableVar<T>>(
    data: &mut T,
) -> Result<P, Error> {
    block_on(async { Ok(P::read_var_from(data).await?) })
}
