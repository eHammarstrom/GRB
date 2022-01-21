use gamerboy::*;

// Gameboy EMU
fn main() {
    let mut ram = EmuRAM::<{ 8 * 1024 }>::create();
    let mut vram = EmuRAM::<{ 8 * 1024 }>::create();
    let mut gpu = EmuGPU::create(&mut vram);
    let bus = EmuBus::create(&mut ram, &mut gpu);
    let cpu = EmuCPU::create(&bus);
    /*
    let emulator_vram = EmuVRAM...;
    let emulator_irq_controller = EmuIRQController...;
    let emulator_gpu = EmuGPU...;
    let emulator_timer = EmuTIMER...;

    let emulator_bus = EmuBUS::create(ram, vram, irq_controller, timer, gpu);

    let emulator_cpu = EmuCPU::create(emulator_bus);

    let emulator_cartridge = EmuCartridge::from_path("pokemon.bin");

    emulator_bus.map_cartridge(emulator_cartridge);

    while (true) {
        let mut cycles = 0;

        // Process all IRQ requests
        while let Some(c) = emulator_cpu.interrupt() {
            // TODO: Figure out cycles for pushing PC, fetching vector from
            // IRQ table and loading into PC.
            // Some guy on the internet says this may not be an issue since
            // timing have to be screwed up "EXTREMELY bad" for games to not
            // work.
            cycles += c;

            emulator_bus.timer().progress(cycles);
            emulator_bus.gpu().progress(cycles);
        }

        // Continue executing instructions
        let cycles = emulator_cpu.step();

        emulator_bus.timer().progress(cycles);
        emulator_bus.gpu().progress(cycles);
    }
    */
}
