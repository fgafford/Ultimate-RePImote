use std::error::Error;
// use std::thread;
// use std::time::Duration;

use rppal::gpio::{Gpio};
use rppal::gpio::Level::{High, Low};
use rppal::system::DeviceInfo;
// use rand::Rng;

// The GPIO module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
// const GPIO_LED: u8 = 23;
const GPIO_IR_IN: u8 = 17;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading IR in on a {}.", DeviceInfo::new()?.model());

    let pin = Gpio::new()?.get(GPIO_IR_IN)?.into_input();

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
        println!("---")
    }

    // let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // let mut count = 0;
    // let mut rng = rand::thread_rng();

    // // Blink the LED by setting the pin's logic level high for 500ms.
    // while count < 30 {
    //     let a = rng.gen_range(5,500);
    //     let b = rng.gen_range(5,500);
    //     // Blink an LED attached to the pin on and off
    //     pin.set_high();
    //     thread::sleep(Duration::from_millis(a));
    //     pin.set_low();
    //     thread::sleep(Duration::from_millis(b));
    //     count += 1 // = count + 1
    // }

    // Ok(())
}