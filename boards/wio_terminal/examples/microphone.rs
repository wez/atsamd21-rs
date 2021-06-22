#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use cortex_m::peripheral::NVIC;
use wio::hal::adc::{FreeRunning, InterruptAdc};
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{interrupt, ADC1};
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

use core::fmt::Write;
use eg::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use eg::pixelcolor::Rgb565;
use eg::prelude::*;
use eg::primitives::{PrimitiveStyleBuilder, Rectangle};
use eg::text::Text;
use heapless::consts::*;
use heapless::String;

use heapless::spsc::Queue;
struct Ctx {
    adc: InterruptAdc<ADC1, ConversionMode>,
    samples: Queue<u16, U8>,
}
static mut CTX: Option<Ctx> = None;

type ConversionMode = FreeRunning;
// You also have to uncomment the line which calls start_conversion function in
// the main loop. type ConversionMode = SingleConversion;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut sets: Sets = Pins::new(peripherals.PORT).split();

    // Set up the display so we can log our progress.
    let (display, _backlight) = sets
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
    let mut terminal = Terminal::new(display);
    let mut textbuffer = String::<U256>::new();

    let mut user_led = sets.user_led.into_open_drain_output(&mut sets.port);
    user_led.set_high().unwrap();

    // Construct an InterruptAdc with free-running mode.
    let (mut microphone_adc, mut microphone_pin) = {
        let (adc, pin) = sets.microphone.init(
            peripherals.ADC1,
            &mut clocks,
            &mut peripherals.MCLK,
            &mut sets.port,
        );
        let interrupt_adc: InterruptAdc<_, ConversionMode> = InterruptAdc::from(adc);
        (interrupt_adc, pin)
    };

    microphone_adc.start_conversion(&mut microphone_pin);

    unsafe {
        CTX = Some(Ctx {
            adc: microphone_adc,
            samples: Queue::new(),
        });
    }
    let mut consumer = unsafe { CTX.as_mut().unwrap().samples.split().1 };

    terminal.write_str("min,max,avg\n");

    unsafe {
        // Enable ADC1 result ready interrupt.
        NVIC::unmask(interrupt::ADC1_RESRDY);
    }
    user_led.set_low().unwrap();

    loop {
        let mut min = core::f32::INFINITY;
        let mut max = core::f32::NEG_INFINITY;
        let mut sum = 0f32;
        // Though the ADC sampling rate is set to 250[kSPS] according to the comment in
        // the adc.rs, actual sampling rate seems 83.333[kSPS], which is 1/3 of
        // expected sampling rate.
        let count_max = 83333;
        for _count in 0..count_max {
            // Uncomment if you use single conversion mode.
            // unsafe { CTX.as_mut().unwrap().adc.start_conversion(&mut microphone_pin); }
            let value = loop {
                if let Some(value) = consumer.dequeue() {
                    break value as f32;
                }
            };
            if value < min {
                min = value;
            }
            if max < value {
                max = value
            }
            sum += value;
        }
        textbuffer.clear();
        writeln!(textbuffer, "{},{},{}", min, max, sum / count_max as f32).unwrap();
        terminal.write_str(textbuffer.as_str());
    }
}

#[interrupt]
fn ADC1_RESRDY() {
    unsafe {
        let ctx = CTX.as_mut().unwrap();
        let mut producer = ctx.samples.split().0;
        if let Some(sample) = ctx.adc.service_interrupt_ready() {
            producer.enqueue_unchecked(sample);
        }
    }
}

/// Handly helper for logging text to the screen.
struct Terminal<'a> {
    text_style: MonoTextStyle<'a, Rgb565>,
    cursor: Point,
    display: wio::LCD,
}

impl<'a> Terminal<'a> {
    pub fn new(mut display: wio::LCD) -> Self {
        // Clear the screen.
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let backdrop =
            Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
        backdrop.draw(&mut display).ok().unwrap();

        Self {
            text_style: MonoTextStyle::new(&FONT_6X12, Rgb565::WHITE),
            cursor: Point::new(0, 0),
            display,
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
            // Clear the screen.
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK)
                .build();
            let backdrop =
                Rectangle::with_corners(Point::new(0, 0), Point::new(320, 320)).into_styled(style);
            backdrop.draw(&mut self.display).ok().unwrap();
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
}
