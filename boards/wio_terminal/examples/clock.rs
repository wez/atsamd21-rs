#![no_std]
#![no_main]

/// Makes the wio_terminal appear as a USB serial port, and display
/// the time on the screen.
///
/// You can tell the terminal the time by running (on linux):
///  - sudo stty -F /dev/ttyACM0 115200 raw -echo
///  - sudo bash -c "echo 'time=hour:minute:second' > /dev/ttyACM0"
use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyleBuilder, Rectangle};
use eg::text::Text;

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{interrupt, CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

use core::fmt::Write;
use heapless::consts::U16;
use heapless::String;
use wio::hal::rtc;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use wio::hal::usb::UsbBus;

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

    // Configure the RTC. a 1024 Hz clock is configured for us when enabling our
    // main clock
    let rtc = rtc::Rtc::clock_mode(peripherals.RTC, 1024.hz(), &mut peripherals.MCLK);

    unsafe {
        RTC = Some(rtc);
    }

    // Initialize the ILI9341-based LCD display. Create a black backdrop the size of
    // the screen.
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
    Rectangle::with_corners(Point::new(0, 0), Point::new(320, 240))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build(),
        )
        .draw(&mut display)
        .ok()
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

    let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

    loop {
        delay.delay_ms(1000 as u16);
        let time =
            disable_interrupts(|_| unsafe { RTC.as_mut().map(|rtc| rtc.current_time()) }).unwrap();

        let mut data = String::<U16>::new();
        write!(
            data,
            "{:02}:{:02}:{:02}",
            time.hours, time.minutes, time.seconds
        )
        .ok()
        .unwrap();

        Rectangle::with_corners(Point::new(55, 80), Point::new(250, 112))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::BLACK)
                    .build(),
            )
            .draw(&mut display)
            .ok()
            .unwrap();

        Text::new(data.as_str(), Point::new(55, 80), style)
            .draw(&mut display)
            .ok()
            .unwrap();
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut RTC: Option<rtc::Rtc<rtc::ClockMode>> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 32];

                if let Ok(count) = serial.read(&mut buf) {
                    // terminal.enqueue((buf, count)).ok().unwrap();
                    let mut buffer: &[u8] = &buf[..count];

                    while buffer.len() > 5 {
                        match timespec(buffer) {
                            Ok((remaining, time)) => {
                                buffer = remaining;
                                disable_interrupts(|_| {
                                    RTC.as_mut().map(|rtc| {
                                        rtc.set_time(rtc::Datetime {
                                            seconds: time.second as u8,
                                            minutes: time.minute as u8,
                                            hours: time.hour as u8,
                                            day: 0,
                                            month: 0,
                                            year: 0,
                                        });
                                    });
                                });
                            }
                            _ => break,
                        };
                    }
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

#[derive(Clone)]
pub struct Time {
    hour: u32,
    minute: u32,
    second: u32,
}

fn atoi(digits: &[u8]) -> u32 {
    let mut num: u32 = 0;
    let len = digits.len();
    for (i, digit) in digits.iter().enumerate() {
        let digit = (*digit - '0' as u8) as u32;
        let mut exp = 1;
        for _ in 0..(len - i - 1) {
            exp *= 10;
        }
        num += exp * digit;
    }
    num
}

#[macro_use]
extern crate nom;
use nom::character::streaming::digit1 as nom_ascii_digit;

nom::named!(
    pub timespec<Time>,
    do_parse!(
        opt!( char!('\n') ) >>
        tag!("time=") >>
        hour: nom_ascii_digit >>
        char!(':') >>
        minute: nom_ascii_digit >>
        char!(':') >>
        second: nom_ascii_digit >>
        tag!("\n") >>
        (
            Time{ hour: atoi(hour), minute: atoi(minute), second: atoi(second) }
        )
    )
);
