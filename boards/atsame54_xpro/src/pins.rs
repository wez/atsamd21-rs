//! Grand Central M4 Express Pins

use super::{hal, pac::MCLK, pac::SERCOM2, pac::SERCOM6, pac::SERCOM7, target_device};

use hal::define_pins;
use hal::gmac::*;
use hal::gpio::{self, *};
use hal::sercom::{
    I2CMaster7, PadPin, SPIMaster6, Sercom2Pad0, Sercom2Pad1, Sercom6Pad0, Sercom6Pad1,
    Sercom6Pad3, Sercom7Pad0, Sercom7Pad1, UART2,
};
use hal::time::Hertz;

use hal::clock::GenericClockController;

#[cfg(feature = "usb")]
use super::pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    pin sw0 = b31,
    pin led = c18,
    pin qt_button = a16,

    pin tx = b25,   // SERCOM2 Pad0
    pin rx = b24,   // SERCOM2 Pad1

    pin sda = d8,   // SERCOM7 Pad0
    pin scl = d9,   // SERCOM7 Pad1

    pin mosi = c4,  // SERCOM6 Pad0
    pin miso = c7,  // SERCOM6 Pad3
    pin sck = c5,   // SERCOM6 Pad1

    // Exension Header EXT1
    pin pb04 = b4,  // ADC+
    pin pb05 = b5,  // ADC-
    pin pa06 = a6,  // GPIO1
    pin pa07 = a7,  // GPIO2
    pin pb08 = b8,  // PWM+
    pin pb09 = b9,  // PWM-
    pin pb07 = b7,  // IRQ/GPIO
    pin pa27 = a27, // SPI_SS_B/GPIO
    pin uart0_rx = a5,   // SERCOM0 Pad1
    pin uart0_tx = a4,   // SERCOM0 Pad0
    // pin spi4_ss = b28,   // SPI_SS_A SERCOM4 Pad2
    pin spi4_mosi = b27, // SPI_MOSI SERCOM4 Pad0
    // pin spi4_miso = b29, // SPI_MISO SERCOM4 Pad3
    pin spi4_sck = b26,  // SPI_SCK SERCOM4 Pad1

    // Exension Header EXT2
    pin pb00 = b0,  // ADC+
    pin pa03 = a3,  // ADC-
    pin pb01 = b1,  // GPIO1
    pin pb06 = b6,  // GPIO2
    pin pb14 = b14, // PWM+
    pin pb15 = b15, // PWM-
    pin pd00 = d0,  // IRQ/GPIO
    pin pb02 = b2,  // SPI_SS_B/GPIO
    pin spi6_ss = c6,    // SPI_SS_A SERCOM6 Pad2

    // Exension Header EXT3
    pin pc02 = c2,   // ADC+
    pin pc03 = c3,   // ADC-
    pin pc01 = c1,   // GPIO1
    pin pc10 = c10,  // GPIO2
    pin pd10 = d10,  // PWM+
    pin pd11 = d11,  // PWM-
    pin pc30 = c30,  // IRQ/GPIO
    pin pc31 = c31,  // SPI_SS_B/GPIO
    pin uart1_rx = c23,  // SERCOM5 Pad1
    pin uart1_tx = c22,  // SERCOM5 Pad0
    pin pc14 = c14,      // SPI_SS_A SERCOM6 Pad2

    // SD/SDIO
    pin sd_d0 = b18,
    pin sd_d1 = b19,
    pin sd_d2 = b20,
    pin sd_d3 = b21,
    pin sd_clk_i2s_sdo = a21,
    pin sd_cmd_i2s_fs0 = a20,
    pin sd_cd = d20,
    pin sd_wp = d21,

    // PCC Camera Connector
    // pin pcc_vsync = a12,
    // pin pcc_hsync = a13,
    // pin pcc_pclk = a14,
    // pin pcc_xclk = a15,
    // pin pcc_dout7 = a23,
    // pin pcc_dout6 = a22,
    // pin pcc_dout5 = a21,
    // pin pcc_dout4 = a20,
    // pin pcc_dout3 = a19,
    // pin pcc_dout2 = a18,
    // pin pcc_dout1 = a17,
    // pin pcc_dout0 = a16,
    // pin pcc_dout9 = b15,
    // pin pcc_dout8 = b14,
    // pin pcc_reset = c12,
    // pin pcc_pwdn = c11,

    // Position Decoder Header
    pin pdec_phase_a = c16,
    pin pdec_phase_b = c17,
    // pin pdec_index = c18,    // same as led

    pin vbat = b3,

    // ADC/DAC Header
    pin adc_dac = a2,

    // USB
    pin vbus_detecion = c0,
    pin usb_id = c19,
    pin usb_dm = a24,
    pin usb_dp = a25,

    pin swd = a31,
    pin swc = a30,
    pin swo = b30,

    // CAN
    pin can_tx = b12,
    pin can_rx = b13,
    pin can_standby = c13,

    // Ethernet
    pin gtxck = a14,
    pin gtxen = a17,
    pin gtx0 = a18,
    pin gtx1 = a19,
    pin grxdv = c20,
    pin grx0 = a13,
    pin grx1 = a12,
    pin grxer = a15,
    pin gmdc = c11,
    pin gmdio = c12,
    pin ethernet_interrupt = d12,
    pin ethernet_reset = c21,

    // QSPI
    pin flash_io0 = a8,
    pin flash_io1 = a9,
    pin flash_io2 = a10,
    pin flash_io3 = a11,
    pin flash_sck = b10,
    pin flash_cs = b11,

    // I²S
    pin i2s_sck0 = b16,
    pin i2s_mck0 = b17,
    pin i2s_sdi = a22,
    pin i2s_fs1 = a23,
    pin i2s_sck1 = b28,
    pin i2s_mck1 = b29,

    // Data Gateway Interface
    pin dgi_ss = d1,
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let analog = Analog {
        //     a0: self.a0,
        //     a1: self.a1,
        //     a2: self.a2,
        //     a3: self.a3,
        //     a4: self.a4,
        //     a5: self.a5,
        //     a6: self.a6,
        //     a7: self.a7,
        //     a8: self.a8,
        //     a9: self.a9,
        //     a10: self.a10,
        //     a11: self.a11,
        //     a12: self.a12,
        //     a13: self.a13,
        //     a14: self.a14,
        //     a15: self.a15,
        };

        let flash = QSPIFlash {
            sck: self.flash_sck,
            cs: self.flash_cs,
            data0: self.flash_io0,
            data1: self.flash_io1,
            data2: self.flash_io2,
            data3: self.flash_io3,
        };

        let spi = SPI {
            sck: self.sck,
            mosi: self.mosi,
            miso: self.miso,
        };

        let sdcard = SdCard {
            d0: self.sd_d0,
            d1: self.sd_d1,
            d2: self.sd_d2,
            d3: self.sd_d3,
            clk: self.sd_clk_i2s_sdo,
            cmd: self.sd_cmd_i2s_fs0,
            cd: self.sd_cd,
            wp: self.sd_wp,
        };

        let i2c = I2C {
            sda: self.sda,
            scl: self.scl,
        };

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let ethernet = Ethernet {
            gtxck: self.gtxck,
            gtxen: self.gtxen,
            gtx0: self.gtx0,
            gtx1: self.gtx1,
            grxdv: self.grxdv,
            grx0: self.grx0,
            grx1: self.grx1,
            grxer: self.grxer,
            gmdc: self.gmdc,
            gmdio: self.gmdio,
            ethernet_interrupt: self.ethernet_interrupt,
            ethernet_reset: self.ethernet_reset,
        };

        let uart = UART_ {
            rx: self.rx,
            tx: self.tx,
        };

        Sets {
            port: self.port,
            analog,
            spi,
            usb,
            ethernet,
            flash,
            sdcard,
            i2c,
            uart,
            led: self.led,
            sw0: self.sw0,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    pub led: Pc18<Input<Floating>>,
    pub sw0: Pb31<Input<Floating>>,

    /// Analog pins
    pub analog: Analog,

    /// SPI (external pinout) pins
    pub spi: SPI,

    /// SdCard
    pub sdcard: SdCard,

    /// I2C (external pinout) pins
    pub i2c: I2C,

    /// QSPI Flash pins
    pub flash: QSPIFlash,

    /// USB pins
    pub usb: USB,

    /// Ethernet pins
    pub ethernet: Ethernet,

    /// UART (external pinout) pins
    pub uart: UART_,
    /// Port
    pub port: Port,
}

