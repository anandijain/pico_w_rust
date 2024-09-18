#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::i2c::{self, Blocking, Config};
use embedded_hal::i2c::I2c;
use embassy_time::Timer;
// use {defmt_rtt as _, panic_probe as _};
use panic_probe as _;
use defmt_serial as _;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_14;
    let scl = p.PIN_15;

    info!("set up i2c ");
    let mut i2c = i2c::I2c::new_blocking(p.I2C1, scl, sda, Config::default());

    info!("Scanning I2C bus...");

    for addr in 0x03..=0x77 {
        let result = i2c.write(addr, &[]);
        if result.is_ok() {
            info!("Found I2C device at address: 0x{:02x}", addr);
        }
    }

    info!("I2C scan complete.");

    loop {
        Timer::after_secs(1).await;
    }
}
