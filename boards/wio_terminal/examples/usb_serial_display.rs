#![no_std]
#![no_main]

/// Makes the wio_terminal appear as a USB serial port. The screen can
/// be written to by sending messages down the serial port.
use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyleBuilder, Rectangle};
use eg::text::Text;

use cortex_m::peripheral::NVIC;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{interrupt, CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};
use wio::{Scroller, LCD};

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use wio::hal::usb::UsbBus;

use heapless::consts::U16;
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

    let mut user_led = sets.user_led.into_open_drain_output(&mut sets.port);
    user_led.set_low().unwrap();

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen.
    let (display, _backlight) = sets
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

    // Initialize USB.
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(sets.usb.usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.MCLK,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };
    unsafe {
        USB_SERIAL = Some(SerialPort::new(&bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Serial port")
                .serial_number("TEST")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }
    unsafe {
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }

    let mut t = Terminal::new(display);
    t.write_str("Hello! Send text to me over the USB serial port, and I'll display it!");
    t.write_str("\n");
    t.write_str("On linux:\n");
    t.write_str("  sudo stty -F /dev/ttyACM0 115200 raw -echo\n");
    t.write_str("  sudo bash -c \"echo 'Hi' > /dev/ttyACM0\"\n");

    let mut consumer = unsafe { Q.split().1 };
    loop {
        if let Some(segment) = consumer.dequeue() {
            t.write(segment);
            user_led.toggle();
        }
    }
}

type TextSegment = ([u8; 32], usize);

struct Terminal<'a> {
    text_style: MonoTextStyle<'a, Rgb565>,
    cursor: Point,
    display: LCD,
    scroller: Scroller,
}

impl<'a> Terminal<'a> {
    pub fn new(mut display: LCD) -> Self {
        // Clear the screen.
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let backdrop =
            Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
        backdrop.draw(&mut display).ok().unwrap();

        let scroller = display.configure_vertical_scroll(0, 0).unwrap();

        Self {
            text_style: MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
            cursor: Point::new(0, 0),
            display,
            scroller,
        }
    }

    pub fn write_str(&mut self, str: &str) {
        for character in str.chars() {
            self.write_character(character);
        }
    }

    pub fn write_character(&mut self, c: char) {
        if self.cursor.x >= 320 || c == '\n' {
            self.cursor = Point::new(0, self.cursor.y + FONT_6X12.character_size.height as i32);
        }
        if self.cursor.y >= 240 {
            self.animate_clear();
            self.cursor = Point::new(0, 0);
        }

        if c != '\n' {
            let mut buf = [0u8; 8];
            Text::new(c.encode_utf8(&mut buf), self.cursor, self.text_style)
                .draw(&mut self.display)
                .ok()
                .unwrap();

            self.cursor.x += (FONT_6X12.character_size.width + FONT_6X12.character_spacing) as i32;
        }
    }

    pub fn write(&mut self, segment: TextSegment) {
        let (buf, count) = segment;
        for (i, character) in buf.iter().enumerate() {
            if i >= count {
                break;
            }
            self.write_character(*character as char);
        }
    }

    fn animate_clear(&mut self) {
        for x in (0..320).step_by(FONT_6X12.character_size.width as usize) {
            self.display
                .scroll_vertically(&mut self.scroller, FONT_6X12.character_size.width as u16)
                .ok()
                .unwrap();
            Rectangle::with_corners(
                Point::new(x + 0, 0),
                Point::new(x + FONT_6X12.character_size.width as i32, 240),
            )
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::BLACK)
                    .build(),
            )
            .draw(&mut self.display)
            .ok()
            .unwrap();

            for _ in 0..1000 {
                cortex_m::asm::nop();
            }
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut Q: Queue<TextSegment, U16> = Queue(heapless::i::Queue::new());

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 32];
                let mut terminal = Q.split().0;

                if let Ok(count) = serial.read(&mut buf) {
                    terminal.enqueue((buf, count)).ok().unwrap();
                };
            });
        });
    };
}

#[interrupt]
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}
