use std::error::Error;
// use std::thread;

mod record;

// The GPIO module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
const GPIO_IR_IN: u8 = 17; // Physical pin 11


fn main() -> Result<(), Box<dyn Error>> {

    record::run();

    Ok(())

}