use crate::addressable::*;
use crate::bus::{Bus, CopyOf};
use crate::cpu;
use crate::cpu::CPUError;
use crate::cpu::Word;
use crate::gameboy_cpu_inst::*;
use crate::timed::*;

#[derive(Clone, Copy, Debug)]
pub enum Reg {
    /// 8-bit registers
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    /// 16-bit registers (some reference the concatenation of two 8-bit registers)
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

impl Reg {
    // FIXME: Create matrix (https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) instead
    fn src_from_operand(oper: u8) -> Self {
        use Reg::*;

        let lo = oper & 0xF;
        let regs: [Reg; 16] = [B, C, D, E, H, L, HL, A, B, C, D, E, H, L, HL, A];
        regs[lo as usize]
    }
}

/// GameBoy CPU
#[derive(Debug)]
pub struct CPU<'a> {
    /// CPU bus
    bus: &'a mut dyn Bus<'a, Addr = u16, Data = u8>,
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

impl<'a> CPU<'a> {
    pub fn get_vram(&self) -> Vec<u8> {
        self.bus.copy_of(CopyOf::VRAM)
    }

    fn get_reg_byte(&mut self, reg: Reg) -> Result<u8, CPUError<'a, Self>> {
        match reg {
            Reg::A => Ok(self.AF.get_high()),
            Reg::B => Ok(self.BC.get_high()),
            Reg::C => Ok(self.BC.get_low()),
            Reg::D => Ok(self.DE.get_high()),
            Reg::E => Ok(self.DE.get_low()),
            Reg::F => Ok(self.AF.get_low()),
            Reg::H => Ok(self.HL.get_high()),
            Reg::L => Ok(self.HL.get_low()),
            _ => Err(CPUError::BadRegisterAccess(
                "Tried reading Byte from Word reg",
            )),
        }
    }

    fn get_reg_word(&mut self, reg: Reg) -> Result<u16, CPUError<'a, Self>> {
        match reg {
            Reg::AF => Ok(self.AF.into()),
            Reg::BC => Ok(self.BC.into()),
            Reg::DE => Ok(self.BC.into()),
            Reg::HL => Ok(self.HL.into()),
            Reg::SP => Ok(self.SP.into()),
            Reg::PC => Ok(self.PC.into()),
            _ => Err(CPUError::BadRegisterAccess(
                "Tried reading Word from Byte reg",
            )),
        }
    }

    fn set_reg_byte(&mut self, reg: Reg, val: u8) -> Result<(), CPUError<'a, Self>> {
        match reg {
            Reg::A => Ok(self.AF.set_high(val)),
            Reg::B => Ok(self.BC.set_high(val)),
            Reg::C => Ok(self.BC.set_low(val)),
            Reg::D => Ok(self.DE.set_high(val)),
            Reg::E => Ok(self.DE.set_low(val)),
            Reg::F => Ok(self.AF.set_low(val)),
            Reg::H => Ok(self.HL.set_high(val)),
            Reg::L => Ok(self.HL.set_low(val)),
            _ => Err(CPUError::BadRegisterAccess(
                "Mismatching register {reg} and value {val} width",
            )),
        }
    }

    fn set_reg_word(&mut self, reg: Reg, val: u16) -> Result<(), CPUError<'a, Self>> {
        match reg {
            Reg::AF => Ok(self.AF = val.into()),
            Reg::BC => Ok(self.BC = val.into()),
            Reg::DE => Ok(self.BC = val.into()),
            Reg::HL => Ok(self.HL = val.into()),
            Reg::SP => Ok(self.SP = val.into()),
            Reg::PC => Ok(self.PC = val.into()),
            _ => Err(CPUError::BadRegisterAccess(
                "Mismatching register {reg} and value {val} width",
            )),
        }
    }

    fn add_byte(&mut self, dst_reg: Reg, val: u8) -> Result<(), CPUError<'a, Self>> {
        let reg_val = self.get_reg_byte(dst_reg)?;
        self.set_reg_byte(dst_reg, reg_val + val)
    }

    fn add_word(&mut self, dst_reg: Reg, val: u16) -> Result<(), CPUError<'a, Self>> {
        let reg_val = self.get_reg_word(dst_reg)?;
        self.set_reg_word(dst_reg, reg_val + val)
    }
}

impl<'a> cpu::CPU<'a> for CPU<'a> {
    type Addr = u16;
    type Data = u8;

    fn create(clock: u32, bus: &'a mut dyn Bus<'a, Addr = u16, Data = u8>) -> Self {
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
    fn step(&mut self) -> Result<u32, CPUError<'a, Self>> {
        let opcode: u8 = self.bus.read_byte(self.PC.into())?;
        let instruction = INSTRUCTION_LOOKUP[opcode as usize];

        match instruction.opcode {
            Opcode::Invalid => return Err(AddressError::IllegalInstr(self.PC.into()).into()),
            Opcode::NOP => unimplemented!(),
            Opcode::LD => unimplemented!(),
            Opcode::ADD => {
                let dst_reg = match instruction.dst {
                    Operand::Value(r) => r,
                    _ => return Err(AddressError::IllegalInstr(self.PC.into()).into()),
                };
                let src_val = match instruction.src {
                    Operand::Value(r) => self.get_reg_byte(r)?,
                    _ => unimplemented!(),
                };
                self.add_byte(dst_reg, src_val)?;
            },
            Opcode::ADC => unimplemented!(),
            Opcode::INC => unimplemented!(),
            Opcode::DEC => unimplemented!(),
            Opcode::RLCA => unimplemented!(),
            Opcode::RRA => unimplemented!(),
            Opcode::JR => unimplemented!(),
            Opcode::RRCA => unimplemented!(),
            Opcode::STOP => unimplemented!(),
            Opcode::RLA => unimplemented!(),
            Opcode::LDI => unimplemented!(),
            Opcode::DAA => unimplemented!(),
            Opcode::CPL => unimplemented!(),
            Opcode::LDD => unimplemented!(),
            Opcode::SCF => unimplemented!(),
            Opcode::CCF => unimplemented!(),
            Opcode::HALT => unimplemented!(),
            Opcode::SUB => unimplemented!(),
            Opcode::SBC => unimplemented!(),
            Opcode::AND => unimplemented!(),
            Opcode::XOR => unimplemented!(),
            Opcode::OR => unimplemented!(),
            Opcode::CP => unimplemented!(),
            Opcode::RET => unimplemented!(),
            Opcode::POP => unimplemented!(),
            Opcode::JP => unimplemented!(),
            Opcode::CALL => unimplemented!(),
            Opcode::PUSH => unimplemented!(),
            Opcode::RST => unimplemented!(),
            Opcode::PREFIX => unimplemented!(),
            Opcode::RETI => unimplemented!(),
            Opcode::LDH => unimplemented!(),
            Opcode::DI => unimplemented!(),
            Opcode::LDHL => unimplemented!(),
            Opcode::EI => unimplemented!(),
        }

        self.bus.catchup(CycleTime::new(self.frequency(), instruction.cycles));

        Ok(instruction.cycles)
    }

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()> {
        Some(())
    }

    fn frequency(&self) -> u32 {
        self.clock
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
        bus.write_byte(RAM_START.into(), AB_ADD)
            .expect("AB addition to be written to RAM");
        let mut cpu: gameboy_cpu::CPU = CPU::create(4194304, &mut bus);

        cpu.PC = RAM_START.into();
        cpu.BC.set_high(10);

        cpu.step().expect("CPU to step once");

        assert_eq!(cpu.AF.get_high(), 10);
    }
}
