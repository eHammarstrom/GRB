use crate::cpu;
use crate::cpu::Word;
use crate::bus::Bus;
use crate::addressable::*;

use std::fmt;

#[derive(Debug)]
enum InstructionError {
    IllegalOperand(&'static str),
}

impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for InstructionError {}

#[derive(Debug)]
enum CPUError {
    BadRegisterWrite(&'static str),
}

#[derive(Clone, Copy, Debug)]
enum CPUOperation {
    NOP,
    STOP,
    LD,
    INC,
    DEC,
    // FIXME: To be continued
}

#[derive(Clone, Copy, Debug)]
enum CPURegister {
    /// 8-bit registers
    A, B, C, D, E, F, H, L,
    /// 16-bit registers (some reference the concatenation of two 8-bit registers)
    AF, BC, DE, HL, PC, SP,
}

impl CPURegister {
    // FIXME: Create matrix (https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) instead
    fn src_from_operand(oper: u8) -> Self {
        use CPURegister::*;

        let lo = oper & 0xF;
        let regs: [CPURegister; 16] = [ B, C, D, E, H, L, HL, A, B, C, D, E, H, L, HL, A ];
        regs[lo as usize]
    }
}

/// GameBoy CPU
pub struct CPU<'a> {
    /// CPU bus
    bus: &'a dyn Bus<'a, Addr = u16, Data = u8>,
    /// MSB = A, LSB = Flags
    AF: Word,
    /// MSB = B, LSB = C
    BC: Word,
    /// MSB = D, LSB = E
    DE: Word,
    /// MSB = H, LSB = L
    HL: Word,
    /// Stack pointer address
    SP: Word,
    /// Program counter
    PC: Word,
    /// CPU clock speed in Hz
    clock: u32,
}

impl CPU<'_> {
    fn get_reg_byte(&mut self, reg: CPURegister) -> Result<u8, CPUError> {
        match reg {
            CPURegister::A => Ok(self.AF.get_high()),
            CPURegister::B => Ok(self.BC.get_high()),
            CPURegister::C => Ok(self.BC.get_low()),
            CPURegister::D => Ok(self.DE.get_high()),
            CPURegister::E => Ok(self.DE.get_low()),
            CPURegister::F => Ok(self.AF.get_low()),
            CPURegister::H => Ok(self.HL.get_high()),
            CPURegister::L => Ok(self.HL.get_low()),
            _ => Err(CPUError::BadRegisterWrite("Tried reading Byte from Word reg")),
        }
    }

    fn get_reg_word(&mut self, reg: CPURegister) -> Result<u16, CPUError> {
        match reg {
            CPURegister::AF => Ok(self.AF.into()),
            CPURegister::BC => Ok(self.BC.into()),
            CPURegister::DE => Ok(self.BC.into()),
            CPURegister::HL => Ok(self.HL.into()),
            CPURegister::SP => Ok(self.SP.into()),
            CPURegister::PC => Ok(self.PC.into()),
            _ => Err(CPUError::BadRegisterWrite("Tried reading Word from Byte reg")),
        }
    }

    fn set_reg_byte(&mut self, reg: CPURegister, val: u8) -> Result<(), CPUError> {
        match reg {
            CPURegister::A => Ok(self.AF.set_high(val)),
            CPURegister::B => Ok(self.BC.set_high(val)),
            CPURegister::C => Ok(self.BC.set_low(val)),
            CPURegister::D => Ok(self.DE.set_high(val)),
            CPURegister::E => Ok(self.DE.set_low(val)),
            CPURegister::F => Ok(self.AF.set_low(val)),
            CPURegister::H => Ok(self.HL.set_high(val)),
            CPURegister::L => Ok(self.HL.set_low(val)),
            _ => Err(CPUError::BadRegisterWrite("Mismatching register {reg} and value {val} width")),
        }
    }

    fn set_reg_word(&mut self, reg: CPURegister, val: u16) -> Result<(), CPUError> {
        match reg {
            CPURegister::AF => Ok(self.AF = val.into()),
            CPURegister::BC => Ok(self.BC = val.into()),
            CPURegister::DE => Ok(self.BC = val.into()),
            CPURegister::HL => Ok(self.HL = val.into()),
            CPURegister::SP => Ok(self.SP = val.into()),
            CPURegister::PC => Ok(self.PC = val.into()),
            _ => Err(CPUError::BadRegisterWrite("Mismatching register {reg} and value {val} width")),
        }
    }

    fn add_byte(&mut self, dst_reg: CPURegister, val: u8) -> Result<(), CPUError>{
        let reg_val = self.get_reg_byte(dst_reg)?;
        self.set_reg_byte(dst_reg, reg_val + val)
    }

    fn add_word(&mut self, dst_reg: CPURegister, val: u16) -> Result<(), CPUError>{
        let reg_val = self.get_reg_word(dst_reg)?;
        self.set_reg_word(dst_reg, reg_val + val)
    }
}

impl<'a> cpu::CPU<'a> for CPU<'a>
{
    type Addr = u16;
    type Data = u8;

    fn create(
        clock: u32,
        bus: &'a dyn Bus<'a, Addr = u16, Data = u8>
    ) -> Self {
        CPU {
            bus: bus,
            AF: Word::default(),
            BC: Word::default(),
            DE: Word::default(),
            HL: Word::default(),
            SP: Word::default(),
            PC: Word::default(),
            clock,
        }
    }

    /// Executes the instruction at PC and returns cycles spent
    fn step(&mut self) -> Result<usize, AddressError<Self::Addr>> {
        const ILLEGAL_INSTR: [u8; 11] = [
            0xD3, 0xE3, 0xE4, 0xF4, 0xDB, 0xEB, 0xEC, 0xFC, 0xDD, 0xED, 0xFD,
        ];
        let opcode = self.bus.read_byte(self.PC.into())?;

        if opcode & 0x80 == 0x80 { // ADD8
            let dst_reg = CPURegister::A;
            let src_reg = CPURegister::src_from_operand(opcode);
            // FIXME: unsafe unwrap
            let src_val = self.get_reg_byte(src_reg).unwrap();
            self.add_byte(dst_reg, src_val).unwrap()
        } else if opcode & 0x09 == 0x09 { // ADD16
        }

        // It's a rotation/shifting instruction
        if opcode == 0xCB {
            let opcode_contd = self.bus.read_byte(self.PC.into())?;
        }

        // Illegal instructions
        if ILLEGAL_INSTR.contains(&opcode) {
            return Err(AddressError::IllegalInstr(self.PC.into()))
        }

        self.PC += 1.into();

        Ok(0)
    }

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()> {
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_cpu_add8() {
        const RAM_START: u16 = 0xC000;
        const AB_ADD: u8 = 0x80;

        let mut ram = gameboy::RAM::<{ 8 * 1024 }>::create(RAM_START);
        let mut vram = gameboy::RAM::<{ 8 * 1024 }>::create(0x8000);
        let mut gpu = gameboy::GPU::create(&mut vram);
        let mut bus = gameboy::Bus::create(&mut ram, &mut gpu);
        bus.write_byte(RAM_START.into(), AB_ADD).expect("AB addition to be written to RAM");
        let mut cpu: gameboy_cpu::CPU = CPU::create(4194304, &bus);

        cpu.PC = RAM_START.into();
        cpu.BC.set_high(10);

        cpu.step().expect("CPU to step once");

        assert_eq!(cpu.AF.get_high(), 10);
    }
}
