# ATSAMD11C Support Crate

This crate provides a type-safe API for working with the [ATSAMD11C](https://www.microchip.com/wwwproducts/en/ATSAMD11C14).

## Examples?

Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/samd11_bare/examples


## Getting code onto the device with debugger: cargo-flash

This is the preferred pure rust ecosystem method for flashing with debugger.

[cargo flash](https://github.com/probe-rs/cargo-flash) replaces the `cargo build` command to include flashing over debugger using probe-rs and libusb.

```bash
$ cargo install cargo-flash
```

Cargo flash needs to know the specific id of your chip, but its included in the package.metadata field of the Cargo.toml so you can omit it.

Then cargo flash simply replaces your cargo build command!
```bash
$ cargo flash --example blinky_basic --features unproven --release
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
    Finished release [optimized + debuginfo] target(s) in 0.99s
     Running `probe-run --chip ATSAMD11C14A target\thumbv6m-none-eabi\release\examples\adc`
  (HOST) INFO  flashing program (7.18 KiB)
  (HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
3828
3829
414
413
419
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
