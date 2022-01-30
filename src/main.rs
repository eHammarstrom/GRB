use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use gamerboy::*;

enum GUIData {
    VRAMBuf(Vec<u8>),
    Shutdown,
}

// Gameboy EMU
fn main() {
    let mut ram = gameboy::RAM::<{ 8 * 1024 }>::create(0xC000);
    let mut vram = gameboy::RAM::<{ 8 * 1024 }>::create(0x8000);
    let mut gpu = gameboy::GPU::create(&mut vram);
    let mut bus = gameboy::Bus::create(&mut ram, &mut gpu);
    let mut cpu = gameboy::CPU::create(4194304, &mut bus);

    let (tx, rx): (Sender<GUIData>, Receiver<GUIData>) = mpsc::channel();

    thread::spawn(move || loop {
        let data = rx.recv().unwrap();
        let vram = match data {
            GUIData::Shutdown => return,
            GUIData::VRAMBuf(vram) => vram,
        };

        dbg!(vram);
    });

    loop {
        // TODO: Handle interrupts before stepping

        let cycles = match cpu.step() {
            Err(_) => break,
            Ok(c) => c,
        };
        dbg!(cycles);

        // Send vram to GUI thread
        let vram = cpu.get_vram();
        tx.send(GUIData::VRAMBuf(vram)).unwrap();
    }

    tx.send(GUIData::Shutdown).unwrap();
}
