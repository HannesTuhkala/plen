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

SDL2 needs to be added to be able to compile Plen under Windows. You can follow [this tutorial](https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md#windows-with-build-script)
(or possibly one of the others listed below it, not tested it myself but should work).

You also need to install SDL2_image, SDL2_mixer and SDL2_ttf which can be found here:
- https://www.libsdl.org/projects/SDL_image/
- https://www.libsdl.org/projects/SDL_mixer/
- https://www.libsdl.org/projects/SDL_ttf/

Download both zip files under "Development Libraries", SDL2_xxxxx-devel-2.x.x-VC.zip and SDL2_xxxxx-devel-2.x.x-mingw.tar.gz.

Extract them both according to the same instructions as noted in the SDL2 tutorial.

Plen should now compile and you should be able to run the game under Windows.

## Attribution
Flame animation by serenajeanne used under CC-BY-SA 3.0
[opengameart](https://opengameart.org/content/small-flame-pack-0)
