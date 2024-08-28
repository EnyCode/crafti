<div align="center">
  <h1>Crafti</h1>
  A simple Minecraft Proxy. 
</div>
<br />

## Features
- [x] Forward traffic straight to the server - no dencryption, meaning packets are secure.
- [x] A downtime MOTD if your backend server is not running
- [x] Fancy logging
- [x] Easy to use

## Building
Crafti is built using [Rust](https://rust-lang.org), and is therefore a requirement for building and running. 

Just run:
```
cargo run <target-ip> [listening-ip:port] [motd] [favicon]
```
and your server will be up and running!

## License
Crafti is licensed under Mozilla Public License 2.0 unless otherwise stated. 

btw this underwent a name change halfway that I have half sticked to and half not so in the logs it says mc-nano. 
