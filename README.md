## Native app (SDL)

To build and run this natively, you'll need to set up the SDL development libraries.

For Windows, follow the instructions for [Windows (MSVC)](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc).  It doesn't seem like you need to define a `LIB` environment variable, though.

Then build and run via:

```
cargo build -p sdl
cargo run -p sdl
```

## Web app

You will need to install [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/).

Then run:

```
wasm-pack build web --target web
```

Then start a web server in the root directory of this repository via e.g.
`python -m http.server` and visit it.
