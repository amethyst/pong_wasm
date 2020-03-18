# Pong -- WASM end-to-end proof of concept

## Building

### Native

```bash
cargo run --features gl
cargo run --features vulkan
cargo run --features metal
```

### WASM

### One Time Setup

```bash
npm init wasm-app www
```



### Ongoing Development

```bash
wasm-pack build -- --features "wasm gl"
```
