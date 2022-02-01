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
    let ram = Box::new(gameboy::RAM::<{ 8 * 1024 }>::create(0xC000));
    let vram = Box::new(gameboy::RAM::<{ 8 * 1024 }>::create(0x8000));
    let gpu = Box::new(gameboy::GPU::create(vram));
    let bus = Box::new(gameboy::Bus::create(ram, gpu));
    let mut cpu = gameboy::CPU::create(4194304, bus);

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
        tx.send(GUIData::VRAMBuf(vec![0x00])).unwrap();
    }

    tx.send(GUIData::Shutdown).unwrap();
}
