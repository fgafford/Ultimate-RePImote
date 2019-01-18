# Ultimate Re-Pi-mote

## WIP: this is a Work In Progress

## Goal
The goal here is to make an ultimate remote for the many many IR devices all the the same room. The goal is to use a `Raspberry-pi Zero W` to expose an web interface that will send IR signals to control numerious IR devices: 
- TV
- LED backlight strips
- Soundbar
- DVD / Blueray play
- ... the list goes on ...

-----


## Setup
There is a bit of setup that is required here to get this all running. Here are the steps needed to get started:

### 1. Install Rust
`curl https://sh.rustup.rs -sSf | sh`

### 2. Setup the Raspberry-pi toolchain
For this project I used a Raspberry-Pi Zero W. The Pi Zero has an ARMv6 architecture. There is an issue with Ubuntu compiling to ARMv7 even when using `arm-unknow-linux-gnueabihf`. In order to correct this you will need to setup the raspberry pi toolchain on the compiling machine.

A good overview of how to do this can be found here:
https://github.com/tiziano88/rust-raspberry-pi/blob/master/Dockerfile

### 3. Setup the linker
You will need to define the linker for Rust in `~/.cargo/config` as follows:
```toml
[target.arm-unknown-linux-gnueabihf]
linker = "[path to installed pi-tools]/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc-4.8.3"
```
Notice the `-x64`. ([credit](https://github.com/rust-lang/rust/issues/37420))

## Running the project
See `run.sh`. Modify it to suit your needs of compiling the code locally, movivg it to the PI, and running it. 


### Helpful resources:
https://www.instructables.com/id/How-To-Useemulate-remotes-with-Arduino-and-Raspber/  
https://www.raspberrypi.org/forums/viewtopic.php?t=79978 
