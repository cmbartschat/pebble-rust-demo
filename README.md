# Pebble Test App

Showcase for the capabilities of https://github.com/cmbartschat/pebble-rust-2026

## Development

### Prerequisites

1. Cargo: https://rustup.rs/
2. Pebble SDK: https://developer.repebble.com/sdk/
3. Cargo Pebble: `cargo install --git https://codeberg.org/filmroellchen/cargo-pebble.git`
4. Rust targets: `rustup target add thumbv7m-none-eabi thumbv7em-none-eabi thumbv8m.main-none-eabi`

```sh
cargo pebble build
pebble install --emulator emery
```
