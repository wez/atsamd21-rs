use atsamd_hal::adc::Adc;
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::{Floating, Input, Pa12, Pa13, Pd1, PfB, PfD, Port};
use atsamd_hal::prelude::*;
use atsamd_hal::sercom::{I2CMaster4, PadPin, Sercom4Pad0, Sercom4Pad1};
use atsamd_hal::target_device::gclk::pchctrl::GEN_A::GCLK11;
use atsamd_hal::target_device::{ADC1, MCLK, SERCOM4};
use atsamd_hal::time::Hertz;
use core::convert::TryFrom;

use lis3dh::{Lis3dh, SlaveAddr};

/// I2C Accelerometer pins (uses `SERCOM4`)
pub struct Accelerometer {
    /// `I2C0` bus clock pin
    pub scl: Pa12<Input<Floating>>,

    /// `I2C0` bus data pin
    pub sda: Pa13<Input<Floating>>,
}

impl Accelerometer {
    /// Initialize the LIS3DH accelerometer using the correct pins and
    // peripherals. Use the driver's default settings.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        sercom4: SERCOM4,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> Lis3dh<I2CMaster4<Sercom4Pad0<Pa13<PfD>>, Sercom4Pad1<Pa12<PfD>>>> {
        // The accelerometer is connected to the Wio Terminal's `I2C0` bus, so
        // based on the possible padouts listed in the datasheet it must use
        // `SERCOM4` and in turn `I2CMaster4`.
        let gclk0 = clocks.gclk0();
        let i2c = I2CMaster4::new(
            &clocks.sercom4_core(&gclk0).unwrap(),
            Hertz::try_from(Hertz::from(400.kHz())).unwrap(),
            sercom4,
            mclk,
            self.sda.into_pad(port),
            self.scl.into_pad(port),
        );

        // The schematic states that the alternate I2C address `0x19` is used,
        // but that doesn't appear to work!
        Lis3dh::new(i2c, SlaveAddr::Default).unwrap()
    }
}

/// Analog Light Sensor
pub struct LightSensor {
    /// Analog Light Sensor input pin
    pub pd1: Pd1<Input<Floating>>,
}

impl LightSensor {
    /// Initialize Pd1 as an ADC input, and return a Tuple containing the ADC
    /// peripheral and the configured pin.
    pub fn init(
        self,
        adc: ADC1,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> (Adc<ADC1>, Pd1<PfB>) {
        let adc1 = Adc::adc1(adc, mclk, clocks, GCLK11);
        let pd1 = self.pd1.into_function_b(port);

        (adc1, pd1)
    }
}
