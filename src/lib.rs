mod cpu;
mod bus;
mod ram;
mod addressable;

pub mod emulator_ram;

pub use addressable::Addressable;
pub use ram::RAM;

pub fn test() {
    println!("hehe");
}
