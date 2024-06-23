use bevy_ecs::event::Event;
use crafti_world::{Chunk, LightSection};

// TODO: improve docs?
// I don't know how but they are a bit basic

/// Sent to load a chunk.
#[derive(Event, Debug)]
pub struct LoadChunkEvent {
    /// The chunk data. This includes the x and z coordinates.
    pub chunk: Chunk,
}

/// Sent to update lighting in a chunk section.
///
/// This could be either block light or sky light.
/// It is sent when a chunk is loaded or when lighting changes.
#[derive(Event, Debug)]
pub struct UpdateLightEvent {
    /// The x coordinate of the chunk.
    pub x: i32,
    /// The y coordinate of the chunk section.
    pub y: i32,
    /// The z coordinate of the chunk.
    pub z: i32,
    /// The light data.
    pub light: LightSection,
}

/// Sent to update the time.
///
/// Time increases by 20 every second (so 1 every tick). There are 24,000 ticks in a day
/// meaning Minecraft days are 20 minutes long.
#[derive(Event, Debug)]
pub struct UpdateTimeEvent {
    /// The world age in ticks. This is not changed by server commands.
    pub world_age: i64,
    /// The world time, in ticks.
    pub time: i64,
}
