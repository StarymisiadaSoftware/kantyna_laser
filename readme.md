# Kantyna laser

TODO: Describe what I am

## Requirements

* Rust
* `wasm-pack` (version `0.9.1` is the NEWEST known to work)
* `microserver`
* `yt-dlp`
* Optional: `cargo-make` (for using cargo to run all commands)


### Getting `wasm-pack`

You may need to run this:

```
rustup target add wasm32-unknown-unknown
```

This command installs `wasm-pack`:

```
cargo install --force wasm-pack --version 0.9.1
```

## Building Website

Go to the `website/` subfolder and run:

`cargo make build_release`

or using `wasm-pack` directly:

`wasm-pack build --target web --out-name package`

After that, the website is ready to be served (`website/` directory will be its' root).

## Serving website

Inside the `website/` directory, run:

`cargo make serve`

or using `microserver` directly:

`microserver --port 8000`

You'll need to change the port to `80` for "production".

## Running backend service

`cargo run -r -p service`
