To build this, you'll need to set up the SDL development libraries.

For Windows, follow the instructions for [Windows (MSVC)](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc).  It doesn't seem like you need to define a `LIB` environment variable, though.

Then build and run via:

```
cargo build -p sdl
cargo run -p sdl
```
