#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_rp::interrupt;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    log::info!("firing up ...");
    let p = embassy_rp::init(Default::default());
    let irq = interrupt::take!(USBCTRL_IRQ);
    let driver = Driver::new(p.USB, irq);
    spawner.spawn(logger_task(driver)).unwrap();

    log::info!("entering loop ...");

    // FIXME: i can read data off this at 9600 8n1 but its very flakey, why?

    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("Tick {}\n", counter);
        Timer::after(Duration::from_secs(1)).await;
    }
}
