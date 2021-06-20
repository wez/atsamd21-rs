//! Makes the pygamer appear as a USB serial port. The color of the
//! neopixel LED can be changed by sending bytes to the serial port.
//!
//! Sending the characters R, G, and O set the LED red, green, and off
//! respectively. For example:
//! $> sudo stty -F /dev/ttyACM0 115200 raw -echo
//! $> sudo bash -c "echo 'R' > /dev/ttyACM0"
//! $> sudo bash -c "echo 'G' > /dev/ttyACM0"
//! $> sudo bash -c "echo 'O' > /dev/ttyACM0"
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use hal::clock::GenericClockController;
use hal::timer::SpinTimer;
use hal::usb::UsbBus;
use pac::{interrupt, CorePeripherals, Peripherals};
use smart_leds::{colors, hsv::RGB8, SmartLedsWrite};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = Pins::new(peripherals.PORT).split();

    let timer = SpinTimer::new(4);
    let mut neopixel = pins.neopixel.init(timer);

    let _ = neopixel.write((0..5).map(|_| RGB8::default()));

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(
            pins.usb
                .init(peripherals.USB, &mut clocks, &mut peripherals.MCLK),
        );
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

    loop {
        let pending = disable_interrupts(|_| unsafe {
            let pending = PENDING_COLOR;
            PENDING_COLOR = None;
            pending
        });
        if let Some(color) = pending {
            let _ = neopixel.write((0..5).map(|_| color));
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut PENDING_COLOR: Option<RGB8> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);

                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    let last = buf[count - 1] as char;
                    let color = match last {
                        'R' => colors::RED,
                        'G' => colors::GREEN,
                        'O' => colors::ORANGE,
                        _ => RGB8::default(),
                    };

                    PENDING_COLOR = Some(color);
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
