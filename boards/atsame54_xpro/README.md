# SAM E54 Xplained Pro Evaluation Kit Board Support Crate

This crate provides a type-safe Rust API for working with the
[SAM E54 Xplained Pro Evaluation Kit](https://www.microchip.com/developmenttools/productdetails/atsame54_xpro).

## Board Features

- Microchip [ATSAME54P] Cortex-M4 microcontroller @ 120 MHz (32-bit, 3.3V logic and power)
  - 1MB Flash
  - 256kB SRAM
  - 8MB SPI Flash chip

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/atsame54_xpro/examples

* Be in this directory `cd boards/atsame54_xpro`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```
$ cargo hf2 --release --example blinky_basic
    Finished release [optimized + debuginfo] target(s) in 2m 02s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Microchip")) Ok(Some("SAM E54 Xplained Pro Evaluation Kit"))
    Flashing "/path/to/atsamd/boards/atsame54_xpro/target/thumbv7em-none-eabihf/release/examples/blinky_basic"
    Finished in 0.085s
$
```