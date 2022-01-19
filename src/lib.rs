mod cpu;
mod bus;
mod gpu;
mod ram;
mod addressable;
mod timed;

pub mod emulator_ram;
pub mod emulator_gpu;

pub use addressable::Addressable;
pub use ram::RAM;

pub fn test() {
    println!("hehe");
}
