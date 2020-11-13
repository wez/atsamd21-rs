//! PyGamer pins

use super::{hal, pac, pac::MCLK, pac::QSPI, target_device};

use crate::hal::gpio::{self, *};
use gpio::{Floating, Input, Output, Port, PushPull};
use hal::define_pins;

use hal::prelude::*;

use hal::sercom::{
    I2CMaster2, PadPin, SPIMaster1, SPIMaster4, Sercom2Pad0, Sercom2Pad1, Sercom4Pad1, Sercom4Pad2,
    Sercom4Pad3, UART5,
};

use hal::qspi;

use embedded_hal::{digital::v1_compat::OldOutputPin, timer::CountDown, timer::Periodic};
use ws2812_timer_delay as ws2812;

use hal::clock::GenericClockController;

#[cfg(feature = "unproven")]
use hal::pwm::Pwm2;

use hal::time::Hertz;

use st7735_lcd::{Orientation, ST7735};

#[cfg(feature = "usb")]
use super::pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

#[cfg(feature = "unproven")]
use cortex_m::asm::delay as cycle_delay;

#[cfg(feature = "unproven")]
use super::pac::{ADC0, ADC1};

define_pins!(
    /// Maps the pins to their arduino names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// Analog pin 0.  Can act as a true analog output
    /// as it has a DAC (which is not currently supported
    /// by this hal) as well as input.
    pin speaker = a2,
    /// enable speaker amplifier
    pin speaker_enable = a27,

    /// Analog pin 1
    pin a1 = a5,
    /// Analog pin 2
    pin a2 = b8,
    /// Analog pin 3
    pin a3 = b9,
    /// Analog pin 4
    pin a4 = a4,
    /// Analog pin 5
    pin a5 = a6,
    /// Battery Measure (also Analog pin 6)
    pin battery = b1,
    /// Light sensor (also Analog pin 7)
    pin light = b4,
    /// Digital pin 2 (also Analog pin 8)
    pin d2 = b3,
    /// Digital pin 3 (also Analog pin 9)
    pin d3 = b2,
    /// Digital pin 5
    pin d5 = a16,
    /// Digital pin 6
    pin d6 = a18,
    /// Accelerometer interrupt pin (also d7)
    pin accel_irq = b14,
    /// Neopixel data line (controls all 5 neopixels, also d8)
    pin neopixel = a15,
    /// Digital pin 9
    pin d9 = a19,
    /// Digital pin 10
    pin d10 = a20,
    /// Digital pin 11
    pin d11 = a21,
    /// Digital pin 12
    pin d12 = a22,
    /// D13 LED/JACDAC
    pin d13 = a23,

    // TFT(Thin-film-transistor liquid-crystal display) control pins
    /// TFT MOSI
    pin tft_mosi = b15,
    /// TFT SCK
    pin tft_sck = b13,
    /// TFT Reset
    pin tft_reset = a0,
    /// TFT DC
    pin tft_dc = b5,
    /// TFT CS
    pin tft_cs = b12,
    /// TFT Backlight (also Analog pin 7)
    pin tft_backlight = a1,

    // UART - Universal Asynchronous Receiver/Transmitter
    /// Pin TX (d1)
    pin tx = b16,
    /// Pin RX (d0)
    pin rx = b17,

    // SPI - Serial Peripheral Interface (connected to sd card slot)
    /// Pin MISO
    pin miso = b22,
    /// Pin MOSI
    pin mosi = b23,
    /// Pin SCK
    pin sck = a17,

    // I2C (connected to LIS3DH accelerometer)
    /// STEMMA SDA
    pin sda = a12,
    /// STEMMA SCL
    pin scl = a13,

    /// USB D- pin
    pin usb_dm = a24,
    /// USB D+ pin
    pin usb_dp = a25,

    /// SD card chip select (also d4)
    pin sd_cs = a14,

    /// Joystick X
    pin joy_x = b7,
    /// Joystick Y
    pin joy_y = b6,

    /// Button Latch
    pin button_latch = b0,
    /// Button Out
    pin button_out = b30,
    /// Button Clock
    pin button_clock = b31,

    /// qspi flash
    pin flash_sck = b10,
    pin flash_cs = b11,
    pin flash_d0 = a8,
    pin flash_d1 = a9,
    pin flash_d2 = a10,
    pin flash_d3 = a11,
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let speaker = Speaker {
            speaker: self.speaker,
            enable: self.speaker_enable,
        };

        let display = Display {
            accel_irq: self.accel_irq,
            tft_mosi: self.tft_mosi,
            tft_sck: self.tft_sck,
            tft_reset: self.tft_reset,
            tft_cs: self.tft_cs,
            tft_dc: self.tft_dc,
            tft_backlight: self.tft_backlight,
        };

        let analog = Analog {
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
            a4: self.a4,
            a5: self.a5,
        };

        let digital = Digital {
            d2: self.d2,
            d3: self.d3,
            d5: self.d5,
            d6: self.d6,
            d9: self.d9,
            d10: self.d10,
            d11: self.d11,
            d12: self.d12,
        };

        let flash = QSPIFlash {
            sck: self.flash_sck,
            cs: self.flash_cs,
            data0: self.flash_d0,
            data1: self.flash_d1,
            data2: self.flash_d2,
            data3: self.flash_d3,
        };

        let spi = SPI {
            sck: self.sck,
            mosi: self.mosi,
            miso: self.miso,
        };

        let i2c = I2C {
            sda: self.sda,
            scl: self.scl,
        };

        let neopixel = Neopixel {
            neopixel: self.neopixel,
        };

        let battery = Battery {
            battery: self.battery,
        };

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let uart = UART {
            rx: self.rx,
            tx: self.tx,
        };

        let buttons = Buttons {
            latch: self.button_latch,
            data_in: self.button_out,
            clock: self.button_clock,
        };

        let joystick = Joystick {
            joy_x: self.joy_x,
            joy_y: self.joy_y,
        };

        Sets {
            port: self.port,
            display,
            led_pin: self.d13,
            neopixel,
            battery,
            light_pin: self.light,
            i2c,
            sd_cs_pin: self.sd_cs,
            analog,
            digital,
            speaker,
            spi,
            flash,
            usb,
            uart,
            buttons,
            joystick,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    /// Port
    pub port: Port,

    /// LCD Display
    pub display: Display,

    /// Red Led
    pub led_pin: Pa23<Input<Floating>>,

    /// Neopixel (RGB LED) pins
    pub neopixel: Neopixel,

    /// Analog Light Sensor
    pub light_pin: Pb4<Input<Floating>>,

    /// I2C (connected to LIS3DH accelerometer and "Stemma" port)
    pub i2c: I2C,

    /// SD Card CS pin
    pub sd_cs_pin: Pa14<Input<Floating>>,

    /// Battery Voltage
    pub battery: Battery,

    /// Speaker (DAC not implemented in hal yet)
    pub speaker: Speaker,

    /// SPI (connected to SD Card)
    pub spi: SPI,

    /// USB pins
    pub usb: USB,

    /// UART (external pinout) pins
    pub uart: UART,

    /// Analog pins.
    pub analog: Analog,

    /// Digital pins.
    pub digital: Digital,

    /// Flash storage
    pub flash: QSPIFlash,

    pub buttons: Buttons,

    pub joystick: Joystick,
}

/// Display pins
pub struct Display {
    pub accel_irq: Pb14<Input<Floating>>, // TODO remove once we make miso optional
    pub tft_mosi: Pb15<Input<Floating>>,
    pub tft_sck: Pb13<Input<Floating>>,
    pub tft_reset: Pa0<Input<Floating>>,
    pub tft_cs: Pb12<Input<Floating>>,
    pub tft_dc: Pb5<Input<Floating>>,
    pub tft_backlight: Pa1<Input<Floating>>,
}

#[cfg(feature = "unproven")]
impl Display {
    /// Convenience for setting up the on board display.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        sercom4: pac::SERCOM4,
        mclk: &mut pac::MCLK,
        timer2: pac::TC2,
        delay: &mut hal::delay::Delay,
        port: &mut Port,
    ) -> Result<
        (
            ST7735<
                SPIMaster4<
                    Sercom4Pad2<gpio::Pb14<gpio::PfC>>,
                    Sercom4Pad3<gpio::Pb15<gpio::PfC>>,
                    Sercom4Pad1<gpio::Pb13<gpio::PfC>>,
                >,
                gpio::Pb5<Output<PushPull>>,
                gpio::Pa0<Output<PushPull>>,
            >,
            atsamd_hal::samd51::pwm::Pwm2,
        ),
        (),
    > {
        let gclk0 = clocks.gclk0();
        let tft_spi = SPIMaster4::new(
            &clocks.sercom4_core(&gclk0).ok_or(())?,
            16_000_000.Hz(),
            hal::hal::spi::Mode {
                phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
                polarity: hal::hal::spi::Polarity::IdleLow,
            },
            sercom4,
            mclk,
            (
                self.accel_irq.into_pad(port),
                self.tft_mosi.into_pad(port),
                self.tft_sck.into_pad(port),
            ),
        );

        let mut tft_cs = self.tft_cs.into_push_pull_output(port);
        tft_cs.set_low()?;

        let tft_dc = self.tft_dc.into_push_pull_output(port);
        let tft_reset = self.tft_reset.into_push_pull_output(port);

        let mut display =
            st7735_lcd::ST7735::new(tft_spi, tft_dc, tft_reset, true, false, 160, 128);
        display.init(delay)?;
        display.set_orientation(&Orientation::LandscapeSwapped)?;

        let tft_backlight = self.tft_backlight.into_function_e(port);
        let mut pwm2 = Pwm2::new(
            &clocks.tc2_tc3(&gclk0).ok_or(())?,
            1000.Hz(),
            timer2,
            hal::pwm::TC2Pinout::Pa1(tft_backlight),
            mclk,
        );

        pwm2.set_duty(pwm2.get_max_duty());

        Ok((display, pwm2))
    }
}

