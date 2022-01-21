use crate::cpu::CPU;
use crate::bus::Bus;

/// GameBoy CPU
pub struct EmuCPU<'a> {
    /// CPU bus
    bus: &'a dyn Bus<'a, Addr = u16, Data = u8>,
    /// CPU Registers
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    H: u8,
    L: u8,
    SP: u16,
    PC: u16,
}

impl EmuCPU<'_> {
}

impl<'a> CPU<'a, u16, u8> for EmuCPU<'a>
{
    fn create(
        bus: &'a dyn Bus<'a, Addr = u16, Data = u8>
    ) -> Self {
        EmuCPU {
            bus: bus,
            A: 0, B: 0, C: 0, D: 0, H: 0, L: 0,
            SP: 0,
            PC: 0,
        }
    }

    /// Executes the instruction at PC and returns cycles spent
    fn step(&mut self) -> usize {
        0
    }

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()> {
        Some(())
    }
}
