use std::error::Error;
use std::time::Instant;
// use std::thread'

use rppal::gpio::{Gpio};
use rppal::gpio::Level;
use rppal::gpio::Level::{Low};
use rppal::system::DeviceInfo;

// The GPIO module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
// const GPIO_IR_IN: u8 = 17; // Physical pin 11
// const GPIO_LED: u8 = 23; // Physical pin 16
const GPIO_IR_IN: u8 = 18; // Physical pin 12
const GPIO_LED: u8 = 24; // Physical pin 18

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Reading IR in on a {}.", DeviceInfo::new()?.model());

    let mut toggels: Vec<Instant> = vec![];

    let pin = Gpio::new()
                ?.get(GPIO_IR_IN)
                ?.into_input();

    let mut prev_read : Level = pin.read();

    let mut led = Gpio::new()
                ?.get(GPIO_LED)
                ?.into_output();



    while true {
      // For IR recievers: 
      //  1 is not recieving
      //  0 is for recieving 
      let lvl = pin.read();
      if lvl != prev_read {
        toggels.push(Instant::now());
        // toggle : set opposite of IR sensor
        // toggle_led(lvl == Low)
        if lvl == Low {
          led.set_high();
        } else {
          led.set_low();
        }
      println!("Times: {:?}", toggels);
      }
      prev_read = lvl;

    }


    Ok(())
}