/// Neopixel pins
pub struct Neopixel {
    pub neopixel: Pa15<Input<Floating>>,
}

impl Neopixel {
    /// Convenience for setting up the onboard neopixels using the provided
    /// Timer preconfigured to 3mhz.
    pub fn init<T: CountDown + Periodic>(
        self,
        timer: T,
        port: &mut Port,
    ) -> ws2812::Ws2812<
        T,
        embedded_hal::digital::v1_compat::OldOutputPin<
            atsamd_hal::common::gpio::Pa15<
                atsamd_hal::common::gpio::Output<atsamd_hal::common::gpio::PushPull>,
            >,
        >,
    > {
        let neopixel_pin: OldOutputPin<_> = self.neopixel.into_push_pull_output(port).into();

        ws2812::Ws2812::new(timer, neopixel_pin)
    }
}

/// SPI pins
pub struct SPI {
    pub mosi: gpio::Pb23<Input<Floating>>,
    pub miso: gpio::Pb22<Input<Floating>>,
    pub sck: gpio::Pa17<Input<Floating>>,
}

impl SPI {
    /// Convenience for setting up the labelled pins to operate
    /// as an SPI master, running at the specified frequency.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom1: pac::SERCOM1,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> SPIMaster1<
        hal::sercom::Sercom1Pad2<gpio::Pb22<gpio::PfC>>,
        hal::sercom::Sercom1Pad3<gpio::Pb23<gpio::PfC>>,
        hal::sercom::Sercom1Pad1<gpio::Pa17<gpio::PfC>>,
    > {
        let gclk0 = clocks.gclk0();
        SPIMaster1::new(
            &clocks.sercom1_core(&gclk0).unwrap(),
            bus_speed.into(),
            hal::hal::spi::Mode {
                phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
                polarity: hal::hal::spi::Polarity::IdleLow,
            },
            sercom1,
            mclk,
            (
                self.miso.into_pad(port),
                self.mosi.into_pad(port),
                self.sck.into_pad(port),
            ),
        )
    }
}

