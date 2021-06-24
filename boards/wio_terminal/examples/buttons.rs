#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle};

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{interrupt, CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{button_interrupt, entry, Button, ButtonController, ButtonEvent, Pins, Sets};

use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};

use heapless::consts::U8;
use heapless::spsc::Queue;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = Pins::new(peripherals.PORT);
    let mut sets: Sets = pins.split();

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen, load an image of Ferris from a RAW file, and draw it to the
    // screen.
    // By default, the display is in the LandscapeFlipped orientation.
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let backdrop =
        Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
    backdrop.draw(&mut display).unwrap();

    let button_ctrlr = sets.buttons.init(
        peripherals.EIC,
        &mut clocks,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    let nvic = &mut core.NVIC;
    disable_interrupts(|_| unsafe {
        button_ctrlr.enable(nvic);
        BUTTON_CTRLR = Some(button_ctrlr);
    });

    let mut consumer = unsafe { Q.split().1 };
    loop {
        if let Some(press) = consumer.dequeue() {
            let color = match press.down {
                true => Rgb565::RED,
                false => Rgb565::BLACK,
            };

            draw_button_marker(
                &mut display,
                press.button,
                PrimitiveStyleBuilder::new().fill_color(color).build(),
            );
        }
    }
}

fn draw_button_marker<D>(display: &mut D, button: Button, style: PrimitiveStyle<Rgb565>)
where
    D: DrawTarget<Color = Rgb565>,
{
    match button {
        Button::TopLeft => {
            Rectangle::with_corners(Point::new(5, 5), Point::new(5, 35))
                .into_styled(style)
                .draw(display)
                .ok();
            Triangle::new(Point::new(0, 5), Point::new(5, 0), Point::new(10, 5))
                .into_styled(style)
                .draw(display)
                .ok();
        }
        Button::TopMiddle => {
            Rectangle::with_corners(Point::new(80, 5), Point::new(80, 35))
                .into_styled(style)
                .draw(display)
                .ok();
            Triangle::new(Point::new(75, 5), Point::new(80, 0), Point::new(85, 5))
                .into_styled(style)
                .draw(display)
                .ok();
        }
        Button::Left => {
            Rectangle::with_corners(Point::new(90, 120), Point::new(120, 120))
                .into_styled(style)
                .draw(display)
                .ok();
            Triangle::new(
                Point::new(90, 115),
                Point::new(85, 120),
                Point::new(90, 125),
            )
            .into_styled(style)
            .draw(display)
            .ok();
        }
        Button::Right => {
            Rectangle::with_corners(Point::new(190, 120), Point::new(220, 120))
                .into_styled(style)
                .draw(display)
                .ok();
            Triangle::new(
                Point::new(220, 115),
                Point::new(225, 120),
                Point::new(220, 125),
            )
            .into_styled(style)
            .draw(display)
            .ok();
        }
        Button::Down => {
            Rectangle::with_corners(Point::new(160, 150), Point::new(160, 180))
                .into_styled(style)
                .draw(display)
                .ok();
            Triangle::new(
                Point::new(155, 180),
                Point::new(160, 185),
                Point::new(165, 180),
            )
            .into_styled(style)
            .draw(display)
            .ok();
        }
        Button::Up => {
            Rectangle::with_corners(Point::new(160, 60), Point::new(160, 90))
                .into_styled(style)
                .draw(display)
                .ok();
            Triangle::new(
                Point::new(155, 60),
                Point::new(160, 55),
                Point::new(165, 60),
            )
            .into_styled(style)
            .draw(display)
            .ok();
        }
        Button::Click => {
            Circle::new(Point::new(160, 120), 15)
                .into_styled(style)
                .draw(display)
                .ok();
        }
    }
}

static mut BUTTON_CTRLR: Option<ButtonController> = None;
static mut Q: Queue<ButtonEvent, U8> = Queue(heapless::i::Queue::new());

button_interrupt!(
    BUTTON_CTRLR,
    unsafe fn on_button_event(_cs: &CriticalSection, event: ButtonEvent) {
        let mut q = Q.split().0;
        q.enqueue(event).ok();
    }
);
