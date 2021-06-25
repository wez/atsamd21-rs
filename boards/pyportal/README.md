# Adafruit PyPortal Board Support Crate

This crate provides a type-safe API for working with the [Adafruit PyPortal
board](https://www.adafruit.com/product/4116).

Check out the repository for examples:
https://github.com/atsamd-rs/atsamd/tree/master/boards/pyportal/examples

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [hf2-cli] the uf2 bootloader flasher tool(https://crates.io/crates/hf2-cli) however your platform requires

* Be in this directory `cd boards/pyportal`
* Put your device in bootloader mode usually by hitting the reset button twice.

### Uploading an example: cargo run

The `.cargo/config` in this bsp is preset with `hf2-cli` as a `runner`. This means you can use cargo run to both cargo build and attempt to upload:
```bash
$ cargo run --release --example blinky_basic
```

Or even better just your ide's "run" button or hotkey and cargo run will build and upload.
