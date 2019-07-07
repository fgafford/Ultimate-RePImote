use std::error::Error;
// use std::thread;

use rppal::gpio::{Gpio};
use rppal::gpio::Level::{High, Low};
use rppal::system::DeviceInfo;
use rppal::gpio::Level;

// The GPIO module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
const GPIO_IR_IN: u8 = 17; // Physical pin 11


pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Reading IR in on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()
                ?.get(GPIO_IR_IN)
                ?.into_input();

    while true {
        let lvl = pin.read() as u8;
        println!("{}", lvl);
    }
    
    // println!("Before Blocking");

    // pin.poll_interrupt(false, None);

    // println!("After Blocking");

    // fn input_handler(level: Level){
    //     println!("{}", level);
    // }

    // Blink the LED by setting the pin's logic level high for 500ms.
    loop { 
       || { 
            let level = pin.read();
            let out = match level {
                High => "High",
                Low => "Low",
            };
            println!("{}", out);
        };
    }

    Ok(())
}