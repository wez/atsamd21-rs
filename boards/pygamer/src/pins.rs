//! PyGamer pins

use embedded_hal::{digital::v1_compat::OldOutputPin, timer::CountDown, timer::Periodic};

use super::{hal, pac};

use pac::{MCLK, QSPI};

use hal::prelude::*;

use hal::clock::GenericClockController;
use hal::gpio::v2::PA01;
use hal::pwm;
use hal::qspi;
use hal::sercom::v2::{spi, IoSet1, Sercom1, Sercom4, UndocIoSet1};
use hal::sercom::{I2CMaster2, UART5};
use hal::time::Hertz;
use hal::typelevel::NoneT;

use st7735_lcd::{Orientation, ST7735};
use ws2812_timer_delay as ws2812;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;
#[cfg(feature = "usb")]
use pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

#[cfg(feature = "unproven")]
pub use crate::buttons::ButtonReader;
#[cfg(feature = "unproven")]
pub use crate::buttons::Keys;
#[cfg(feature = "unproven")]
use hal::pwm::Pwm2;
#[cfg(feature = "unproven")]
use pac::{ADC0, ADC1};

/// Pin constants and type aliases
pub use aliases::*;

pub mod aliases {
    use super::hal::bsp_pins;

    bsp_pins!(

        /// Analog pin 0. Can act as a true analog output
        /// as it has a DAC (which is not currently supported
        /// by this hal) as well as input.
        PA02 {
            name: speaker
            aliases: {
                Reset: SpeakerReset
            }
        },
        /// enable speaker amplifier
        PA27 {
            name: speaker_enable
            aliases: {
                Reset: SpeakerEnableReset
            }
        },

        /// Analog pin 1
        PA05 {
            name: a1
            aliases: {
                Reset: A1Reset
            }
        },
        /// Analog pin 2
        PB08 {
            name: a2
            aliases: {
                Reset: A2Reset
            }
        },
        /// Analog pin 3
        PB09 {
            name: a3
            aliases: {
                Reset: A3Reset
            }
        },
        /// Analog pin 4
        PA04 {
            name: a4
            aliases: {
                Reset: A4Reset
            }
        },
        /// Analog pin 5
        PA06 {
            name: a5
            aliases: {
                Reset: A5Reset
            }
        },
        /// Battery Measure (also Analog pin 6)
        PB01 {
            name: battery
            aliases: {
                AlternateB: BatteryPin,
                Reset: BatteryReset
            }
        },
        /// Light sensor (also Analog pin 7)
        PB04 {
            name: light
            aliases: {
                Reset: LightReset
            }
        },
        /// Digital pin 2 (also Analog pin 8)
        PB03 {
            name: d2
            aliases: {
                Reset: D2Reset
            }
        },
        /// Digital pin 3 (also Analog pin 9)
        PB02 {
            name: d3
            aliases: {
                Reset: D3Reset
            }
        },
        /// Digital pin 5
        PA16 {
            name: d5
            aliases: {
                Reset: D5Reset
            }
        },
        /// Digital pin 6
        PA18 {
            name: d6
            aliases: {
                Reset: D6Reset
            }
        },
        /// Accelerometer interrupt pin (also d7)
        PB14 {
            name: accel_irq
            aliases: {
                Reset: AccelIrqReset
            }
        },
        PA15 {
            /// Neopixel data line (controls all 5 neopixels, also d8)
            name: neopixel
            aliases: {
                PushPullOutput: NeopixelPin,
                Reset: NeopixelReset
            }
        },
        /// Digital pin 9
        PA19 {
            name: d9
            aliases: {
                Reset: D9Reset
            }
        },
        /// Digital pin 10
        PA20 {
            name: d10
            aliases: {
                Reset: D10Reset
            }
        },
        /// Digital pin 11
        PA21 {
            name: d11
            aliases: {
                Reset: D11Reset
            }
        },
        /// Digital pin 12
        PA22 {
            name: d12
            aliases: {
                Reset: D12Reset
            }
        },
        /// D13 LED/JACDAC
        PA23 {
            name: d13
            aliases: {
                PushPullOutput: RedLed,
                Reset: D13Reset
            }
        },

        // TFT(Thin-film-transistor liquid-crystal display) control pins
        PB15 {
            /// TFT MOSI
            name: tft_mosi,
            aliases: {
                AlternateC: TftMosi,
                Reset: TftMosiReset
            }
        },
        PB13 {
            /// TFT SCLK
            name: tft_sclk
            aliases: {
                AlternateC: TftSclk,
                Reset: TftSclkReset
            }
        },
        PA00 {
            /// TFT Reset
            name: tft_reset
            aliases: {
                PushPullOutput: TftReset,
                Reset: TftResetReset
            }
        },
        PB05 {
            /// TFT DC
            name: tft_dc
            aliases: {
                PushPullOutput: TftDc,
                Reset: TftDcReset
            }
        },
        PB12 {
            /// TFT CS
            name: tft_cs
            aliases: {
                PushPullOutput: TftCs,
                Reset: TftCsReset
            }
        },
        PA01 {
            /// TFT Backlight (also Analog pin 7)
            name: tft_backlight
            aliases: {
                AlternateE: TftBacklight,
                Reset: TftBacklightReset
            }
        },

        // UART - Universal Asynchronous Receiver/Transmitter
        PB16 {
            /// Pin TX (d1)
            name: tx
            aliases: {
                AlternateC: UartTx,
                Reset: UartTxReset
            }
        },
        PB17 {
            /// Pin RX (d0)
            name: rx
            aliases: {
                AlternateC: UartRx,
                Reset: UartRxReset
            }
        },

        // SPI - Serial Peripheral Interface (connected to sd card slot)
        PB22 {
            /// Pin MISO
            name: miso
            aliases: {
                AlternateC: SpiMiso,
                Reset: SpiMisoReset
            }
        },
        PB23 {
            /// Pin MOSI
            name: mosi
            aliases: {
                AlternateC: SpiMosi,
                Reset: SpiMosiReset
            }
        },
        PA17 {
            /// Pin SCK
            name: sclk
            aliases: {
                AlternateC: SpiSclk,
                Reset: SpiSclkReset
            }
        },

        // I2C (connected to LIS3DH accelerometer)
        PA12 {
            /// STEMMA SDA
            name: sda
            aliases: {
                AlternateC: Sda,
                Reset: SdaReset
            }
        },
        PA13 {
            /// STEMMA SCL
            name: scl
            aliases: {
                AlternateC: Scl,
                Reset: SclReset
            }
        },

        PA24 {
            /// USB D- pin
            name: usb_dm
            aliases: {
                AlternateH: UsbDm,
                Reset: UsbDmReset
            }
        },
        PA25 {
            /// USB D+ pin
            name: usb_dp
            aliases: {
                AlternateH: UsbDp,
                Reset: UsbDpReset
            }
        },

        /// SD card chip select (also d4)
        PA14 {
            name: sd_cs
            aliases: {
                Reset: SdCsReset
            }
        },

        PB07 {
            /// Joystick X
            name: joy_x
            aliases: {
                AlternateB: JoyX,
                Reset: JoyXReset
            }
        },
        PB06 {
            /// Joystick Y
            name: joy_y
            aliases: {
                AlternateB: JoyY,
                Reset: JoyYReset
            }
        },

        PB00 {
            /// Button Latch
            name: button_latch
            aliases: {
                PushPullOutput: ButtonLatch,
                Reset: ButtonLatchReset
            }
        },
        PB30 {
            /// Button Out
            name: button_out
            aliases: {
                FloatingInput: ButtonOut,
                Reset: ButtonOutReset
            }
        },
        PB31 {
            /// Button Clock
            name: button_clock
            aliases: {
                PushPullOutput: ButtonClock,
                Reset: ButtonClockReset
            }
        },

        // qspi flash
        PB10 {
            name: flash_sclk
            aliases: {
                AlternateH: QspiSclk,
                Reset: QspiSclkReset
            }
        },
        PB11 {
            name: flash_cs
            aliases: {
                AlternateH: QspiCs,
                Reset: QspiCsReset
            }
        },
        PA08 {
            name: flash_d0
            aliases: {
                AlternateH: QspiD0,
                Reset: QspiD0Reset
            }
        },
        PA09 {
            name: flash_d1
            aliases: {
                AlternateH: QspiD1,
                Reset: QspiD1Reset
            }
        },
        PA10 {
            name: flash_d2
            aliases: {
                AlternateH: QspiD2,
                Reset: QspiD2Reset
            }
        },
        PA11 {
            name: flash_d3
            aliases: {
                AlternateH: QspiD3,
                Reset: QspiD3Reset
            }
        },
    );
}

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let speaker = Speaker {
            speaker: self.speaker,
            enable: self.speaker_enable,
        };

        let display = Display {
            tft_mosi: self.tft_mosi,
            tft_sclk: self.tft_sclk,
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
            sclk: self.flash_sclk,
            cs: self.flash_cs,
            data0: self.flash_d0,
            data1: self.flash_d1,
            data2: self.flash_d2,
            data3: self.flash_d3,
        };

        let spi = SPI {
            sclk: self.sclk,
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
    /// LCD Display
    pub display: Display,

    /// Red Led
    pub led_pin: D13Reset,

    /// Neopixel (RGB LED) pins
    pub neopixel: Neopixel,

    /// Analog Light Sensor
    pub light_pin: LightReset,

    /// I2C (connected to LIS3DH accelerometer and "Stemma" port)
    pub i2c: I2C,

    /// SD Card CS pin
    pub sd_cs_pin: SdCsReset,

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
    pub tft_mosi: TftMosiReset,
    pub tft_sclk: TftSclkReset,
    pub tft_reset: TftResetReset,
    pub tft_cs: TftCsReset,
    pub tft_dc: TftDcReset,
    pub tft_backlight: TftBacklightReset,
}

