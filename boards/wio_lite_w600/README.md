# WIO Lite W600 Board Support Crate

This crate provides a type-safe API for working with the WIO Lite W600(https://wiki.seeedstudio.com/Wio-Lite-W600/).

## Examples
### Blinky Basic
#### Requirements
 - Arduino IDE installed
   - You will also need the Wio Lite W600 Arduino tools, [follow the guide here](https://wiki.seeedstudio.com/Wio-Lite-W600/#software).
   - This library contains the version of bossac you will need in `Arduino15/packages/Seeeduino/tools/bossac/1.7.0[-arduino3]/`.
     It's very important that you use _this_ version only, as it is possible to lock yourself out of the board with the
     wrong one. You can find Arduino15 in the following locations:
       - **linux**: `Arduino15` is likely something like `~/.arduino15/`
       - **OSX**: `Arduino15` is likely something like `~/Library/Arduino15`
   - You can test that everything is working by installing an example sketch via the IDE
   - Note that the [arduino cli](https://github.com/arduino/arduino-cli) (or just regular bossac) may soon replace this
      section
 - arm-none-eabi tools installed, you need gcc and objcopy.
   -  **Note**: Alternatively, you can use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils), which is
      likely easier to install on OSX and also easier to use, as it will automatically detect the target
 - thumbv6m-none-eabi rust target installed via `rustup target add thumbv6m-none-eabi`

#### Steps

To build your binary, you can either use `arm-none-eabi-objcopy` or `cargo-binutils`:

If using `arm-none-eabi-objcopy`
```bash
cargo build --release --example blinky_basic
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin
```
Or, if using  `cargo-binutils`
```bash
cargo objcopy --example blinky_basic --release -- -O binary target/blinky_basic.bin
```

To copy the file across, first you need to find the port the Wio Lite W600 is attached to. This can be done easily in
the Arduino IDE or on nix based systems, you can find it inside the `/dev` directory. Note, it might not show up,
until you put the device into bootloader mode by double tapping the reset button on the board.

For example, if your device attaches to `/dev/cu.usbmodem14601` you will need `cu.usbmodem14601` below.

You will also need to put the device into bootloader mode before copying the file by double tapping the reset button.
Using the bossac you found above, with the previously found port that the device is attached to. 

Example:
```bash
~/Library/Arduino15/packages/Seeeduino/tools/bossac/1.7.0-arduino3/bossac --port=cu.usbmodem14601 -idewvRU target/blinky_basic.bin
```
