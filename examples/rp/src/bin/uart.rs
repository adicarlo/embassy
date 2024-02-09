//! This example shows how to use UART (Universal asynchronous receiver-transmitter) in the RP2040 chip.
//!
//! No specific hardware is specified in this example. Only output on pin 0 is tested.
//! The Raspberry Pi Debug Probe (https://www.raspberrypi.com/products/debug-probe/) could be used
//! with its UART port.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::uart;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let config = uart::Config::default();

    info!("setting up serial port");

    // to see serial, run, for example:
    //   screen /dev/ttyUSB0 115200
    // also works, requires 4 wires
    let mut uart = uart::Uart::new_with_rtscts_blocking(p.UART0, p.PIN_0, p.PIN_1, p.PIN_3, p.PIN_2, config);
    // works:
    // let mut uart = uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config);
    info!("initial serial write with RTS");
    uart.blocking_write("Hello World!\r\n".as_bytes()).unwrap();

    info!("entering loop");
    loop {
        uart.blocking_write("hello there 2!\r\n".as_bytes()).unwrap();
        cortex_m::asm::delay(1_000_000);
    }
}