pub type TftPads = spi::Pads<Sercom4, IoSet1, NoneT, TftMosi, TftSclk>;
pub type TftSpi = spi::Spi<spi::Config<TftPads>, spi::Tx>;

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
    ) -> Result<(ST7735<TftSpi, TftDc, TftReset>, Pwm2<PA01>), ()> {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom4_core(&gclk0).ok_or(())?;
        let pads = spi::Pads::default()
            .sclk(self.tft_sclk)
            .data_out(self.tft_mosi);
        let tft_spi = spi::Config::new(mclk, sercom4, pads, clock.freq())
            .spi_mode(spi::MODE_0)
            .baud(16.mhz())
            .enable();
        let mut tft_cs: TftCs = self.tft_cs.into();
        tft_cs.set_low().ok();
        let mut display = st7735_lcd::ST7735::new(
            tft_spi,
            self.tft_dc.into(),
            self.tft_reset.into(),
            true,
            false,
            160,
            128,
        );
        display.init(delay)?;
        display.set_orientation(&Orientation::LandscapeSwapped)?;
        let pwm_clock = &clocks.tc2_tc3(&gclk0).ok_or(())?;
        let pwm_pinout = pwm::TC2Pinout::Pa1(self.tft_backlight);
        let mut pwm2 = Pwm2::new(pwm_clock, 1.khz(), timer2, pwm_pinout, mclk);
        pwm2.set_duty(pwm2.get_max_duty());
        Ok((display, pwm2))
    }
}

