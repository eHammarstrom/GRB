use gamerboy::emulator_ram::EmuRAM;
use gamerboy::RAM;

fn main() {
    let emulator_ram = EmuRAM::<{ 8 * 1024 }>::create();
}
