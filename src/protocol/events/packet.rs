use async_std::io::Cursor;
use bevy_ecs::event::Event;

/// Sent when a packet is read by the stream. 
/// 
/// See [`NetworkStream`](crate) for an example. 
#[derive(Event, Debug)]
pub struct RecievePacketEvent {
    /// Contains the id of the packet. When reading packets, you should check this against the one you want to read. 
    pub id: i32,
    /// Contains the packet data without a size. This can be read with [`read_packet_unsized`](crafti_protocol). 
    pub packet: Cursor<Vec<u8>>,
}

/// Read by the stream to write data. 
/// 
/// This can be sent from anywhere in response to an action. e.g. a keepalive packet would be read using [`RecievePacketEvent`] and a response can be written by sending a [`SendPacketEvent`]. 
#[derive(Event, Debug)]
pub struct SendPacketEvent {
    /// The data in this buffer will be written directly to the stream. 
    pub packet: Cursor<Vec<u8>>,
}