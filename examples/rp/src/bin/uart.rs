#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::uart;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let config = uart::Config::default();

    info!("setting up serial port");

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
