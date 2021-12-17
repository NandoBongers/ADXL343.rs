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
    let mut test = Adxl343::new(i2c);
    
    

    //let reg = adxl343::Adxl343::Register;
   // let buf: &mut [u8];
    //reg = Register::DATAX0;
    //let mut DATA = Adxl343::write_read_register(&test, reg, buf);





    //let mut hallo: &mut Adxl343<I2C2>;

    
    //let mut data = Adxl343::accel_raw(hallo);

    //let mut reg: Register;
    //reg.addr() = 0x32;
    //let mut res: &mut [u8];
    //TODO use register
    //let mut DATAX0 = Adxl343::write_read_register(&mut self, reg, &mut res);
    //let mut DATAX= Adxl343::write_read_u16(&test, register::Register.);

    //0x32 DATA_X0 R DATA_X0[7:0]
    /*   
    0x33 DATA_X1 R DATA_X1[7:0] 0x00
    0x34 DATA_Y0 R DATA_Y0[7:0] 0x00
    0x35 DATA_Y1 R DATA_Y1[7:0] 0x00
    0x36 DATA_Z0 R DATA_Z0[7:0] 0x00
    0x37 DATA_Z1 R DATA_Z1[7:0] 0x00
    */
    

    /*
    let buf: [u8; 5] = [0, 1, 2, 3, 4];
    i2c.write(0x1d, &buf).unwrap();
    */

    loop {}
}
