extern crate rppal;
extern crate rand;

use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Mode, Level};

const PIN: u8 = 17 // Phisical pin 11

fn main() {
    let gpio = Gpio::new()?.get(PIN).

    while true {
      let v = gpio.read() as u8;
      println!(v)
    }
}