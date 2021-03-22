## Native app (SDL)

To build and run this natively, you'll need to set up the SDL development libraries.

For Windows, follow the instructions for [Windows (MSVC)](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc).  It doesn't seem like you need to define a `LIB` environment variable, though.

Then build and run via:

```
cargo build -p sdl
cargo run -p sdl
```

## Web app

You will need to install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), [https](https://crates.io/crates/https) and [Node JS](https://nodejs.org/).

Then run:

```
npm install
npm run watch
```

Then visit http://localhost:8000.

Note that while TypeScript will be automatically rebuild under `npm run watch`, rust code will _not_ be. You will need to rebuild the rust source manually via:

```
npm run wasm
```

## Deployment

Deploying the web version to GitHub Pages can be done via:

```
npm run deploy
```

## Other notes

The algorithm for the simulation was inspired by Petri Purho's GDC talk [Exploring the Tech and Design of Noita](https://www.youtube.com/watch?v=prXuyMCgbTc).
