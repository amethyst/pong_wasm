# Pong -- WASM end-to-end proof of concept

## Building

### Native

```bash
cargo run --features gl
cargo run --features vulkan
cargo run --features metal
```

### WASM

### Ongoing Development

Requires nightly rust.

```bash
./build.sh
```

Then run your favorite HTTP server:
* Node: `npm install -g http-server` then `http-server`
* Python: `python3 -m http.server`

Open http://localhost:8080. Currently broken game appears. We need to fix it.

Only Chrome renders something (stable and nightly). Firefox nightly loads but complains.
