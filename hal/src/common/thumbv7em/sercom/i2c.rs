// Note: section 7.2.3 shows which pins support I2C Hs mode

use crate::clock;
use crate::hal::blocking::i2c::{Read, Write, WriteRead};
use crate::target_device::sercom0::I2CM;
use crate::target_device::{MCLK, SERCOM0, SERCOM1, SERCOM2, SERCOM3, SERCOM4, SERCOM5};
#[cfg(feature = "min-samd51n")]
use crate::target_device::{SERCOM6, SERCOM7};
use embedded_time::rate::Hertz;

const BUS_STATE_IDLE: u8 = 1;
const BUS_STATE_OWNED: u8 = 2;

const MASTER_ACT_READ: u8 = 2;
const MASTER_ACT_STOP: u8 = 3;

/// Define an I2C master type for the given SERCOM and pad pair.
macro_rules! i2c {
    ([
        $($Type:ident:
            (
                $pad0:ident,
                $pad1:ident,
                $SERCOM:ident,
                $powermask:ident,
                $clock:ident,
                $apmask:ident
            ),
        )+
    ]) => {

        $(

/// Represents the Sercom instance configured to act as an I2C Master.
/// The embedded_hal blocking I2C traits are implemented by this instance.
pub struct $Type<$pad0, $pad1> {
    sda: $pad0,
    scl: $pad1,
    sercom: $SERCOM,
}

impl<$pad0, $pad1> $Type<$pad0, $pad1> {
    /// Configures the sercom instance to work as an I2C Master.
    /// The clock is obtained via the `GenericClockGenerator` type.
    /// `freq` specifies the bus frequency to use for I2C communication.
    /// There are typically a handful of values that tend to be supported;
    /// standard mode is 100.khz(), full speed mode is 400.khz().
    /// The hardware in the atsamd device supports fast mode at 1.mhz()
    /// and fast mode, but there may be additional hardware configuration
    /// missing from the current software implementation that prevents that
    /// from working as-written today.
    ///
    /// ```no_run
    /// let mut i2c = I2CMaster3::new(
    ///     &clocks.sercom3_core(&gclk0).unwrap(),
    ///     400.khz(),
    ///     p.device.SERCOM3,
    ///     &mut p.device.MCLK,
    ///     // Metro M0 express has I2C on pins PA22, PA23
    ///     pins.pa22.into_pad(&mut pins.port),
    ///     pins.pa23.into_pad(&mut pins.port),
    /// );
    /// ```
    pub fn new<F: Into<Hertz>>(
        clock: &clock::$clock,
        freq: F,
        sercom: $SERCOM,
        mclk: &mut MCLK,
        sda: $pad0,
        scl: $pad1,
    ) -> Self {
        // Power up the peripheral bus clock.
        // safe because we're exclusively owning SERCOM
        mclk.$apmask.modify(|_, w| w.$powermask().set_bit());

        unsafe {
            // reset the sercom instance
            sercom.i2cm().ctrla.modify(|_, w| w.swrst().set_bit());
            // wait for reset to complete
            while sercom.i2cm().syncbusy.read().swrst().bit_is_set()
                || sercom.i2cm().ctrla.read().swrst().bit_is_set()
            {}

            // Put the hardware into i2c master mode
            sercom.i2cm().ctrla.modify(|_, w| w.mode().i2c_master());
            // wait for configuration to take effect
            while sercom.i2cm().syncbusy.read().enable().bit_is_set() {}

            // set the baud rate
            let gclk = clock.freq();
            let baud = (gclk.0 / (2 * freq.into().0) - 1) as u8;
            sercom.i2cm().baud.modify(|_, w| w.baud().bits(baud));

            sercom.i2cm().ctrla.modify(|_, w| w.enable().set_bit());
            // wait for configuration to take effect
            while sercom.i2cm().syncbusy.read().enable().bit_is_set() {}

            // set the bus idle
            sercom
                .i2cm()
                .status
                .modify(|_, w| w.busstate().bits(BUS_STATE_IDLE));
            // wait for it to take effect
            while sercom.i2cm().syncbusy.read().sysop().bit_is_set() {}
        }

        Self { sda, scl, sercom }
    }

    /// Breaks the sercom device up into its constituent pins and the SERCOM
    /// instance.  Does not make any changes to power management.
    pub fn free(self) -> ($pad0, $pad1, $SERCOM) {
        (self.sda, self.scl, self.sercom)
    }

    fn start_tx_write(&mut self, addr: u8) -> Result<(), I2CError> {
        loop {
            match self.i2cm().status.read().busstate().bits() {
                BUS_STATE_IDLE | BUS_STATE_OWNED => break,
                _ => continue,
            }
        }

        // Signal start and transmit encoded address.
        unsafe {
            self.i2cm()
                .addr
                .write(|w| w.addr().bits((addr as u16) << 1));
        }

        // wait for transmission to complete
        while !self.i2cm().intflag.read().mb().bit_is_set() {}

        self.status_to_err()
    }

    fn status_to_err(&mut self) -> Result<(), I2CError> {
        let status = self.i2cm().status.read();
        if status.arblost().bit_is_set() {
            return Err(I2CError::ArbitrationLost);
        }
        if status.buserr().bit_is_set() {
            return Err(I2CError::BusError);
        }
        if status.rxnack().bit_is_set() {
            return Err(I2CError::Nack);
        }
        if status.lowtout().bit_is_set() || status.sexttout().bit_is_set()
            || status.mexttout().bit_is_set()
        {
            return Err(I2CError::Timeout);
        }

        Ok(())
    }

    fn start_tx_read(&mut self, addr: u8) -> Result<(), I2CError> {
        loop {
            match self.i2cm().status.read().busstate().bits() {
                BUS_STATE_IDLE | BUS_STATE_OWNED => break,
                _ => continue,
            }
        }

        self.i2cm().intflag.modify(|_, w| w.error().clear_bit());

        // Signal start (or rep start if appropriate)
        // and transmit encoded address.
        unsafe {
            self.i2cm()
                .addr
                .write(|w| w.addr().bits(((addr as u16) << 1) | 1));
        }

        // wait for transmission to complete
        loop {
            let intflag = self.i2cm().intflag.read();
            // If arbitration was lost, it will be signalled via the mb bit
            if intflag.mb().bit_is_set() {
                return Err(I2CError::ArbitrationLost);
            }
            if intflag.sb().bit_is_set() || intflag.error().bit_is_set() {
                break;
            }
        }

        self.status_to_err()
    }

    fn wait_sync(&mut self) {
        while self.i2cm().syncbusy.read().sysop().bit_is_set() {}
    }

    fn cmd(&mut self, cmd: u8) {
        unsafe {
            self.i2cm().ctrlb.modify(|_, w| w.cmd().bits(cmd));
        }
        self.wait_sync();
    }

    fn cmd_stop(&mut self) {
        self.cmd(MASTER_ACT_STOP)
    }

    fn cmd_read(&mut self) {
        unsafe {
            self.i2cm().ctrlb.modify(|_, w| {
                // clear bit means send ack
                w.ackact().clear_bit();
                w.cmd().bits(MASTER_ACT_READ)
            });
        }
        self.wait_sync();
    }

    fn i2cm(&mut self) -> &I2CM {
        self.sercom.i2cm()
    }

    fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), I2CError> {
        for b in bytes {
            unsafe {
                self.i2cm().data.write(|w| w.bits(*b));
            }

            loop {
                let intflag = self.i2cm().intflag.read();
                if intflag.mb().bit_is_set() || intflag.error().bit_is_set() {
                    break;
                }
            }
            self.status_to_err()?;
        }
        Ok(())
    }

    fn read_one(&mut self) -> u8 {
        while !self.i2cm().intflag.read().sb().bit_is_set() {}
        self.i2cm().data.read().bits() as u8
    }

    fn fill_buffer(&mut self, buffer: &mut [u8]) -> Result<(), I2CError> {
        // Some manual iterator gumph because we need to ack bytes after the first.
        let mut iter = buffer.iter_mut();
        *iter.next().expect("buffer len is at least 1") = self.read_one();

        loop {
            match iter.next() {
                None => break,
                Some(dest) => {
                    // Ack the last byte so that we can receive another one
                    self.cmd_read();
                    *dest = self.read_one();
                }
            }
        }

        // arrange to send nack on next command to
        // stop slave from transmitting more data
        self.i2cm().ctrlb.modify(|_, w| w.ackact().set_bit());

        Ok(())
    }

    fn do_write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), I2CError> {
        self.start_tx_write(addr)?;
        self.send_bytes(bytes)
    }

    fn do_read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2CError> {
        self.start_tx_read(addr)?;
        self.fill_buffer(buffer)
    }

    fn do_write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), I2CError> {
        self.start_tx_write(addr)?;
        self.send_bytes(bytes)?;
        self.start_tx_read(addr)?;
        self.fill_buffer(buffer)
    }
}

