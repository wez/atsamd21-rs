#![no_std]
#![no_main]

/// Makes the itsybitsy_m4 appear as a USB serial port. The color of the
/// dotstar LED can be changed by sending bytes to the serial port.
///
/// Sending the characters R, G, and O set the LED red, green, and off
/// respectively. For example:
/// $> sudo stty -F /dev/ttyACM0 115200 raw -echo
/// $> sudo bash -c "echo 'R' > /dev/ttyACM0"
/// $> sudo bash -c "echo 'G' > /dev/ttyACM0"
/// $> sudo bash -c "echo 'O' > /dev/ttyACM0"
extern crate itsybitsy_m4 as hal;
extern crate panic_halt;

use hal::clock::GenericClockController;

use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use hal::entry;
use hal::pac::{interrupt, CorePeripherals, Peripherals};

use hal::usb::UsbBus;
use usb_device::bus::UsbBusAllocator;

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use embedded_time::rate::Hertz;
use hal::dbgprint;
use hal::{uart, uart_debug};

use hal::timer::SpinTimer;
use smart_leds::{hsv::RGB8, SmartLedsWrite};

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
    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let rstc = &peripherals.RSTC;

    let mut rgb = hal::dotstar_bitbang(pins.dotstar, &mut pins.port, SpinTimer::new(12));
    rgb.write([RGB8 { r: 0, g: 0, b: 0 }].iter().cloned())
        .unwrap();

    uart_debug::wire_uart(uart(
        pins.uart,
        &mut clocks,
        Hertz(115200),
        peripherals.SERCOM3,
        &mut peripherals.MCLK,
        &mut pins.port,
    ));
    dbgprint!(
        "\n\n\n\n~========== STARTING {:?} ==========~\n",
        hal::serial_number()
    );
    dbgprint!("Last reset was from {:?}\n", hal::reset_cause(rstc));

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(pins.usb.usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.MCLK,
            &mut pins.port,
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

    loop {
        let pending = disable_interrupts(|_| unsafe {
            let pending = PENDING_COLOR;
            PENDING_COLOR = None;
            pending
        });
        if let Some(color) = pending {
            rgb.write(color.iter().cloned()).unwrap();
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut PENDING_COLOR: Option<[RGB8; 1]> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        match c.clone() as char {
                            'R' => {
                                PENDING_COLOR = Some([RGB8 { r: 120, g: 0, b: 0 }]);
                            }
                            'G' => {
                                PENDING_COLOR = Some([RGB8 { r: 0, g: 120, b: 0 }]);
                            }
                            'O' => {
                                PENDING_COLOR = Some([RGB8 { r: 0, g: 0, b: 0 }]);
                            }
                            _ => {}
                        }
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
