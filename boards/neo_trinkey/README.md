# Adafruit Neo Trinkey Board Support Crate

This crate provides a type-safe API for working with the [Adafruit Neo Trinkey
board](https://www.adafruit.com/product/4870).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/neo_trinkey/examples

* Be in this directory `cd boards/neo_trinkey`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```bash
$ cargo hf2 --release --example blinky_basic --pid 0x00ef --vid 0x239a
```
You should see the following output
```text
Finished release [optimized] target(s) in 5.55s
Trying  Ok(Some("Adafruit Industries")) Ok(Some("NeoPixel Trinkey M0"))
Flashing "/Users/danielmason/projects/rust/atsamd/boards/neo_trinkey/target/thumbv6m-none-eabi/release/examples/blinky_basic"
Finished in 0.051s
```
Note: If hf2 can not find your Neo Trinkey, you should check the Product ID (pid) and Vendor ID (vid) in your system
settings.
