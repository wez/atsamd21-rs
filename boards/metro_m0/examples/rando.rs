#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate metro_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
// extern crate ssd1331;
extern crate sx1509;

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

use hal::clock::GenericClockController;
use hal::delay::Delay;
// use hal::gpio;
use hal::pac::CorePeripherals;
use hal::prelude::*;
use hal::sercom::{I2CMaster3, SPIMaster5};
use hal::time::Nanoseconds;
use rtic::app;

/*
type Display = ssd1331::Ssd1331<
    SPIMaster4,
    gpio::Pa7<gpio::Output<gpio::PushPull>>,  // DC
    gpio::Pa18<gpio::Output<gpio::PushPull>>, // CS
>;
*/

#[app(device = crate::hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        RED_LED: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>,
        I2C: I2CMaster3<
            hal::sercom::Sercom3Pad0<hal::gpio::Pa22<hal::gpio::PfC>>,
            hal::sercom::Sercom3Pad1<hal::gpio::Pa23<hal::gpio::PfC>>,
        >,
        FLASH: SPIMaster5<
            hal::sercom::Sercom5Pad1<hal::gpio::Pb3<hal::gpio::PfD>>,
            hal::sercom::Sercom5Pad2<hal::gpio::Pb22<hal::gpio::PfD>>,
            hal::sercom::Sercom5Pad3<hal::gpio::Pb23<hal::gpio::PfD>>,
        >,
        SX1509: sx1509::Sx1509<
            I2CMaster3<
                hal::sercom::Sercom3Pad0<hal::gpio::Pa22<hal::gpio::PfC>>,
                hal::sercom::Sercom3Pad1<hal::gpio::Pa23<hal::gpio::PfC>>,
            >,
        >,
        // SPI: SPIMaster4,
        // SSD1131: Display,
        TIMER: hal::timer::TimerCounter3,
    }

    #[task(binds = TC3, resources = [TIMER, RED_LED])]
    fn TC3(ctx: TC3::Context) {
        if ctx.resources.TIMER.wait().is_ok() {
            ctx.resources.RED_LED.toggle();
        }
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        let interval = 1u32.Hz();

        let mut device = c.device;
        let core = CorePeripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );
        let gclk0 = clocks.gclk0();
        let mut pins = hal::Pins::new(device.PORT);

        let mut delay = Delay::new(core.SYST, &mut clocks);

        // in-line query of the on-board SPI flash to determine the JEDEC id
        let (mut flash, mut flash_cs) = hal::flash_spi_master(
            &mut clocks,
            device.SERCOM5,
            &mut device.PM,
            pins.flash_sck,
            pins.flash_mosi,
            pins.flash_miso,
            pins.flash_cs,
            &mut pins.port,
        );
        delay.delay_ms(200u8);
        flash_cs.set_low().unwrap();

        let mut buf = [0x9f, 0, 0, 0];

        let _res = flash.transfer(&mut buf);
        dbgprint!("tx result {}", res.is_ok());
        flash_cs.set_high().unwrap();

        /*
            let mut spi = SPIMaster4::new(
                &clocks.sercom4_core(&gclk0).unwrap(),
                24.mhz(),
                hal::hal::spi::Mode {
                    phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
                    polarity: hal::hal::spi::Polarity::IdleLow,
                },
                device.SERCOM4,
                &mut device.PM,
                // Metro M0 express has SPI on these pins
                hal::sercom::SPI4Pinout::Dipo0Dopo1 {
                    miso: pins.pa12.into_pad(&mut pins.port),
                    mosi: pins.pb10.into_pad(&mut pins.port),
                    sck: pins.pb11.into_pad(&mut pins.port),
                },
            );

            dbgprint!("made spi");

            let mut reset_pin = pins.pa6.into_push_pull_output(&mut pins.port);

            let mut display = Display::new(
                &mut spi,
                pins.pa7.into_push_pull_output(&mut pins.port),
                pins.pa18.into_push_pull_output(&mut pins.port),
            );

            display.reset(&mut reset_pin, &mut delay);
            display.borrow(&mut spi).initialize().is_ok();

            use ssd1331::Command;
            let cmds = [
                Command::DrawRect {
                    x1: 1,
                    y1: 1,
                    x2: 63,
                    y2: 63,
                    r: 0xff,
                    g: 0xff,
                    b: 0xff,
                    fill_r: 0xa0,
                    fill_g: 0,
                    fill_b: 0xa0,
                },
                Command::DrawLine {
                    x1: 0,
                    y1: 0,
                    x2: 10,
                    y2: 10,
                    r: 0x0,
                    g: 0xff,
                    b: 0,
                },
            ];

            for cmd in &cmds {
                let spires = cmd.send(&mut display.borrow(&mut spi));
                if !spires.is_ok() {
                    dbgprint!("fail: {:?}", cmd);
                }
                delay.delay_ms(3u8);
            }
        */

        let mut i2c = hal::i2c_master(
            &mut clocks,
            400_000.Hz(),
            device.SERCOM3,
            &mut device.PM,
            pins.sda,
            pins.scl,
            &mut pins.port,
        );

        let mut expander = sx1509::Sx1509::new(&mut i2c, sx1509::DEFAULT_ADDRESS);

        dbgprint!("do first write");
        // Let's try to init an sx1509 attached to the i2c bus
        let _res1 = expander.borrow(&mut i2c).software_reset();
        dbgprint!("send reset {:?}", res1.is_ok());

        let res3 = expander
            .borrow(&mut i2c)
            .read_16(sx1509::Register::RegInterruptMaskA);
        match res3 {
            Err(_e) => dbgprint!("read intmaska fail {:?}", e),
            Ok(_val) => dbgprint!("read intmaska {:x}", val),
        };

        let mut tc3 = hal::timer::TimerCounter::tc3_(
            &clocks.tcc2_tc3(&gclk0).unwrap(),
            device.TC3,
            &mut device.PM,
        );
        dbgprint!("start timer");
        tc3.start(interval.to_duration::<Nanoseconds>().unwrap());
        tc3.enable_interrupt();

        dbgprint!("done init");
        init::LateResources {
            RED_LED: pins.d13.into_open_drain_output(&mut pins.port),
            I2C: i2c,
            SX1509: expander,
            TIMER: tc3,
            // SPI: spi,
            // SSD1131: display,
            FLASH: flash,
        }
    }
};
