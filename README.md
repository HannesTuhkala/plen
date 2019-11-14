# Plen

Fly the plen blyat

A multiplayer shoot at each other with planes game

## Usage instructions

The game client is very slow in debug mode, so it should be run in release mode

- Start a server using `cargo run --bin server`
- Start the client using `cargo run --bin client --release`
    - The default is to connect to `localhost:4444`
    - Specify another IP using the environment variable`SERVER=<url>:<port>`


