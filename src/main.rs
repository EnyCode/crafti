use async_std::{
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    task::spawn,
};
use protocol::packets::{HandshakePacket, NextState, StatusResponsePacket};

pub mod protocol;

#[derive(Debug, Clone)]
pub struct Config {
    pub target_ip: String,
    pub listening_ip: String,
    pub motd: String,
    pub offline_motd: String,
    pub favicon: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_ip: "".to_owned(),
            listening_ip: "0.0.0.0:25565".to_string(),
            motd: r#"[{"text":"A ","color":"gold"},{"text":"nano-mc","color":"green"},{"text":" proxy.","color":"gold"}]"#
                .to_string(),
            offline_motd: r#"[{"text":"A ","color":"gold"},{"text":"nano-mc","color":"green"},{"text":" proxy. ","color":"gold"},{"text":"(","color":"gray"},{"text":"Offline","color":"red"},{"text":")","color":"gray"}]"#
                .to_string(),
            favicon: "".to_string(),
        }
    }
}

#[async_std::main]
fn main() {
    let config = Config::default();

    let listener = TcpListener::bind(config.listening_ip.clone())
        .await
        .unwrap();

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream.unwrap();
        let cloned = config.clone();
        spawn(async move { handle_conn(stream, cloned).await });
    }
}

async fn handle_conn(mut client: TcpStream, config: Config) -> Result<(), Error> {
    let mut handshake: HandshakePacket = client.read_packet().await?;
}
