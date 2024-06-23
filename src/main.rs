use anyhow::Error;
use async_std::{
    io::copy,
    net::{TcpListener, TcpStream},
    stream::StreamExt,
    task::spawn,
};
use futures::try_join;
use protocol::{
    packets::{HandshakePacket, LoginStartPacket, NextState},
    stream::MinecraftStream,
};

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
async fn main() {
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

    if handshake.next_state == NextState::Status {
    } else {
        handshake.server_address = config.target_ip.to_owned();

        let mut login_start: LoginStartPacket = client.read_packet().await?;
        println!("connecting player {}", login_start.name);

        let mut server = TcpStream::connect(config.target_ip.to_owned() + ":25565").await?;
        server.write_packet(&mut handshake).await?;
        server.write_packet(&mut login_start).await?;

        let (mut client_recv, mut client_send) = (&client, &client);
        let (mut server_recv, mut server_send) = (&server, &server);

        let client_to_server = copy(&mut client_recv, &mut server_send);
        let server_to_client = copy(&mut server_recv, &mut client_send);

        let (_, _) = try_join!(client_to_server, server_to_client)?;
    }

    Ok(())
}
