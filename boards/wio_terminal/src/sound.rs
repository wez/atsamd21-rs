use atsamd_hal::adc::Adc;
use atsamd_hal::clock::GenericClockController;
use atsamd_hal::gpio::{Floating, Input, Pc30, Pd11, PfB, Port};
use atsamd_hal::prelude::*;
use atsamd_hal::pwm::{TCC0Pinout, Tcc0Pwm};
use atsamd_hal::target_device::gclk::pchctrl::GEN_A::GCLK11;
use atsamd_hal::target_device::{ADC1, MCLK, TCC0};

/// Buzzer pins
pub struct Buzzer {
    /// Buzzer control pin
    pub ctr: Pd11<Input<Floating>>,
}

impl Buzzer {
    /// Initialize the pin connected to the piezo buzzer. Configure the pin as a
    /// PWM output using TCC0, and return the Pwm instance.
    pub fn init(
        self,
        clocks: &mut GenericClockController,
        tcc0: TCC0,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> Tcc0Pwm {
        let pinout = TCC0Pinout::Pd11(self.ctr.into_function_f(port));

        let gclk0 = clocks.gclk0();
        let pwm0 = Tcc0Pwm::new(
            &clocks.tcc0_tcc1(&gclk0).unwrap(),
            1000.Hz(),
            tcc0,
            pinout,
            mclk,
        );

        pwm0
    }
}

/// Microphone pins
pub struct Microphone {
    /// Microphone output (analog input) pin
    pub mic: Pc30<Input<Floating>>,
}

impl Microphone {
    /// Initialize Pd1 as an ADC input, and return a Tuple containing the ADC
    /// peripheral and the configured pin.
    pub fn init(
        self,
        adc: ADC1,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> (Adc<ADC1>, Pc30<PfB>) {
        let adc1 = Adc::adc1(adc, mclk, clocks, GCLK11);
        let pc30 = self.mic.into_function_b(port);

        (adc1, pc30)
    }
}
