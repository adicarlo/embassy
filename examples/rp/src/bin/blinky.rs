#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_15, Level::Low);
    let dur = Duration::from_millis(1000);

    loop {
        info!("led on!");
        led.set_high();
        Timer::after(dur).await;

        info!("led off!");
        led.set_low();
        Timer::after(dur).await;
    }
}
