use gamerboy::*;

// Gameboy EMU
fn main() {
    let mut ram = GameBoyRAM::<{ 8 * 1024 }>::create(0xC000, 0xDFFF);
    let mut vram = GameBoyRAM::<{ 8 * 1024 }>::create(0x8000, 0x9FFF);
    let mut gpu = GameBoyGPU::create(&mut vram);
    let bus = GameBoyBus::create(&mut ram, &mut gpu);
    let cpu = GameBoyCPU::create(&bus);
    /*
    let gameboy_vram = GameBoyVRAM...;
    let gameboy_irq_controller = GameBoyIRQController...;
    let gameboy_gpu = GameBoyGPU...;
    let gameboy_timer = GameBoyTIMER...;

    let gameboy_bus = GameBoyBUS::create(ram, vram, irq_controller, timer, gpu);

    let gameboy_cpu = GameBoyCPU::create(gameboy_bus);

    let gameboy_cartridge = GameBoyCartridge::from_path("pokemon.bin");

    gameboy_bus.map_cartridge(gameboy_cartridge);

    while (true) {
        let mut cycles = 0;

        // Process all IRQ requests
        while let Some(c) = gameboy_cpu.interrupt() {
            // TODO: Figure out cycles for pushing PC, fetching vector from
            // IRQ table and loading into PC.
            // Some guy on the internet says this may not be an issue since
            // timing have to be screwed up "EXTREMELY bad" for games to not
            // work.
            cycles += c;

            gameboy_bus.timer().progress(cycles);
            gameboy_bus.gpu().progress(cycles);
        }

        // Continue executing instructions
        let cycles = gameboy_cpu.step();

        gameboy_bus.timer().progress(cycles);
        gameboy_bus.gpu().progress(cycles);
    }
    */
}
