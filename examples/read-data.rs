#![no_main]
#![no_std]

use accelerometer::Accelerometer;
use accelerometer::RawAccelerometer;
use adxl343::Adxl343;
use cortex_m_rt::entry;
extern crate panic_halt;

use hal::cortex_m::register;
use hal::i2c;
use hal::stm32;
use hal::stm32::I2C2;
use stm32g4xx_hal as hal;

use hal::gpio::gpioa::PA8;
use hal::gpio::gpioa::PA9;
use hal::gpio::AlternateOD;
use hal::gpio::AF4;
use hal::i2c::Config;
use hal::prelude::*;

use adxl343::*;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);

    let sda: PA8<AlternateOD<AF4>> = gpioa.pa8.into_alternate_open_drain();
    let scl: PA9<AlternateOD<AF4>> = gpioa.pa9.into_alternate_open_drain();

    let mut i2c = dp.I2C2.i2c(sda, scl, Config::new(40.khz()), &mut rcc);
    let mut sensor = Adxl343::new(i2c).unwrap();

    loop {
        let mut accel = sensor.accel_raw().unwrap();

        let x = accel.x;
        let y = accel.y;
        let z = accel.z;
    }
}
