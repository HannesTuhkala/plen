#!/bin/sh
export EMMAKEN_CFLAGS='-s USE_SDL=2 -s USE_SDL_IMAGE=2 -s SDL2_IMAGE_FORMATS='["png"]' -s USE_SDL_TTF=2 -s USE_SDL_MIXER=2'
cargo build --target=wasm32-unknown-emscripten
