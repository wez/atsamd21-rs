#![no_std]
#![no_main]

use panic_halt as _;
use samd21_mini as hal;

use hal::clock::GenericClockController;
use hal::gpio::{OpenDrain, Output, Pa10, Pa11, Pa17, Pa27, Pb3, PfC};
use hal::pac::gclk::clkctrl::GEN_A;
use hal::pac::gclk::genctrl::SRC_A;
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom0Pad2, Sercom0Pad3, UART0};
use nb::block;
use rtic::app;

macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[app(device = crate::hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        blue_led: Pa17<Output<OpenDrain>>,
        tx_led: Pa27<Output<OpenDrain>>,
        rx_led: Pb3<Output<OpenDrain>>,
        uart: UART0<Sercom0Pad3<Pa11<PfC>>, Sercom0Pad2<Pa10<PfC>>, (), ()>,
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        let mut device = c.device;

        let mut pins = hal::Pins::new(device.PORT);

        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL48M, false);
        let gclk2 = clocks
            .get_gclk(GEN_A::GCLK2)
            .expect("Could not get clock 2");

        dbgprint!("Initializing serial port");

        let mut led = pins.led.into_open_drain_output(&mut pins.port);
        led.set_low().unwrap();

        let rx_pin: Sercom0Pad3<_> = pins
            .rx
            .into_pull_down_input(&mut pins.port)
            .into_pad(&mut pins.port);
        let tx_pin: Sercom0Pad2<_> = pins
            .tx
            .into_push_pull_output(&mut pins.port)
            .into_pad(&mut pins.port);
        let uart_clk = clocks
            .sercom0_core(&gclk2)
            .expect("Could not configure sercom0 core clock");

        let uart = UART0::new(
            &uart_clk,
            9600.Hz(),
            device.SERCOM0,
            &mut device.PM,
            (rx_pin, tx_pin),
        );

        let mut rx_led = pins.rx_led.into_open_drain_output(&mut pins.port);
        let mut tx_led = pins.tx_led.into_open_drain_output(&mut pins.port);

        tx_led.set_high().unwrap();
        rx_led.set_high().unwrap();

        dbgprint!("done init");

        init::LateResources {
            blue_led: led,
            tx_led,
            rx_led,
            uart,
        }
    }

    #[task(binds = SERCOM0, resources = [blue_led, tx_led, uart, rx_led])]
    fn SERCOM0(c: SERCOM0::Context) {
        c.resources.rx_led.set_low().unwrap();
        let data = match block!(c.resources.uart.read()) {
            Ok(v) => {
                c.resources.rx_led.set_high().unwrap();
                v
            }
            Err(_) => 0 as u8,
        };

        c.resources.tx_led.set_low().unwrap();
        match block!(c.resources.uart.write(data)) {
            Ok(_) => {
                c.resources.tx_led.set_high().unwrap();
            }
            Err(_) => unimplemented!(),
        }
    }
};
