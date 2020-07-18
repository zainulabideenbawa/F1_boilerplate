// #![no_std]
// #![no_main]

// extern crate cortex_m_rt;
// extern crate panic_halt;
// extern crate stm32f1;

// use cortex_m_rt::entry;
// use stm32f1::stm32f103;

// // use `main` as the entry point of this application
// #[entry]
// fn main() -> ! {
//     // get handles to the hardware
//     let peripherals = stm32f103::Peripherals::take().unwrap();
//     let gpioc = &peripherals.GPIOC;
//     let rcc = &peripherals.RCC;

//     // enable the GPIO clock for IO port C
//     rcc.apb2enr.write(|w| w.iopcen().set_bit());
//     gpioc.crh.write(|w| unsafe {
//         w.mode13().bits(0b11);
//         w.cnf13().bits(0b00)
//     });

//     loop {
//         gpioc.bsrr.write(|w| w.bs13().set_bit());
//         cortex_m::asm::delay(2000000);
//         gpioc.brr.write(|w| w.br13().set_bit());
//         cortex_m::asm::delay(2000000);
//     }
// }

//! Blinks an LED

//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;
use cortex_m_semihosting::hprintln;

use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    //let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut led1 = gpioc.pc14.into_push_pull_output(&mut gpioc.crh);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        //led.set_high();
        led1.set_low();
        hprintln!("Led is low");
        delay.delay_ms(2_000_u16);
        led1.set_high();
        hprintln!("LED is high");
        //led.set_low();
        delay.delay_ms(3_000_u16);
    }
}

// #![deny(unsafe_code)]
// #![no_std]
// #![no_main]

// use panic_halt as _;

// use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
// use stm32f1xx_hal::{
//     delay::Delay,
//     gpio,
//     gpio::{Floating, Input},
//     pac,
//     prelude::*,
// };

// use dht_hal_drv::{dht_read, dht_split_init, dht_split_read, DhtError, DhtType, DhtValue};
// use embedded_hal::digital::v2::OutputPin;

// // Define types for DHT interface
// type DhtHwPin = gpio::gpiob::PB9<Input<Floating>>;
// type DhtHwPinCr = gpio::gpiob::CRH;

// #[entry]
// fn main() -> ! {
//     // Get access to the core peripherals from the cortex-m crate
//     let cp = cortex_m::Peripherals::take().unwrap();
//     // Get access to the device specific peripherals from the peripheral access crate
//     let dp = pac::Peripherals::take().unwrap();

//     // Take ownership over the raw flash and rcc devices and convert them into the corresponding
//     // HAL structs
//     let mut flash = dp.FLASH.constrain();
//     // dp.RCC.cfgr.sysclk(1.mhz());
//     let mut rcc = dp.RCC.constrain();

//     // Freeze the configuration of all the clocks in the system and store
//     // the frozen frequencies in `clocks`
//     let clocks = rcc.cfgr.freeze(&mut flash.acr);

//     // Acquire the GPIOC peripheral
//     let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

//     // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
//     // in order to configure the port. For pins 0-7, crl should be passed instead.
//     let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
//     // Configure the syst timer to trigger an update every second
//     // let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);
//     let mut delay = Delay::new(cp.SYST, clocks);

//     // DHT pin config
//     let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
//     //let mut dht_pin: DhtHwPin = gpiob.pb9.into_floating_input(&mut gpiob.crh);
//     //             let nss = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
//     //let mut dht_open_drain = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
//     let mut dht_open_drain = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);

//     loop {
//         // Open/Drain emulation
//         //let (readings, pout) = read_dht_splitted(dht_pin, &mut gpiob.crh, &mut delay);
//         //dht_pin = pout;

//         let readings = dht_read(DhtType::DHT11, &mut dht_open_drain, &mut |d| {
//             delay.delay_us(d)
//         });

//         match readings {
//             Ok(res) => {
//                 // Long blinks if everything is OK
//                 led_blink(&mut led, &mut delay, 250);
//                 hprintln!("DHT readins {} C {}%", res.temperature(), res.humidity());
//             }
//             Err(err) => {
//                 // Short blinks on errors
//                 for _ in 0..10 {
//                     led_blink(&mut led, &mut delay, 25);
//                 }
//                 hprintln!("DHT ERROR {:?}", err);
//             }
//         };
//         delay.delay_ms(1_000_u32);
//     }
// }

// /// Example of reading using open drain pin emulation
// fn read_dht_splitted(
//     pin: DhtHwPin,
//     cr: &mut DhtHwPinCr,
//     delay: &mut Delay,
// ) -> (Result<DhtValue, DhtError>, DhtHwPin) {
//     // Implement custom HW specific delay logic that DHT driver is not aware of
//     let mut delay_us = |d| delay.delay_us(d);
//     // Convert pin to output
//     let mut pin_out = pin.into_push_pull_output(cr);
//     // Initialize DHT data transfer
//     let init = dht_split_init(&mut pin_out, &mut delay_us);
//     if init.is_err() {
//         // You can skip this error check if you like
//         return (Err(init.err().unwrap()), pin_out.into_floating_input(cr));
//     }

//     // WARNING there should be no additional logic between dht_init and dht_read

//     // Should convert pin back to input
//     let mut pin_in = pin_out.into_floating_input(cr);
//     // Now let's read some data
//     let readings = dht_split_read(DhtType::DHT11, &mut pin_in, &mut delay_us);
//     // We must return reading + pin together
//     // because of tricky stm32f1xx_hal implementation
//     // where you can have only one pin instance at a time
//     (readings, pin_in)
// }

// fn led_blink<Error>(pin: &mut dyn OutputPin<Error = Error>, delay: &mut Delay, ms: u32) {
//     pin.set_high();
//     delay.delay_ms(ms);
//     pin.set_low();
//     delay.delay_ms(ms);
// }
