# Adafruit PyGamer Board Support Crate

This crate provides a type-safe API for working with the [Adafruit PyGamer
board](https://www.adafruit.com/product/4242).

Check out the repository for examples:
https://github.com/atsamd-rs/atsamd/tree/master/boards/pygamer/examples


## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [hf2-cli] the uf2 bootloader flasher tool(https://crates.io/crates/hf2-cli) however your platform requires
* Be in this directory `cd boards/pygamer`
* Put your device in bootloader mode usually by hitting the reset button twice.

### Uploading an example: cargo run

The `.cargo/config` in this bsp is preset with `hf2-cli` as a `runner`. This means you can use cargo run to both cargo build and attempt to upload:
```bash
$ cargo run --release --example blinky_basic
```

Or even better just your ide's "run" button or hotkey and cargo run will build and upload.

## troubleshooting
Note some examples will tell you they need more features enabled
```
$ cargo run --release --example neopixel_easing
error: target `neopixel_easing` in package `pygamer` requires the features: `math`
Consider enabling them by passing, e.g., `--features="math"`
```
Just follow the instructions to add --features like
```
cargo run --release --example neopixel_easing --features="math"
    Finished release [optimized + debuginfo] target(s) in 0.09s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyGamer"))
    Flashing "/Users/User/atsamd/boards/pygamer/target/thumbv7em-none-eabihf/release/examples/neopixel_easing"
    Finished in 0.167s
$
```