/// Neopixel pins
pub struct Neopixel {
    pub neopixel: NeopixelReset,
}

impl Neopixel {
    /// Convenience for setting up the onboard neopixels using the provided
    /// Timer preconfigured to 3mhz.
    pub fn init<T>(self, timer: T) -> ws2812::Ws2812<T, OldOutputPin<NeopixelPin>>
    where
        T: CountDown + Periodic,
    {
        let neopixel_pin: NeopixelPin = self.neopixel.into();
        let neopixel_pin: OldOutputPin<_> = neopixel_pin.into();
        ws2812::Ws2812::new(timer, neopixel_pin)
    }
}

/// SPI pins
pub struct SPI {
    pub mosi: SpiMosiReset,
    pub miso: SpiMisoReset,
    pub sclk: SpiSclkReset,
}

/// Pads for the labelled SPI pins
///
/// According to the datasheet, the combination of PA17, PB22 & PB23 shouldn't
/// work, even though it does. We have added an undocumented `UndocIoSet1` to
/// `Sercom1` for this combination.
pub type SpiPads = spi::Pads<Sercom1, UndocIoSet1, SpiMiso, SpiMosi, SpiSclk>;

/// SPI master for the labelled pins
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

impl SPI {
    /// Convenience for setting up the labelled pins to operate
    /// as an SPI master, running at the specified frequency.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        baud: impl Into<Hertz>,
        sercom1: pac::SERCOM1,
        mclk: &mut MCLK,
    ) -> Spi {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom1_core(&gclk0).unwrap();
        let pads = spi::Pads::default()
            .data_in(self.miso)
            .data_out(self.mosi)
            .sclk(self.sclk);
        spi::Config::new(mclk, sercom1, pads, clock.freq())
            .spi_mode(spi::MODE_0)
            .baud(baud)
            .enable()
    }
}

/// I2C pins
pub struct I2C {
    pub sda: SdaReset,
    pub scl: SclReset,
}

impl I2C {
    /// Convenience for setting up the labelled SDA, SCL pins to
    /// operate as an I2C master running at the specified frequency.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        baud: impl Into<Hertz>,
        sercom2: pac::SERCOM2,
        mclk: &mut MCLK,
    ) -> I2CMaster2<Sda, Scl> {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom2_core(&gclk0).unwrap();
        let baud = baud.into();
        I2CMaster2::new(clock, baud, sercom2, mclk, self.sda.into(), self.scl.into())
    }
}

