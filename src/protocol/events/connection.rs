use bevy_ecs::event::Event;

/// Sent to initialise the network stream and start everything.
#[derive(Event, Debug)]
pub struct ServerConnectEvent {
    /// The ip address to connect to.
    ///
    /// This should be an address followed by the port, e.g. 127.0.0.1:25565
    pub ip: String,
    /// The username to connect with.
    ///
    /// Only offline mode is supported for now, so this can be any acceptable username.
    // TODO: move to resource for auth
    pub username: String,
}

#[derive(Event, Debug)]
pub struct DisconnectEvent {
    pub reason: String,
}
