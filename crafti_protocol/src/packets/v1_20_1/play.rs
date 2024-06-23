use crafti_protocol_derive::{MinecraftPacket, MinecraftReadable, MinecraftWriteable};
use quartz_nbt::NbtCompound;

use crate::{
    self as crafti_protocol, bitset::BitSet, read::MinecraftReadable, read::MinecraftReadableVar,
};

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 18, direction = PacketDirection::Serverbound, status = NetworkStatus::Play)]
pub struct KeepAliveServerbound {
    pub id: i64,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 26, direction = PacketDirection::Clientbound, status = NetworkStatus::Play)]
pub struct DisconnectPlayPacket {
    pub reason: String,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 35, direction = PacketDirection::Clientbound, status = NetworkStatus::Play)]
pub struct KeepAliveClientbound {
    pub id: i64,
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 36, direction = PacketDirection::Clientbound, status = NetworkStatus::Play)]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: NbtCompound,
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntityData>,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light: Vec<Vec<u8>>,
    pub block_light: Vec<Vec<u8>>,
}

#[derive(MinecraftWriteable, Debug, Default)]
pub struct BlockEntityData {
    pub packed_xz: i8,
    pub y: i16,
    #[var]
    pub entity_type: i32,
    pub data: NbtCompound,
}

#[async_trait::async_trait]
impl<R: std::io::Read + async_std::io::Read + Unpin + Send + Sync> MinecraftReadable<R>
    for BlockEntityData
{
    async fn read_from(buffer: &mut R) -> Result<Self, anyhow::Error> {
        let packed_xz: i8 = i8::read_from(buffer).await?;
        let y = i16::read_from(buffer).await?;
        let entity_type = i32::read_var_from(buffer).await?;
        let mut data = [0u8; 1024];
        buffer.read(&mut data)?;
        //println!("{:#?}", data);
        //let data = NbtCompound::read_from(buffer).await?;
        return Ok(Self {
            packed_xz,
            y,
            entity_type,
            data: NbtCompound::new(),
        });
    }
}

#[derive(MinecraftPacket, Debug, Default)]
#[packet(id = 94, direction = PacketDirection::Clientbound, status = NetworkStatus::Play)]
pub struct UpdateTime {
    pub world_age: i64,
    pub time: i64,
}