/// Speaker pins
pub struct Speaker {
    pub speaker: SpeakerReset,
    pub enable: SpeakerEnableReset,
}

/// USB pins
pub struct USB {
    pub dm: UsbDmReset,
    pub dp: UsbDpReset,
}

impl USB {
    #[cfg(feature = "usb")]
    /// Convenience for setting up the onboard usb port to operate
    /// as a USB device.
    pub fn init(
        self,
        usb: pac::USB,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
    ) -> UsbBusAllocator<UsbBus> {
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
        let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();
        let (dm, dp): (UsbDm, UsbDp) = (self.dm.into(), self.dp.into());
        UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, dm, dp, usb))
    }
}

/// UART pins
pub struct UART {
    pub tx: UartTxReset,
    pub rx: UartRxReset,
}

impl UART {
    /// Convenience for setting up the labelled TX, RX pins
    /// to operate as a UART device at the specified baud rate.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        baud: impl Into<Hertz>,
        sercom5: pac::SERCOM5,
        mclk: &mut MCLK,
    ) -> UART5<UartRx, UartTx, (), ()> {
        let gclk0 = clocks.gclk0();
        let clock = &clocks.sercom5_core(&gclk0).unwrap();
        let baud = baud.into();
        UART5::new(clock, baud, sercom5, mclk, (self.rx.into(), self.tx.into()))
    }
}

/// Analog pins
pub struct Analog {
    pub a1: A1Reset,
    pub a2: A2Reset,
    pub a3: A3Reset,
    pub a4: A4Reset,
    pub a5: A5Reset,
}

/// Digital pins
pub struct Digital {
    /// also usabe as A8
    pub d2: D2Reset,
    /// also usabe as A9
    pub d3: D3Reset,
    pub d5: D5Reset,
    pub d6: D6Reset,
    pub d9: D9Reset,
    pub d10: D10Reset,
    pub d11: D11Reset,
    pub d12: D12Reset,
}

/// QSPI flash pins
pub struct QSPIFlash {
    pub sclk: QspiSclkReset,
    pub cs: QspiCsReset,
    pub data0: QspiD0Reset,
    pub data1: QspiD1Reset,
    pub data2: QspiD2Reset,
    pub data3: QspiD3Reset,
}

impl QSPIFlash {
    pub fn init(self, mclk: &mut MCLK, qspi: QSPI) -> qspi::Qspi<qspi::OneShot> {
        qspi::Qspi::new(
            mclk, qspi, self.sclk, self.cs, self.data0, self.data1, self.data2, self.data3,
        )
    }
}

/// Button pins
pub struct Buttons {
    /// Button Latch
    pub latch: ButtonLatchReset,
    /// Button Out
    pub data_in: ButtonOutReset,
    /// Button Clock
    pub clock: ButtonClockReset,
}

#[cfg(feature = "unproven")]
impl Buttons {
    /// Convenience for setting up the button latch pins
    /// Returns ButtonReader iterator which can be polled for Key events
    pub fn init(self) -> ButtonReader {
        let mut latch: ButtonLatch = self.latch.into();
        latch.set_high().ok();
        let mut clock: ButtonClock = self.clock.into();
        clock.set_high().ok();
        ButtonReader {
            latch,
            data_in: self.data_in.into(),
            clock,
            last: 0,
        }
    }
}

/// Joystick pins
pub struct JoystickReader {
    /// Joystick X
    pub joy_x: JoyX,
    /// Joystick Y
    pub joy_y: JoyY,
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
    pub joy_x: JoyXReset,
    /// Joystick Y
    pub joy_y: JoyYReset,
}

#[cfg(feature = "unproven")]
impl Joystick {
    /// Convenience for setting up the joystick. Returns JoystickReader instance
    /// which can be polled for joystick (x,y) tuple
    pub fn init(self) -> JoystickReader {
        JoystickReader {
            joy_x: self.joy_x.into(),
            joy_y: self.joy_y.into(),
        }
    }
}

/// Battery Reader
#[cfg(feature = "unproven")]
pub struct BatteryReader {
    /// Battery pin
    pub battery: BatteryPin,
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
    pub battery: BatteryReset,
}

#[cfg(feature = "unproven")]
impl Battery {
    /// Convenience for reading Battery Volage. Returns BatteryReader instance
    /// which can be polled for battery voltage
    pub fn init(self) -> BatteryReader {
        BatteryReader {
            battery: self.battery.into(),
        }
    }
}
