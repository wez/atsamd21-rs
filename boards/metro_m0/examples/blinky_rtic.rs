#![no_std]
#![no_main]

use metro_m0 as hal;

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::time::Nanoseconds;
use rtic::app;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

#[cfg(feature = "use_semihosting")]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m_semihosting::hio;
            use core::fmt::Write;
            let mut stdout = hio::hstdout().unwrap();
            writeln!(stdout, $($arg)*).ok();
        }
    };
}

#[cfg(not(feature = "use_semihosting"))]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[app(device = crate::hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        red_led: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>,
        timer: hal::timer::TimerCounter3,
    }

    /// This function is called each time the tc3 interrupt triggers.
    /// We use it to toggle the LED.  The `wait()` call is important
    /// because it checks and resets the counter ready for the next
    /// period.
    #[task(binds = TC3, resources = [timer, red_led])]
    fn tc3(c: tc3::Context) {
        if c.resources.timer.wait().is_ok() {
            c.resources.red_led.toggle();
        }
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        let mut device = c.device;

        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );
        let gclk0 = clocks.gclk0();
        let mut pins = hal::Pins::new(device.PORT);

        let mut tc3 = hal::timer::TimerCounter::tc3_(
            &clocks.tcc2_tc3(&gclk0).unwrap(),
            device.TC3,
            &mut device.PM,
        );
        dbgprint!("start timer");
        tc3.start(1u32.Hz().to_duration::<Nanoseconds>().unwrap());
        tc3.enable_interrupt();

        dbgprint!("done init");

        init::LateResources {
            red_led: pins.d13.into_open_drain_output(&mut pins.port),
            timer: tc3,
        }
    }
};
