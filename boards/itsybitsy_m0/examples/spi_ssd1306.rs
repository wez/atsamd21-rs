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
use hal::time::MegaHertz;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyleBuilder, Rectangle};
use ssd1306::{prelude::*, Ssd1306};

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

    let spi = hal::spi_master(
        &mut clocks,
        MegaHertz(10),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );

    let dc = pins.d9.into_open_drain_output(&mut pins.port);
    let mut rst = pins.d7.into_open_drain_output(&mut pins.port);

    // NOTE the `DisplaySize` enum comes from the ssd1306 package,
    // and currently only supports certain display sizes; see
    // https://jamwaffles.github.io/ssd1306/master/ssd1306/prelude/enum.DisplaySize.html
    let interface = SPIInterfaceNoCS::new(spi, dc);
    let mut disp = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    disp.reset(&mut rst, &mut delay).unwrap();
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

    Rectangle::with_corners(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
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
