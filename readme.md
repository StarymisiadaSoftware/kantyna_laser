# Kantyna laser

A web app enabling everyone to enqueue their favorite music on your party!

Host a local instance on your computer (the one which is plugged to audio equipment) in your home network
and give the guests the IP address so that everyone can access the site and add a track to the music queue.

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

`cargo make serve` (for development only)

or using `microserver` directly:

`microserver --port 8000`

You'll need to change the port to `80` for "production".

`sudo microserver --port 80`

## Running backend service

`cargo run -r -p service`

Song queue is currently held in-memory.
Position in the queue can only advance when ALL hooks finish executing.
Playback is expected to be managed by hooks.
At `service/hooks` you can find some ready-made hooks, one of which plays songs using `mpv` (written in Python).

Backend listens on port `8090`.
This is currently hard-coded.

### Hook dir

The backend will attempt to load hooks from path specified by `KANTYNA_LASER_HOOK_DIR` environment variable.
If the variable doesn't exist the backend will attempt to read `hooks/` directory (relative to current working directory).

## Implementing hooks

Just put anything runnable in your hooks directory.
The backend sets `KANTYNA_LASER_URL` environment variable to the URL of the song to be played.
You can read that value and do anything you want with it.

## API Reference

Reference for the backend's API

### **GET** `/preview_queue`

Todo: Document it

### **POST** `/enqueue`

Todo: Document it