/// I2C pins
pub struct I2C {
    pub sda: Pa12<Input<Floating>>,
    pub scl: Pa13<Input<Floating>>,
}

impl I2C {
    /// Convenience for setting up the labelled SDA, SCL pins to
    /// operate as an I2C master running at the specified frequency.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom2: pac::SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> I2CMaster2<Sercom2Pad0<Pa12<PfC>>, Sercom2Pad1<Pa13<PfC>>> {
        let gclk0 = clocks.gclk0();
        I2CMaster2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom2,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// Sd Card pins
pub struct SdCard {
    pub cs: Pa14<Input<Floating>>,
}

/// Speaker pins
pub struct Speaker {
    pub speaker: Pa2<Input<Floating>>,
    pub enable: Pa27<Input<Floating>>,
}

/// USB pins
pub struct USB {
    pub dm: Pa24<Input<Floating>>,
    pub dp: Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    /// Convenience for setting up the onboard usb port to operate
    /// as a USB device.
    pub fn init(
        self,
        usb: super::pac::USB,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UsbBusAllocator<UsbBus> {
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
        let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();

        UsbBusAllocator::new(UsbBus::new(
            usb_clock,
            mclk,
            self.dm.into_function(port),
            self.dp.into_function(port),
            usb,
        ))
    }
}

/// UART pins
pub struct UART {
    pub tx: Pb16<Input<Floating>>,
    pub rx: Pb17<Input<Floating>>,
}

impl UART {
    /// Convenience for setting up the labelled TX, RX pins
    /// to operate as a UART device at the specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom5: pac::SERCOM5,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART5<
        hal::sercom::Sercom5Pad1<gpio::Pb17<gpio::PfC>>,
        hal::sercom::Sercom5Pad0<gpio::Pb16<gpio::PfC>>,
        (),
        (),
    > {
        let gclk0 = clocks.gclk0();

        UART5::new(
            &clocks.sercom5_core(&gclk0).unwrap(),
            baud.into(),
            sercom5,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

/// Analog pins
pub struct Analog {
    pub a1: Pa5<Input<Floating>>,
    pub a2: Pb8<Input<Floating>>,
    pub a3: Pb9<Input<Floating>>,
    pub a4: Pa4<Input<Floating>>,
    pub a5: Pa6<Input<Floating>>,
}

/// Digital pins
pub struct Digital {
    /// also usabe as A8
    pub d2: Pb3<Input<Floating>>,
    /// also usabe as A9
    pub d3: Pb2<Input<Floating>>,
    pub d5: Pa16<Input<Floating>>,
    pub d6: Pa18<Input<Floating>>,
    pub d9: Pa19<Input<Floating>>,
    pub d10: Pa20<Input<Floating>>,
    pub d11: Pa21<Input<Floating>>,
    pub d12: Pa22<Input<Floating>>,
}

/// QSPI flash pins
pub struct QSPIFlash {
    pub sck: Pb10<Input<Floating>>,
    pub cs: Pb11<Input<Floating>>,
    pub data0: Pa8<Input<Floating>>,
    pub data1: Pa9<Input<Floating>>,
    pub data2: Pa10<Input<Floating>>,
    pub data3: Pa11<Input<Floating>>,
}

impl QSPIFlash {
    pub fn init(self, mclk: &mut MCLK, port: &mut Port, qspi: QSPI) -> qspi::Qspi<qspi::OneShot> {
        qspi::Qspi::new(
            mclk, port, qspi, self.sck, self.cs, self.data0, self.data1, self.data2, self.data3,
        )
    }
}

/// Button pins
pub struct Buttons {
    /// Button Latch
    pub latch: Pb0<Input<Floating>>,
    /// Button Out
    pub data_in: Pb30<Input<Floating>>,
    /// Button Clock
    pub clock: Pb31<Input<Floating>>,
}

#[cfg(feature = "unproven")]
#[derive(Debug, PartialEq)]
pub enum Keys {
    SelectDown,
    SelectUp,
    StartDown,
    StartUp,
    BDown,
    BUp,
    ADown,
    AUp,
}

#[cfg(feature = "unproven")]
pub struct ButtonIter {
    pub pressed: u8,
    pub released: u8,
    pub bit_index: u8,
}

#[cfg(feature = "unproven")]
//should be impossible for released and pressed, but gives released preference
fn mask_to_event(mask: u8, released: u8, pressed: u8) -> Option<Keys> {
    let pressed_bool = mask & pressed == mask;
    let released_bool = mask & released == mask;

    match mask {
        0x8 => {
            if released_bool {
                Some(Keys::BUp)
            } else if pressed_bool {
                Some(Keys::BDown)
            } else {
                None
            }
        }
        0x4 => {
            if released_bool {
                Some(Keys::AUp)
            } else if pressed_bool {
                Some(Keys::ADown)
            } else {
                None
            }
        }
        0x2 => {
            if released_bool {
                Some(Keys::StartUp)
            } else if pressed_bool {
                Some(Keys::StartDown)
            } else {
                None
            }
        }
        0x1 => {
            if released_bool {
                Some(Keys::SelectUp)
            } else if pressed_bool {
                Some(Keys::SelectDown)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(feature = "unproven")]
impl Iterator for ButtonIter {
    type Item = Keys;

    fn next(&mut self) -> Option<Keys> {
        //really want a while post increment but doesnt exist
        //only 4 buttons represented in the shift
        if self.bit_index >= 4 {
            return None;
        }

        //funky do while
        while {
            let mask = 0x01 << self.bit_index;
            self.bit_index += 1;

            let event = mask_to_event(mask, self.released, self.pressed);
            if event.is_some() {
                return event;
            }

            self.bit_index < 4
        } {}

        None
    }
}

/// Button pins
#[cfg(feature = "unproven")]
pub struct ButtonReader {
    /// Button Latch
    pub latch: Pb0<Output<PushPull>>,
    /// Button Out
    pub data_in: Pb30<Input<Floating>>,
    /// Button Clock
    pub clock: Pb31<Output<PushPull>>,
    pub last: u8,
}

//120mhz, 1 cycle = 0.000000008333333 = 8.333333ns
//https://www.onsemi.com/pub/Collateral/MC74HC165A-D.PDF
//3v <=125c
//tsu min setup time 55ns = 7 cycles
//th min hold time 5ns = 1 cycles
//tw min pulse width 36ns = 5 cycles
//trec min recovery time 55ns, how long before you should attempt to read
// again?
#[cfg(feature = "unproven")]
impl ButtonReader {
    // 28*8.333ns total blocking read
    /// Returns a ButtonIter of button changes as Keys enums
    pub fn events(&mut self) -> ButtonIter {
        self.latch.set_low().ok();
        cycle_delay(7); //tsu?
        self.latch.set_high().ok();
        cycle_delay(1); //th?

        let mut current: u8 = 0;

        // they only use the top 4 bits
        for _i in 0..4 {
            current <<= 1;

            self.clock.set_low().ok();
            cycle_delay(5); //tw

            if self.data_in.is_high().unwrap() {
                current |= 1;
            }
            self.clock.set_high().ok();
        }

        let iter = ButtonIter {
            pressed: (self.last ^ current) & current,
            released: (self.last ^ current) & self.last,
            bit_index: 0,
        };

        self.last = current;

        iter
    }
}

#[cfg(feature = "unproven")]
impl Buttons {
    /// Convenience for setting up the button latch pins
    /// Returns ButtonReader iterator which can be polled for Key events
    pub fn init(self, port: &mut Port) -> ButtonReader {
        let mut latch = self.latch.into_push_pull_output(port);
        latch.set_high().ok();

        let data_in = self.data_in.into_floating_input(port);

        let mut clock = self.clock.into_push_pull_output(port);
        clock.set_high().ok();

        ButtonReader {
            latch,
            data_in,
            clock,
            last: 0,
        }
    }
}

/// Joystick pins
pub struct JoystickReader {
    /// Joystick X
    pub joy_x: gpio::Pb7<gpio::PfB>,
    /// Joystick Y
    pub joy_y: gpio::Pb6<gpio::PfB>,
}

#[cfg(feature = "unproven")]
impl JoystickReader {
    /// returns a tuple (x,y) where values are 12 bit, between 0-4095
    /// values are NOT centered, but could be by subtracting 2048
    pub fn read(&mut self, adc: &mut hal::adc::Adc<ADC1>) -> (u16, u16) {
        //note adafruit averages 3 readings on x and y (not inside the adc) seems
        // unnecessary? note adafruit recenters around zero.. Im not doing that
        // either atm.

        let y_data: u16 = adc.read(&mut self.joy_y).unwrap();
        let x_data: u16 = adc.read(&mut self.joy_x).unwrap();

        (x_data, y_data)
    }
}

/// Joystick pins
pub struct Joystick {
    /// Joystick X
    pub joy_x: Pb7<Input<Floating>>,
    /// Joystick Y
    pub joy_y: Pb6<Input<Floating>>,
}

#[cfg(feature = "unproven")]
impl Joystick {
    /// Convenience for setting up the joystick. Returns JoystickReader instance
    /// which can be polled for joystick (x,y) tuple
    pub fn init(self, port: &mut Port) -> JoystickReader {
        JoystickReader {
            joy_x: self.joy_x.into_function_b(port),
            joy_y: self.joy_y.into_function_b(port),
        }
    }
}

/// Battery Reader
#[cfg(feature = "unproven")]
pub struct BatteryReader {
    /// Battery pin
    pub battery: gpio::Pb1<gpio::PfB>,
}

#[cfg(feature = "unproven")]
impl BatteryReader {
    /// Returns a float for voltage of battery
    pub fn read(&mut self, adc: &mut hal::adc::Adc<ADC0>) -> f32 {
        let data: u16 = adc.read(&mut self.battery).unwrap();
        let result: f32 = (data as f32 / 4095.0) * 2.0 * 3.3;
        result
    }
}

/// Battery pin
pub struct Battery {
    pub battery: Pb1<Input<Floating>>,
}

#[cfg(feature = "unproven")]
impl Battery {
    /// Convenience for reading Battery Volage. Returns BatteryReader instance
    /// which can be polled for battery voltage
    pub fn init(self, port: &mut Port) -> BatteryReader {
        BatteryReader {
            battery: self.battery.into_function_b(port),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_from_left() {
        let mut iter = ButtonIter {
            pressed: 0x0F,
            released: 0x00,
            bit_index: 0,
        };

        assert_eq!(Some(Keys::BDown), iter.next());
        assert_eq!(Some(Keys::ADown), iter.next());
        assert_eq!(Some(Keys::StartDown), iter.next());
        assert_eq!(Some(Keys::SelectDown), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_released_before_pressed() {
        let mut iter = ButtonIter {
            pressed: 0x0F,
            released: 0x0F,
            bit_index: 0,
        };

        assert_eq!(Some(Keys::BUp), iter.next());
        assert_eq!(Some(Keys::AUp), iter.next());
        assert_eq!(Some(Keys::StartUp), iter.next());
        assert_eq!(Some(Keys::SelectUp), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_released_before_pressed2() {
        let mut iter = ButtonIter {
            pressed: 0x05,
            released: 0x0A,
            bit_index: 0,
        };

        assert_eq!(Some(Keys::BUp), iter.next());
        assert_eq!(Some(Keys::ADown), iter.next());
        assert_eq!(Some(Keys::StartUp), iter.next());
        assert_eq!(Some(Keys::SelectDown), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn inner_empty_bit_is_skipped() {
        let mut iter = ButtonIter {
            pressed: 0x05,
            released: 0x00,
            bit_index: 0,
        };

        assert_eq!(Some(Keys::ADown), iter.next());
        assert_eq!(Some(Keys::SelectDown), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn empty_returns_none() {
        let mut iter = ButtonIter {
            pressed: 0x00,
            released: 0x00,
            bit_index: 0,
        };

        assert_eq!(None, iter.next());
    }

    #[test]
    fn upper_four_bits_unused() {
        let mut iter = ButtonIter {
            pressed: 0xF0,
            released: 0x00,
            bit_index: 0,
        };

        assert_eq!(None, iter.next());
    }

    #[test]
    fn upper_four_bits_unused2() {
        let mut iter = ButtonIter {
            pressed: 0x00,
            released: 0xF0,
            bit_index: 0,
        };

        assert_eq!(None, iter.next());
    }
}
