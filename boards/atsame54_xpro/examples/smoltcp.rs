#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate atsame54_xpro as hal;
extern crate panic_halt;

use core::mem::MaybeUninit;

use cortex_m_rt::DefaultPreInit;
use generic_array::typenum;
use generic_array::GenericArray;
use hal::delay::Delay;
use hal::entry;
use hal::gmac::*;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::watchdog::{Watchdog, WatchdogTimeout};
use hal::{clock::GenericClockController, pac::GMAC};
use smoltcp;

type RxBuffers = GmacBufferSet<RxBufferDescriptor, RxBufferCount, RxBufferSize>;
type TxBuffers = GmacBufferSet<TxBufferDescriptor, TxBufferCount, TxBufferSize>;

type RxBufferCount = typenum::U16;
type RxBufferSize = typenum::U1024;
static mut RX_BUFFERS: MaybeUninit<GenericArray<GmacBuffer<RxBufferSize>, RxBufferCount>> =
    MaybeUninit::uninit();
static mut RX_DESCRIPTORS: MaybeUninit<GenericArray<RxBufferDescriptor, RxBufferCount>> =
    MaybeUninit::uninit();
static mut RX_BUFFER_SET: MaybeUninit<RxBuffers> = MaybeUninit::uninit();

type TxBufferCount = typenum::U16;
type TxBufferSize = typenum::U1024;
static mut TX_BUFFERS: MaybeUninit<GenericArray<GmacBuffer<TxBufferSize>, TxBufferCount>> =
    MaybeUninit::uninit();
static mut TX_DESCRIPTORS: MaybeUninit<GenericArray<TxBufferDescriptor, TxBufferCount>> =
    MaybeUninit::uninit();
static mut TX_BUFFER_SET: MaybeUninit<TxBuffers> = MaybeUninit::uninit();

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    delay.delay_ms(400u16);

    let pins = hal::Pins::new(peripherals.PORT);
    let mut sets = pins.split();
    let mut led = sets.led.into_open_drain_output(&mut sets.port);

    let rx_buffers = unsafe {
        RX_DESCRIPTORS.as_mut_ptr().write(GenericArray::default());
        RX_BUFFERS.as_mut_ptr().write(GenericArray::default());
        let descriptors = RX_DESCRIPTORS.as_mut_ptr();
        let buffers = RX_BUFFERS.as_mut_ptr();
        let rx_buffers: GmacBufferSet<
            RxBufferDescriptor,
            generic_array::typenum::U16,
            generic_array::typenum::U1024,
        > = RxBuffers::new(&mut *descriptors, &mut *buffers);
        rx_buffers
    };
    let tx_buffers = unsafe {
        TX_DESCRIPTORS.as_mut_ptr().write(GenericArray::default());
        TX_BUFFERS.as_mut_ptr().write(GenericArray::default());
        let descriptors = TX_DESCRIPTORS.as_mut_ptr();
        let buffers = TX_BUFFERS.as_mut_ptr();
        let tx_buffers: GmacBufferSet<
            TxBufferDescriptor,
            generic_array::typenum::U16,
            generic_array::typenum::U1024,
        > = TxBuffers::new(&mut *descriptors, &mut *buffers);
        tx_buffers
    };
    let (_gmac, _gmac_int, _gmac_reset) = sets.ethernet.init(
        &mut sets.port,
        &peripherals.MCLK,
        peripherals.GMAC,
        rx_buffers,
        tx_buffers,
    );

    let mut wdt = Watchdog::new(peripherals.WDT);
    wdt.start(WatchdogTimeout::Cycles256 as u8);

    loop {
        delay.delay_ms(200u8);
        wdt.feed();
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        wdt.feed();
        led.set_low().unwrap();
    }
}
