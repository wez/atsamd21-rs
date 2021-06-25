# atsamd & atsame support for Rust

This repository holds various crates that support/enable working with Atmel `samd11`, `samd21`, `samd51` and `same5x` based devices using Rust.

![CI Build](https://github.com/atsamd-rs/atsamd/workflows/Build%20BSPs/badge.svg)

The **P**eripheral **A**ccess **C**rates (**PAC**s) are automatically generated, and provide low-level access to the peripherals specified by a device's SVD file.

The **H**ardware **A**bstraction **L**ayer (**HAL**) is the result of reading the datasheet for the device and encoding a type-safe layer over the raw PACs. This crate implements traits specified by the [embedded-hal](https://github.com/rust-embedded/embedded-hal) project, making it compatible with various drivers in the embedded rust ecosystem.

In addition to the PACs and HAL, there numerous **B**oard **S**upport **P**ackages (**BSP**s) for popular development boards. They aim to rename pins to match silk screens or Arduino pin assignments, add helpers for initialization, and re-export the `atsamd-hal` crate. These BSPs are listed beside their respective PACs below.

| Crate | Version | Board Support Packages |
|:------|:--------|:-----------------------|
| [atsamd11c](https://docs.rs/atsamd11c/) | [![Crates.io](https://img.shields.io/crates/v/atsamd11c.svg)](https://crates.io/crates/atsamd11c) |  |
| [atsamd21e](https://docs.rs/atsamd21e/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21e.svg)](https://crates.io/crates/atsamd21e) | [Gemma M0][gemma_m0], [Serpente][serpente], [Trinket M0][trinket_m0] |
| [atsamd21g](https://docs.rs/atsamd21g/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21g.svg)](https://crates.io/crates/atsamd21g) | [Arduino Nano 33 IOT][arduino_nano33_iot], [Circuit Playground Express][circuit_playground_express], [Feather M0][feather_m0], [Metro M0][metro_m0], [MKR1000][arduino_mkr1000], [MKR Vidor 4000][arduino_mkr_vidor_4000], [MKR ZERO][arduino_mkrzero], [SAMD21 Mini][samd21_mini], [SODAQ ONE][sodaq_one], [Wio Lite MG126][wio_lite_mg126], [Xiao M0][xiao_m0] |
| [atsamd21j](https://docs.rs/atsamd21j/) | [![Crates.io](https://img.shields.io/crates/v/atsamd21j.svg)](https://crates.io/crates/atsamd21j) | [SODAQ SARA AFF][sodaq_sara_aff] |
| [atsamd51g](https://docs.rs/atsamd51g/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51g.svg)](https://crates.io/crates/atsamd51g) | [ItsyBitsy M4][itsybitsy_m4], [Trellis M4][trellis_m4] |
| [atsamd51j](https://docs.rs/atsamd51j/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51j.svg)](https://crates.io/crates/atsamd51j) | [EdgeBadge][edgebadge], [Feather M4][feather_m4], [Metro M4][metro_m4], [PyPortal][pyportal] |
| [atsame51n](https://docs.rs/atsame51n/) | [![Crates.io](https://img.shields.io/crates/v/atsame51n.svg)](https://crates.io/crates/atsame51n) |  |
| [atsamd51p](https://docs.rs/atsamd51p/) | [![Crates.io](https://img.shields.io/crates/v/atsamd51p.svg)](https://crates.io/crates/atsamd51p) | [Grand Central M4 Express][grand_central_m4], [Wio Terminal][wio_terminal] |
| [atsame51g](https://docs.rs/atsame51g/) | [![Crates.io](https://img.shields.io/crates/v/atsame51g.svg)](https://crates.io/crates/atsame51g) |  |
| [atsame51j](https://docs.rs/atsame51j/) | [![Crates.io](https://img.shields.io/crates/v/atsame51j.svg)](https://crates.io/crates/atsame51j) |  |
| [atsame51n](https://docs.rs/atsame51n/) | [![Crates.io](https://img.shields.io/crates/v/atsame51n.svg)](https://crates.io/crates/atsame51n) |  |
| [atsame53j](https://docs.rs/atsame53j/) | [![Crates.io](https://img.shields.io/crates/v/atsame53j.svg)](https://crates.io/crates/atsame53j) |  |
| [atsame53n](https://docs.rs/atsame53n/) | [![Crates.io](https://img.shields.io/crates/v/atsame53n.svg)](https://crates.io/crates/atsame53n) |  |
| [atsame54n](https://docs.rs/atsame54n/) | [![Crates.io](https://img.shields.io/crates/v/atsame54n.svg)](https://crates.io/crates/atsame54n) |  |
| [atsame54p](https://docs.rs/atsame54p/) | [![Crates.io](https://img.shields.io/crates/v/atsame54p.svg)](https://crates.io/crates/atsame54p) | [PathfinderZA Proto1][pfza_proto1], [SAM E54 Xplained Pro Evaluation Kit][atsame54_xpro] |
| [atsamd-hal](https://docs.rs/atsamd_hal/) | [![Crates.io](https://img.shields.io/crates/v/atsamd_hal.svg)](https://crates.io/crates/atsamd_hal) |  |

[arduino_mkr1000]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_mkr1000
[arduino_mkr_vidor_4000]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_mkrvidor4000
[arduino_mkrzero]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_mkrzero/
[arduino_nano33_iot]: https://github.com/atsamd-rs/atsamd/tree/master/boards/arduino_nano33iot
[atsame54_xpro]: https://github.com/atsamd-rs/atsamd/tree/master/boards/atsame54_xpro/
[circuit_playground_express]: https://github.com/atsamd-rs/atsamd/tree/master/boards/circuit_playground_express/
[edgebadge]: https://github.com/atsamd-rs/atsamd/tree/master/boards/edgebadge
[feather_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0/
[feather_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m4/
[gemma_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/gemma_m0/
[grand_central_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/grand_central_m4/
[itsybitsy_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m0/
[itsybitsy_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m4/
[metro_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m0/
[metro_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m4/
[pfza_proto1]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pfza_proto1/
[pygamer]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pygamer/
[pyportal]: https://github.com/atsamd-rs/atsamd/tree/master/boards/pyportal/
[samd21_mini]: https://github.com/atsamd-rs/atsamd/tree/master/boards/samd21_mini/
[serpente]: https://github.com/atsamd-rs/atsamd/tree/master/boards/serpente/
[sodaq_one]: https://github.com/atsamd-rs/atsamd/tree/master/boards/sodaq_one/
[sodaq_sara_aff]: https://github.com/atsamd-rs/atsamd/tree/master/boards/sodaq_sara_aff/
[trellis_m4]: https://github.com/atsamd-rs/atsamd/tree/master/boards/trellis_m4/
[trinket_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/trinket_m0/
[wio_lite_mg126]: https://github.com/atsamd-rs/atsamd/tree/master/boards/wio_lite_mg126
[wio_terminal]: https://github.com/atsamd-rs/atsamd/tree/master/boards/wio_terminal
[xiao_m0]: https://github.com/atsamd-rs/atsamd/tree/master/boards/xiao_m0

## Building

Make sure that you have a new enough version of the gcc toolchain; the one installable even on recent versions of Ubuntu can fail to correctly link the vector table:

**Note**: you may be able to avoid this step if using `cargo-binutils`. See board specific instructions in `boards/` for more details.

```bash
$ sudo add-apt-repository ppa:team-gcc-arm-embedded/ppa -y
$ sudo apt update
$ sudo apt install gcc-arm-embedded
```

You'll need to add the proper compilation target prior to building as well:

```bash
$ # for samd11, samd21:
$ rustup target add thumbv6m-none-eabi
$ # for samd51, same51, same53, same54:
$ rustup target add thumbv7em-none-eabihf
```

Since a number of different MCUs are used, building the examples requires changing directory into one of the board support package directories prior to building. For example:

```bash
$ cd metro_m0
$ cargo build --examples
$ cd ../gemma_m0
$ cargo build --examples
```

## How to use a BSP (i.e. getting started writing your own code)

To bootstrap your own project you should be able to copy/paste the Rust code from the examples folder within the folder of the BSP you've chosen. But you shouldn't copy the `Cargo.toml` file from there, since that's not only used for the examples, but also for the whole BSP itself. You want to make your own `Cargo.toml` file. If you're new to this and have no clue what you're doing then this is probably the line you want in there:

```rust
[dependencies]
feather_m0 = { git = "https://github.com/atsamd-rs/atsamd" }
```

Replace "feather_m0" with the name of the BSP you want to use. Note a few things:
* By specifying the dependency as a remote git repo without specifying a branch, rather than pinning a specific version, we ask Cargo to always grab the latest master branch from Github. This is probably what you want in the beginning, at least now when the project is young, but expect that you'll later want to pin down a specific version.
* There's a whole bunch of crates inside that git repo, and Cargo automatically figures out which one you want to use by checking what you're naming the dependency. In this case we named it "feather_m0" so Cargo will try to find a BSP called that, somewhere inside the repo.

Now the imports in your Rust code should work, if they look something like this:
```rust
use feather_m0 as bsp;
use bsp::hal;
use bsp::pac;
```

You should now have objects called `bsp` and `hal` and `pac`, which contain all the good stuff.

## Getting code onto the device: Adafruit M0/M4 board (such as Gemma M0 & Feather M0)

If you want to flash the device using the tools that come with the Adafruit arduino support package:

```bash
$ cd gemma_m0
$ cargo build --example blinky_basic
$ arm-none-eabi-objcopy -O binary \
    target/thumbv6m-none-eabi/debug/examples/blinky_basic \
    target/thumbv6m-none-eabi/debug/examples/blinky_basic.bin
# if using cargo-binutils, you can `rust-objcopy` with the same flags, or combine the previous 2 steps with `cargo objcopy`
$ stty -F /dev/ttyACM1 ospeed 1200
$ ~/.arduino15/packages/arduino/tools/bossac/1.7.0/bossac -i -d \
    --port=ttyACM1 -U -e -w -v \
    target/thumbv6m-none-eabi/debug/examples/blinky_basic.bin -R
```

This same technique should work for all of the Adafruit M0/M4 boards, as they all ship with a bossac compatible  bootloader. Note that M0 devices may need `-o 0x2000` and M4 devices may need `-o 0x4000` added to the `bossac`  parameter lists.

## Getting code onto the device: JLink

If you have a board with a SWD debug header, such as the [Metro M0][metro_m0], or if you attached the header yourself, you can use your JLink programmer to either flash the device or debug it (together with gdb).

You can use JFlashLiteExe to just flash the .bin file (note that for some boards, such as [Feather M0][feather_m0], you need to specify the program memory starting address offset or you'll just get a nondescript flashing error).

To debug your board you can run JLinkGDBServer instead of JFlashLiteExe. @wez prefers using the JLinkGDBServer, but you can also use OpenOCD. Note that if you have a `load` command within your .gdbinit then starting a debug session triggers a flashing, so you don't need a separate flashing step before this.

In one window, run `JLinkGDBServer -if SWD -device ATSAMD21G18`, then in another, run these commands from the root of this repo so that you pick up its `.gdbinit` file:

```bash
$ cargo build --manifest-path metro_m0/Cargo.toml --example blinky_basic
$ arm-none-eabi-gdb metro_m0/target/thumbv6m-none-eabi/debug/examples/blinky_basic
```

If you prefer or otherwise need to use OpenOCD, then you'd run it in place of the JLinkGDBServer and then modify the `.gdbinit` file to comment out the JLink section and uncomment the OpenOCD section.

## Getting code onto the devices with bootloaders: hf2-rs

This is the preferred pure rust ecosystem method for interacting with bootloaders. 

[hf2-rs](https://github.com/jacobrosenthal/hf2-rs) implements [Microsofts HID Flashing Format (HF2)](https://github.com/microsoft/uf2/blob/86e101e3a282553756161fe12206c7a609975e70/hf2.md) to upload firmware to UF2 bootloaders. UF2 is factory programmed extensively by [Microsoft MakeCode](https://www.microsoft.com/en-us/makecode) and [Adafruit](https://www.adafruit.com/) hardware.

The `cargo-hf2` crate replaces the `cargo build` command to include flashing over USB to connected UF2 devices, using hf2 flashing over HID protocol.

```bash
$ cargo install cargo-hf2
```

and from a bsp directory
```
$ cargo hf2 --example blinky_basic --features unproven --release
```

For more information, refer to the `README` files for each crate:
* [hf2 library (`hf2`)](https://github.com/jacobrosenthal/hf2-rs/tree/master/hf2)
* [hf2 binary (`hf2-cli`)](https://github.com/jacobrosenthal/hf2-rs/tree/master/hf2-cli)
* [hf2 cargo subcommand (`hf2-cargo`)](https://github.com/jacobrosenthal/hf2-rs/tree/master/cargo-hf2)

## Getting code onto the devices with bootloaders: uf2conv-rs

[uf2conv](https://github.com/sajattack/uf2conv-rs) adds a uf2 header [Microsofts HID Flashing Format (UF2)](https://github.com/microsoft/uf2/blob/86e101e3a282553756161fe12206c7a609975e70/README.md) for copying over to UF2 bootloader mass storage devices. UF2 is factory programmed extensively by [Microsoft MakeCode](https://www.microsoft.com/en-us/makecode) and [Adafruit](https://www.adafruit.com/) hardware.
[cargo-binutils](https://github.com/rust-embedded/cargo-binutils) replaces the `cargo build` command to find and convert elf files into binary. 

Install the dependencies
```bash
$ rustup component add llvm-tools-preview
$ cargo install uf2conv cargo-binutils
```

Then for say, metro_m0 examples
```bash
$ cargo objcopy --example blinky_basic --features unproven --release -- -O binary blinky_basic.bin
$ uf2conv blinky_basic.bin --base 0x2000 --output blinky_basic.uf2
$ cp blinky_basic.uf2 /Volumes/PYGAMERBOOT/
```

For more information, refer to the `README` files for each crate:
* [uf2conv (`uf2conv-rs`)](https://github.com/sajattack/uf2conv-rs)
* [cargo-binutils (`cargo-binutils`)](https://github.com/rust-embedded/cargo-binutils)

## Getting code onto the device with bootloaders: bossac

If you want to flash the device using the tools that come with the Adafruit arduino support package:

```bash
$ cd gemma_m0
$ cargo build --example blinky_basic
$ arm-none-eabi-objcopy -O binary \
    target/thumbv6m-none-eabi/debug/examples/blinky_basic \
    target/thumbv6m-none-eabi/debug/examples/blinky_basic.bin
# if using cargo-binutils, you can `rust-objcopy` with the same flags, or combine the previous 2 steps with `cargo objcopy`
$ stty -F /dev/ttyACM1 ospeed 1200
$ ~/.arduino15/packages/arduino/tools/bossac/1.7.0/bossac -i -d \
    --port=ttyACM1 -U -e -w -v \
    target/thumbv6m-none-eabi/debug/examples/blinky_basic.bin -R
```

This same technique should work for all of the Adafruit M0/M4 boards, as they all ship with a bossac compatible  bootloader. Note that M0 devices may need `-o 0x2000` and M4 devices may need `-o 0x4000` added to the `bossac`  parameter lists.

## Getting code onto the device with debugger: cargo-flash

This is the preferred pure rust ecosystem method for flashing with debugger.

[cargo flash](https://github.com/probe-rs/cargo-flash) replaces the `cargo build` command to include flashing over debugger using probe-rs and libusb.

```bash
$ cargo install cargo-flash
```
We need to know the specific id of your device's chip. Luckily adafruit lists ATSAMD21G18 for metro_m0
```bash
$ cargo flash --list-chips | grep ATSAMD21G18
        ATSAMD21G18A
        ATSAMD21G18AU
```

You can stash this chip in the cargo toml so you never have to pass it as an argument, which we recommend.
```
# for cargo flash
[package.metadata]
chip = "ATSAMD21G18A"
```

And cargo flash simply replaces your cargo build command!
```bash
$ cargo flash --example blinky_basic --features unproven --release
```

Or you can provide it via the chip command line argument
```bash
$ cargo flash --example blinky_basic --features unproven --release --chip ATSAMD21G18A
```

## Debugging: probe-run

This is the preferred pure rust ecosystem method for debugging. It requires no external gdb server, nor C or Python tooling like openocd.

[probe-run](https://github.com/probe-rs/cargo-flash) attemps to bring the hosted cargo run print line debugging experience to embedded. It also has advanced logging features to vastly reduce format size under the [defmt](https://github.com/knurling-rs/defmt) project which is not covered here.

`probe-run` needs to be set as your `runner` in the `.cargo/config` along with the id of your chip. Also debug symbols need to be enabled for any profile you're building for. In your application you'll want to use a `probe-run` compatible panic crate like `panic-probe` and an rtt debug logging crate like `rtt-target`. Also don't forget to init your rtt machinery.

`probe-run` will then be called after a successful build to flash the code directly to the target via debugger and will then wait to receive any rtt prints from your target. Finally if a panic occurs or you ever call `cortex_m::asm::bkpt()` `probe-run` will detect, print a stack trace, and exit. You can exit `probe-run` on the host side with ctrl-c.

```bash
$ cargo install probe-run
```

Then simply use your ide's run or play button, or run:
```bash
$ cargo run --release --example adc --features=unproven
    Finished release [optimized + debuginfo] target(s) in 1.10s
     Running `probe-run --chip ATSAMD11C14A target/thumbv6m-none-eabi/release/examples/adc`
  (HOST) INFO  flashing program (35.17 KiB)
  (HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
4129
4074
4177
^Cstack backtrace:
   0: <atsamd_hal::common::delay::Delay as embedded_hal::blocking::delay::DelayUs<u32>>::delay_us
        at /home/atsamd/hal/src/common/delay.rs:72
   1: <atsamd_hal::common::delay::Delay as embedded_hal::blocking::delay::DelayMs<u32>>::delay_ms
        at /home/atsamd/hal/src/common/delay.rs:35
   2: <atsamd_hal::common::delay::Delay as embedded_hal::blocking::delay::DelayMs<u16>>::delay_ms
        at /home/atsamd/hal/src/common/delay.rs:41
   3: neopixel_adc_battery::__cortex_m_rt_main
        at examples/neopixel_adc_battery.rs:50
   4: main
        at examples/neopixel_adc_battery.rs:23
   5: ResetTrampoline
        at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:547
   6: Reset
        at /home/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:550
```

## Debugging: JLink

If you have a board with a SWD debug header, such as the [Metro M0][metro_m0], or if you attached the header yourself, you can use your JLink together with gdb. @wez prefers using the JLinkGDBServer, but you can also use OpenOCD.

In one window, run `JLinkGDBServer -if SWD -device ATSAMD21G18`, then in another, run these commands from the root   of this repo so that you pick up its `.gdbinit` file:

```bash
$ cargo build --manifest-path metro_m0/Cargo.toml --example blinky_basic
$ arm-none-eabi-gdb metro_m0/target/thumbv6m-none-eabi/debug/examples/blinky_basic
```

If you prefer or otherwise need to use OpenOCD, then you'd run it in place of the JLinkGDBServer and then modify the `.gdbinit` file to comment out the JLink section and uncomment the OpenOCD section.

## Adding a new board

See our wiki page for a [complete guide](https://github.com/atsamd-rs/atsamd/wiki/Adding-a-new-board) on adding a new board.

### Building everything locally

If you'd like to build all the same things that the CI would build but on your local system, you can run:

```bash
$ ./build-all.py
```

Please note that this script requires Python 3.

## License

The included SVD files are sourced from http://packs.download.atmel.com/ and
are licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0).

The remainder of the code is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
