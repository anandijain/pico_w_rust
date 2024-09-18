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

    
    loop {
        info!("hello worlxdc");
        Timer::after_secs(1).await;
    }
}

