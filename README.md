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

```bash
wasm-pack build -- --features "wasm gl"
(cd www && npm install && npm run start)
```

Open http://localhost:8080. Currently nothing appears, we need to make it appear.
