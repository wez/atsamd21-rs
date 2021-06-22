#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::wifi_prelude::*;
use wio::wifi_rpcs as rpc;
use wio::wifi_types::Security;
use wio::{entry, wifi_singleton, Pins, Sets};

use core::fmt::Write;
use cortex_m::interrupt::free as disable_interrupts;

use eg::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::text::Text;

use heapless::{consts::U256, String};

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
    let mut sets: Sets = Pins::new(peripherals.PORT).split();

    // Set up the display so we can print out APs.
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            24.mhz(),
            &mut delay,
        )
        .unwrap();
    clear(&mut display);
    let mut textbuffer = String::<U256>::new();

    let mut user_led = sets.user_led.into_open_drain_output(&mut sets.port);
    user_led.set_low().unwrap();

    // Initialize the wifi peripheral.
    let args = (
        sets.wifi,
        peripherals.SERCOM0,
        &mut clocks,
        &mut peripherals.MCLK,
        &mut sets.port,
        &mut delay,
    );
    let nvic = &mut core.NVIC;
    disable_interrupts(|cs| unsafe {
        wifi_init(cs, args.0, args.1, args.2, args.3, args.4, args.5).unwrap();
        WIFI.as_mut().map(|wifi| {
            wifi.enable(cs, nvic);
        });
    });

    let version = unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::GetVersion {}).unwrap())
            .unwrap()
    };
    writeln!(textbuffer, "fw: {}", version).unwrap();
    write(
        &mut display,
        textbuffer.as_str(),
        Point::new(320 - (3 + version.len() * 12) as i32, 3),
    );
    textbuffer.truncate(0);

    let mac = unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::GetMacAddress {}).unwrap())
            .unwrap()
    };
    writeln!(textbuffer, "mac: {}", mac).unwrap();
    write(&mut display, textbuffer.as_str(), Point::new(3, 3));
    textbuffer.truncate(0);

    let ip_info = unsafe {
        WIFI.as_mut()
            .map(|wifi| {
                wifi.connect_to_ap(
                    &mut delay,
                    "NETWORK_NAME",
                    "PASSWORD_HERE",
                    Security::WPA2_SECURITY | Security::AES_ENABLED,
                )
                .unwrap()
            })
            .unwrap()
    };
    user_led.set_high().ok();
    writeln!(textbuffer, "ip = {}", ip_info.ip).unwrap();
    write(&mut display, textbuffer.as_str(), Point::new(3, 30));
    textbuffer.truncate(0);
    writeln!(textbuffer, "netmask = {}", ip_info.netmask).unwrap();
    write(&mut display, textbuffer.as_str(), Point::new(3, 42));
    textbuffer.truncate(0);
    writeln!(textbuffer, "gateway = {}", ip_info.gateway).unwrap();
    write(&mut display, textbuffer.as_str(), Point::new(3, 54));
    textbuffer.truncate(0);

    loop {
        user_led.toggle();
        delay.delay_ms(200u8);
    }
}

wifi_singleton!(WIFI);

fn clear(display: &mut wio::LCD) {
    display.clear(Rgb565::BLACK).ok().unwrap();
}

fn write<'a, T: Into<&'a str>>(display: &mut wio::LCD, text: T, pos: Point) {
    Text::new(
        text.into(),
        pos,
        MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
    )
    .draw(display)
    .ok()
    .unwrap();
}
