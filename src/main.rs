extern crate rppal;
extern crate rand;

use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;
use rand::Rng;

// The GPIO module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
const GPIO_LED: u8 = 23;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();
    gpio.set_mode(GPIO_LED, Mode::Output);

    let mut count = 0;
    let mut rng = rand::thread_rng();

    while count < 10 {
        let a = rng.gen_range(5,500);
        let b = rng.gen_range(5,500);
        // Blink an LED attached to the pin on and off
        gpio.write(GPIO_LED, Level::High);
        thread::sleep(Duration::from_millis(a));
        gpio.write(GPIO_LED, Level::Low);
        thread::sleep(Duration::from_millis(b));
        count += 1 // = count + 1
    }
}