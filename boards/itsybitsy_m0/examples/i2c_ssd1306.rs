#![no_std]
#![no_main]

extern crate itsybitsy_m0 as hal;
extern crate panic_halt;

extern crate embedded_graphics;
extern crate ssd1306;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle};
use embedded_graphics::style::PrimitiveStyleBuilder;
use ssd1306::prelude::*;
use ssd1306::Builder;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let i2c = hal::i2c_master(
        &mut clocks,
        400_000.Hz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );

    // NOTE the `DisplaySize` enum comes from the ssd1306 package,
    // and currently only supports certain display sizes; see
    // https://jamwaffles.github.io/ssd1306/master/ssd1306/prelude/enum.DisplaySize.html
    let mut disp: GraphicsMode<_> = Builder::new()
        .size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Rectangle::new(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Circle::new(Point::new(96, 16 + 8), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
        red_led.set_low().unwrap();
    }
}
