# Plen

Fly the plen blyat

A multiplayer shoot at each other with planes game

## Usage instructions

The game client is very slow in debug mode, so it should be run in release mode

- Start a server using `cargo run --bin server`
- Start the client using `cargo run --bin client --release`
    - The default is to connect to `localhost:4444`
    - Specify another IP using the environment variable`SERVER=<url>:<port>`


### Compiling under Windows

SDL2 needs to be added to be able to compile Plen under Windows. You can follow this tutorial https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md#windows-with-build-script
or one of the others listed below it.

You also need to install SDL2 mixer which can be found here https://www.libsdl.org/projects/SDL_mixer/.

Download both zip files under "Development Libraries", SDL2_mixer-devel-2.x.x-VC.zip and SDL2_mixer-devel-2.x.x-mingw.tar.gz.

Extract them both according to the same instructions as noted in the SDL2 tutorial.

Plen should now compile and you should be able to run the game under Windows.