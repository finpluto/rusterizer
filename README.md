# Rusterizer

## Why using this name?

I ran out inspiration, it's just the combination between 'Rust' and 'Rasterizer'.

## How to build?

This crate is consumed by the Zig lab main project by dynamic linking.

If you want to build a dynamic lib only, install Rust with `rustup`, then simply run:

```bash
cargo build --release
```

The compiled library (`dll` or `so`) will appear under `target` folder.