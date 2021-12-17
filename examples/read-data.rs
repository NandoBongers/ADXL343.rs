#![no_main]
#![no_std]

use cortex_m_rt::entry;
extern crate panic_halt;

use hal::stm32;
use stm32g4xx_hal as hal;

use hal::gpio::gpioa::PA8;
use hal::gpio::gpioa::PA9;
use hal::gpio::AlternateOD;
use hal::gpio::AF4;
use hal::i2c::Config;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);

    let sda: PA8<AlternateOD<AF4>> = gpioa.pa8.into_alternate_open_drain();
    let scl: PA9<AlternateOD<AF4>> = gpioa.pa9.into_alternate_open_drain();

    let mut i2c = dp.I2C2.i2c(sda, scl, Config::new(40.khz()), &mut rcc);

    let buf: [u8; 5] = [0, 1, 2, 3, 4];
    i2c.write(0x1d, &buf).unwrap();

    loop {}
}
