//! This example showcases the uart::v2 module.

#![no_std]
#![no_main]

use cortex_m::asm;
use panic_halt as _;

use bsp::hal;
use bsp::pac;
use feather_m4 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::dmac::{DmaController, PriorityLevel};
use hal::prelude::*;

use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut mclk = peripherals.MCLK;
    let dmac = peripherals.DMAC;
    let pins = bsp::Pins::new(peripherals.PORT);

    // Take RX and TX pins
    let (rx_pin, tx_pin) = (pins.d0, pins.d1);

    // Setup DMA channels for later use
    let mut dmac = DmaController::init(dmac, &mut peripherals.PM);
    let channels = dmac.split();

    let chan0 = channels.0.init(PriorityLevel::LVL0);
    let chan1 = channels.1.init(PriorityLevel::LVL0);

    // Setup UART peripheral
    let uart = bsp::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM5,
        &mut mclk,
        rx_pin,
        tx_pin,
    );

    // Split uart in rx + tx halves
    let (mut rx, mut tx) = uart.split();

    // Get a 50 byte buffer to store data to send/receive
    const LENGTH: usize = 50;
    let rx_buffer: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();
    let tx_buffer: &'static mut [u8; LENGTH] =
        cortex_m::singleton!(: [u8; LENGTH] = [0x00; LENGTH]).unwrap();

    // For fun, store numbers from 0 to 49 in buffer
    for (i, c) in tx_buffer.iter_mut().enumerate() {
        *c = i as u8;
    }

    // Send data in a blocking way
    for c in tx_buffer.iter() {
        nb::block!(tx.write(*c)).unwrap();
    }

    // We'll now receive data in a blocking way
    rx.flush_rx_buffer();
    for c in rx_buffer.iter_mut() {
        *c = nb::block!(rx.read()).unwrap();
    }

    // Finally, we'll receive AND send data at the same time with DMA

    // Setup a DMA transfer to send our data asynchronously.
    // We'll set the waker to be a no-op
    let tx_dma = tx.send_with_dma(tx_buffer, chan0, |_| {});

    // Setup a DMA transfer to receive our data asynchronously.
    // Again, we'll set the waker to be a no-op
    let rx_dma = rx.receive_with_dma(rx_buffer, chan1, |_| {});

    // Wait for transmit DMA transfer to complete
    let (_chan0, _tx_buffer, _tx) = tx_dma.wait();

    // Wait for receive DMA transfer to complete
    let (_chan1, _rx_buffer, _rx) = rx_dma.wait();

    loop {
        // Go to sleep
        asm::wfi();
    }
}
