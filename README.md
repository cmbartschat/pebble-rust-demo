# Pebble Test App

Showcase for the capabilities of https://github.com/cmbartschat/pebble-rust-2026

## Development

### Prerequisites

1. Cargo: https://rustup.rs/
2. Pebble SDK: https://developer.repebble.com/sdk/

```sh
rustup target add thumbv8m.main-none-eabi
pebble clean
pebble build
pebble install --emulator emery
```
