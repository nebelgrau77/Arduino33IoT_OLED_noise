//!
//! Based on an example from https://github.com/jamwaffles/ssd1306
//! Sends random raw data to the display, emulating an old untuned TV. 
//! Retrieves the underlying display properties struct and allows calling of the low-level `draw()` method,
//! sending a 1024 byte buffer straight to the display.
//! 
//! Uses SmallRng as random number generator.
//! NOTE: these are pseudorandom numbers, not suitable for cryptographic or similar purposes.
//! 


#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;
extern crate ssd1306;
extern crate rand;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::time::KiloHertz;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use ssd1306::{mode::displaymode::DisplayModeTrait, prelude::*, Builder, I2CDIBuilder};

use rand::prelude::*;

const BOOT_DELAY_MS: u16 = 100; //small delay for the I2C to initiate correctly and start on boot without having to reset the board

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);    
    let mut delay = Delay::new(core.SYST, &mut clocks);
    
    delay.delay_ms(BOOT_DELAY_MS);

    let i2c = hal::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM4, 
        &mut peripherals.PM, 
        pins.sda,
        pins.scl,
        &mut pins.port,
    );  

    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();       
    disp.init().unwrap();
    
    let mut props = disp.into_properties();
    
    let mut buf = [0x00u8; 1024]; 

    let mut rng = SmallRng::seed_from_u64(0x0101_0303_0808_0909);

    loop {
        rng.fill_bytes(&mut buf); 
        props.draw(&buf).unwrap();
    }


}


