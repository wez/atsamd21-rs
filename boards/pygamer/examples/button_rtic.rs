#![no_std]
#![no_main]

use bsp::{hal, ButtonReader, Keys, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use hal::prelude::*;
use rtic::app;

#[app(device = crate::hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        red_led: RedLed,
        timer: hal::timer::TimerCounter3,
        buttons: ButtonReader,
    }

    /// This function is called each time the tc3 interrupt triggers.
    /// We use it to toggle the LED.  The `wait()` call is important
    /// because it checks and resets the counter ready for the next
    /// period.
    #[task(binds = TC3, resources = [timer, red_led, buttons])]
    fn tc3(c: tc3::Context) {
        if c.resources.timer.wait().is_ok() {
            for event in c.resources.buttons.events() {
                match event {
                    Keys::SelectDown => {
                        c.resources.red_led.set_high().ok();
                    }
                    Keys::SelectUp => {
                        c.resources.red_led.set_low().ok();
                    }
                    _ => {}
                }
            }
        }
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        let mut device = c.device;
        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.MCLK,
            &mut device.OSC32KCTRL,
            &mut device.OSCCTRL,
            &mut device.NVMCTRL,
        );

        let pins = Pins::new(device.PORT).split();

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();

        let mut tc3 = hal::timer::TimerCounter::tc3_(&timer_clock, device.TC3, &mut device.MCLK);

        tc3.start(200.hz());
        tc3.enable_interrupt();

        init::LateResources {
            buttons: pins.buttons.init(),
            red_led: pins.led_pin.into(),
            timer: tc3,
        }
    }
};