impl<$pad0, $pad1> Write for $Type<$pad0, $pad1> {
    type Error = I2CError;

    /// Sends bytes to slave with address `addr`
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        let res = self.do_write(addr, bytes);
        self.cmd_stop();
        res
    }
}

impl<$pad0, $pad1> Read for $Type<$pad0, $pad1> {
    type Error = I2CError;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let res = self.do_read(addr, buffer);
        self.cmd_stop();
        res
    }
}

impl<$pad0, $pad1> WriteRead for $Type<$pad0, $pad1> {
    type Error = I2CError;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        let res = self.do_write_read(addr, bytes, buffer);
        self.cmd_stop();
        res
    }
}

        )+

    };
}

i2c!([
    I2CMaster0:
        (
            Sercom0Pad0,
            Sercom0Pad1,
            SERCOM0,
            sercom0_,
            Sercom0CoreClock,
            apbamask
        ),
    I2CMaster1:
        (
            Sercom1Pad0,
            Sercom1Pad1,
            SERCOM1,
            sercom1_,
            Sercom1CoreClock,
            apbamask
        ),
    I2CMaster2:
        (
            Sercom2Pad0,
            Sercom2Pad1,
            SERCOM2,
            sercom2_,
            Sercom2CoreClock,
            apbbmask
        ),
    I2CMaster3:
        (
            Sercom3Pad0,
            Sercom3Pad1,
            SERCOM3,
            sercom3_,
            Sercom3CoreClock,
            apbbmask
        ),
    I2CMaster4:
        (
            Sercom4Pad0,
            Sercom4Pad1,
            SERCOM4,
            sercom4_,
            Sercom4CoreClock,
            apbdmask
        ),
    I2CMaster5:
        (
            Sercom5Pad0,
            Sercom5Pad1,
            SERCOM5,
            sercom5_,
            Sercom5CoreClock,
            apbdmask
        ),
]);

#[cfg(feature = "min-samd51n")]
i2c!([
    I2CMaster6:
        (
            Sercom6Pad0,
            Sercom6Pad1,
            SERCOM6,
            sercom6_,
            Sercom6CoreClock,
            apbdmask
        ),
    I2CMaster7:
        (
            Sercom7Pad0,
            Sercom7Pad1,
            SERCOM7,
            sercom7_,
            Sercom7CoreClock,
            apbdmask
        ),
]);

#[derive(Debug)]
pub enum I2CError {
    ArbitrationLost,
    AddressError,
    BusError,
    Timeout,
    Nack,
}