/// SPI pins
pub struct SPI {
    pub mosi: gpio::Pc4<Input<Floating>>,
    pub miso: gpio::Pc7<Input<Floating>>,
    pub sck: gpio::Pc5<Input<Floating>>,
}

impl SPI {
    #[allow(clippy::clippy::type_complexity)]
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom6: SERCOM6,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> SPIMaster6<
        Sercom6Pad3<gpio::Pc7<gpio::PfC>>,
        Sercom6Pad0<gpio::Pc4<gpio::PfC>>,
        Sercom6Pad1<gpio::Pc5<gpio::PfC>>,
    > {
        let gclk0 = clocks.gclk0();
        SPIMaster6::new(
            &clocks.sercom6_core(&gclk0).unwrap(),
            bus_speed.into(),
            hal::hal::spi::Mode {
                phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
                polarity: hal::hal::spi::Polarity::IdleLow,
            },
            sercom6,
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
    pub sda: Pd8<Input<Floating>>,
    pub scl: Pd9<Input<Floating>>,
}

impl I2C {
    /// Convenience for setting up the labelled SDA, SCL pins to
    /// operate as an I2C master running at the specified frequency.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        bus_speed: F,
        sercom7: SERCOM7,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> I2CMaster7<Sercom7Pad0<Pd8<PfC>>, Sercom7Pad1<Pd9<PfC>>> {
        let gclk0 = clocks.gclk0();
        I2CMaster7::new(
            &clocks.sercom7_core(&gclk0).unwrap(),
            bus_speed.into(),
            sercom7,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        )
    }
}

/// Sd Card pins
pub struct SdCard {
    pub d0: Pb18<Input<Floating>>,
    pub d1: Pb19<Input<Floating>>,
    pub d2: Pb20<Input<Floating>>,
    pub d3: Pb21<Input<Floating>>,
    pub clk: Pa21<Input<Floating>>,
    pub cmd: Pa20<Input<Floating>>,
    pub cd: Pd20<Input<Floating>>,
    pub wp: Pd21<Input<Floating>>,
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

/// Ethernet pins
pub struct Ethernet {
    pub gtxck: gpio::Pa14<Input<Floating>>,
    pub gtxen: gpio::Pa17<Input<Floating>>,
    pub gtx0: gpio::Pa18<Input<Floating>>,
    pub gtx1: gpio::Pa19<Input<Floating>>,
    pub grxdv: gpio::Pc20<Input<Floating>>,
    pub grx0: gpio::Pa13<Input<Floating>>,
    pub grx1: gpio::Pa12<Input<Floating>>,
    pub grxer: gpio::Pa15<Input<Floating>>,
    pub gmdc: gpio::Pc11<Input<Floating>>,
    pub gmdio: gpio::Pc12<Input<Floating>>,
    pub ethernet_interrupt: gpio::Pd12<Input<Floating>>,
    pub ethernet_reset: gpio::Pc21<Input<Floating>>,
}

type EthernetPadout = RmiiPadout<Ioset<Pc11<Input<Floating>>, Pc12<Input<Floating>>>>;

impl Ethernet {
    pub fn init<RxBuf, TxBuf>(
        self,
        port: &mut Port,
        mclk: &MCLK,
        gmac: target_device::GMAC,
        rx_buffers: RxBuf,
        tx_buffers: TxBuf,
    ) -> (
        Gmac<EthernetPadout, RxBuf, TxBuf>,
        gpio::Pd12<Input<PullUp>>,
        gpio::Pc21<Output<PushPull>>,
    )
    where
        RxBuf: BufferSet<RxBufferDescriptor>,
        TxBuf: BufferSet<TxBufferDescriptor>,
    {
        let padout = RmiiPadout::new(
            port,
            self.gtxck,
            self.gtxen,
            self.gtx0,
            self.gtx1,
            self.grxdv,
            self.grx0,
            self.grx1,
            self.grxer,
            Ioset::new2(self.gmdc, self.gmdio),
        );
        (
            Gmac::new(padout, mclk, gmac, rx_buffers, tx_buffers),
            self.ethernet_interrupt.into_pull_up_input(port),
            self.ethernet_reset.into_push_pull_output(port),
        )
    }
}

/// UART pins
pub struct UART_ {
    pub tx: Pb25<Input<Floating>>,
    pub rx: Pb24<Input<Floating>>,
}

impl UART_ {
    /// Convenience for setting up the labelled TX, RX pins
    /// to operate as a UART device at the specified baud rate.
    pub fn init<F: Into<Hertz>>(
        self,
        clocks: &mut GenericClockController,
        baud: F,
        sercom2: SERCOM2,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UART2<Sercom2Pad1<gpio::Pb24<gpio::PfD>>, Sercom2Pad0<gpio::Pb25<gpio::PfD>>, (), ()> {
        let gclk0 = clocks.gclk0();

        UART2::new(
            &clocks.sercom2_core(&gclk0).unwrap(),
            baud.into(),
            sercom2,
            mclk,
            (self.rx.into_pad(port), self.tx.into_pad(port)),
        )
    }
}

pub struct Analog {
    //     pub a0: Pa2<Input<Floating>>,
//     pub a1: Pa5<Input<Floating>>,
//     pub a2: Pb3<Input<Floating>>,
//     pub a3: Pc0<Input<Floating>>,
//     pub a4: Pc1<Input<Floating>>,
//     pub a5: Pc2<Input<Floating>>,
//     pub a6: Pc3<Input<Floating>>,
//     pub a7: Pb4<Input<Floating>>,
//     pub a8: Pb5<Input<Floating>>,
//     pub a9: Pb6<Input<Floating>>,
//     pub a10: Pb7<Input<Floating>>,
//     pub a11: Pb8<Input<Floating>>,
//     pub a12: Pb9<Input<Floating>>,
//     pub a13: Pa4<Input<Floating>>,
//     pub a14: Pa6<Input<Floating>>,
//     pub a15: Pa7<Input<Floating>>,
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
