// Due to the naming of some registers we allow non_snake_case
#![allow(non_snake_case)]

use crate::addressable::*;
use crate::bus::{Bus};
use crate::cpu;
use crate::cpu::CPUError;
use crate::cpu::Word;
use crate::gameboy_cpu_inst::*;
use crate::timed::*;

use std::{cmp, ops};

use either::*;

trait EitherIntoWordExt {
    fn into_word(self) -> u16;
}

impl EitherIntoWordExt for Either<u8, u16> {
    fn into_word(self) -> u16 {
        match self {
            Either::Left(byte) => byte.into(),
            Either::Right(word) => word,
        }
    }
}

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

/// CPU Flag register flags
enum Flag {
    /// Z (Zero) is set if the result of an operation is 0
    Z,
    /// C or Cy (Carry) is set if one of the following cases occur,
    /// * 8-bit addition overflow
    /// * 16-bit addition overflow
    /// * Subtraction or comparison is less than 0
    /// * Rotate or Shift pushes out a "1" bit
    C,
    /// N is set if the last operation was a subtraction
    N,
    /// H (Half-Carry) is set if a 8-bit addition 4-bit overflows
    H,
}

impl Flag {
    const fn bit(self) -> u8 {
        match self {
            Self::Z => 7,
            Self::C => 6,
            Self::N => 5,
            Self::H => 4,
        }
    }

    const fn mask(self) -> u8 {
        1 << self.bit()
    }
}

/// GameBoy CPU
#[derive(Debug)]
pub struct CPU {
    /// CPU bus
    bus: Box<dyn Bus<Addr = u16, Data = u8>>,
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

fn check_overflow<T>(dst: T, src: T, overflow_mask: T) -> bool
where
    T: cmp::PartialOrd + ops::Sub<Output = T>,
{
    dst <= overflow_mask && src >= overflow_mask
        || dst <= overflow_mask && dst > overflow_mask - src
}

impl CPU {
    // According to http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
    const U4MAX: u8 = 0xF; // 4-bit overflow limit (8-bit Half-Carry)
    const U8MAX: u8 = u8::MAX; // 8-bit overflow limit (8-bit Carry)
    const U12MAX: u16 = 0x0FFF; // 12-bit overflow limit (16-bit Half-Carry)
    const U16MAX: u16 = u16::MAX; // 16-bit overflow limit (16-bit Carry)

    pub fn bus_apply<FUN>(&mut self, f: FUN)
    where
        FUN: Fn(&mut dyn Bus<Addr = <CPU as cpu::CPU>::Addr, Data = <CPU as cpu::CPU>::Data>),
    {
        f(&mut *self.bus)
    }

    fn get_reg_byte(&self, reg: Reg) -> Result<u8, CPUError<Self>> {
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

    fn get_reg_word(&self, reg: Reg) -> Result<u16, CPUError<Self>> {
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

    fn set_reg_byte(&mut self, reg: Reg, val: u8) -> Result<(), CPUError<Self>> {
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

    fn set_reg_word(&mut self, reg: Reg, val: u16) -> Result<(), CPUError<Self>> {
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

    /// Retrieve either a Byte or a Word from a Reg
    fn get_reg_value(&self, r: Reg) -> Either<u8, u16> {
        match (self.get_reg_byte(r), self.get_reg_word(r)) {
            (Ok(byte), _) => Either::Left(byte),
            (_, Ok(word)) => Either::Right(word),
            _ => panic!("Tried to retrieve value from bad CPU register"),
        }
    }

    /// Expect operand to resolve to a Reg
    fn operand_to_reg(&self, oper: Operand) -> Result<Reg, CPUError<Self>> {
        match oper {
            Operand::Value(r) => Ok(r),
            _ => Err(CPUError::BadRegisterAccess(
                "Failed to retrieve Reg from Operand {}",
            )),
        }
    }

    /// Resolve operand to either a Byte or a Word given the current PC
    /// PC is needed if the instruction spans several bytes
    fn operand_to_value(
        &self,
        oper: Operand,
        instr_pc: Word,
    ) -> Result<Either<u8, u16>, CPUError<Self>> {
        match oper {
            Operand::Value(r) => Ok(self.get_reg_value(r)),
            Operand::DerefReg(r) => {
                let addr = match self.get_reg_value(r) {
                    Either::Left(byte) => byte as u16,
                    Either::Right(word) => word,
                };

                Ok(Either::Left(self.bus.read_byte(addr)?))
            }
            Operand::Imm8 => {
                let imm_addr = u16::from(instr_pc + Word::from(1u8));
                Ok(Either::Left(self.bus.read_byte(imm_addr)?))
            }
            Operand::DerefImm8 => {
                // NOTE: DerefImm8 is a special case of an Imm8 that is always
                // offset by 0xFF00, i.e. the Imm8 will take the place of the
                // lower byte of 0xFF00.
                let fixed_offset: u16 = 0xFF00;
                let derefimm8_addr = u16::from(instr_pc + Word::from(1u8));
                let derefimm8 = self.bus.read_byte(derefimm8_addr)? as u16;
                Ok(Either::Left(self.bus.read_byte(fixed_offset | derefimm8)?))
            }
            Operand::Imm16 => {
                let imm_addr_upper = u16::from(instr_pc + Word::from(1u8));
                let imm_addr_lower = u16::from(instr_pc + Word::from(2u8));
                let upper_byte = self.bus.read_byte(imm_addr_upper)? as u16;
                let lower_byte = self.bus.read_byte(imm_addr_lower)? as u16;
                Ok(Either::Right(upper_byte << 8 | lower_byte))
            }
            _ => Err(CPUError::BadRegisterAccess(
                "Failed to retrieve value from operand register: {r}",
            )),
        }
    }

    /// Expect operand to resolve to a Byte
    fn operand_to_byte(&self, oper: Operand, instr_pc: Word) -> Result<u8, CPUError<Self>> {
        self.operand_to_value(oper, instr_pc)?
            .left()
            .ok_or(CPUError::AddrErr(AddressError::IllegalInstr(
                self.PC.into(),
            )))
    }

    /// Expect operand to resolve to a Word
    fn operand_to_word(&self, oper: Operand, instr_pc: Word) -> Result<u16, CPUError<Self>> {
        self.operand_to_value(oper, instr_pc)?
            .right()
            .ok_or(CPUError::AddrErr(AddressError::IllegalInstr(
                self.PC.into(),
            )))
    }
}

impl cpu::CPU for CPU {
    type Addr = u16;
    type Data = u8;

    fn create(clock: u32, bus: Box<dyn Bus<Addr = u16, Data = u8>>) -> Self {
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
    fn step(&mut self) -> Result<u32, CPUError<Self>> {
        let opcode: u8 = self.bus.read_byte(self.PC.into())?;
        let instruction: Instr = INSTRUCTION_LOOKUP[opcode as usize];

        match instruction.opcode {
            Opcode::Invalid => return Err(AddressError::IllegalInstr(self.PC.into()).into()),
            Opcode::NOP => self.PC += Word::from(1u8),
            Opcode::LD => {
                // First we retrieve the value we need to store
                let src_val: Self::Data = self.operand_to_value(instruction.src, self.PC)?
                    .expect_left("LD to only fetch a single byte");

                // LD either loads into Reg or Addr
                match instruction.dst {
                    Operand::Value(r) => {
                        self.set_reg_byte(r, src_val)?;
                    }
                    _ => {
                        let dst_addr: Self::Addr = self.operand_to_value(instruction.dst, self.PC)?.into_word();
                        self.bus.write_byte(dst_addr, src_val)?;
                    }
                }

                self.PC += Word::from(instruction.width);
            },
            Opcode::ADD => {
                // We expect DST to be a Reg since there is no ADD instruction
                // with anything other than a Reg as the dst
                let dst_reg = self
                    .operand_to_reg(instruction.dst)
                    .expect("ADD dst operand to be a register");

                // Clear carries
                self.AF.unset_bit(Flag::H.bit());
                self.AF.unset_bit(Flag::C.bit());
                // Always clear SUB flag if addition was performed
                self.AF.unset_bit(Flag::N.bit());

                match self.operand_to_value(instruction.dst, self.PC)? {
                    Either::Left(dst_byte) => {
                        let src_val = self.operand_to_byte(instruction.src, self.PC)?;

                        if check_overflow(dst_byte, src_val, CPU::U4MAX) {
                            self.AF.set_bit(Flag::H.bit());
                        }
                        if check_overflow(dst_byte, src_val, CPU::U8MAX) {
                            self.AF.set_bit(Flag::C.bit());
                        }

                        self.set_reg_byte(dst_reg, dst_byte.wrapping_add(src_val))?;
                    }
                    Either::Right(dst_word) => {
                        let src_val = self.operand_to_word(instruction.src, self.PC)?;

                        if check_overflow(dst_word, src_val, CPU::U12MAX) {
                            self.AF.set_bit(Flag::H.bit());
                        }
                        if check_overflow(dst_word, src_val, CPU::U16MAX) {
                            self.AF.set_bit(Flag::C.bit());
                        }

                        self.set_reg_word(dst_reg, dst_word.wrapping_add(src_val))?;
                    }
                }

                // Check if result was Zero
                if self.get_reg_value(dst_reg).into_word() == 0 {
                    self.AF.set_bit(Flag::Z.bit())
                }

                self.PC += Word::from(instruction.width);
            }
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

        self.bus
            .catchup(CycleTime::new(self.frequency(), instruction.cycles));

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
    use gameboy_cpu::Flag;

    const RAM_START: u16 = 0xC000;
    const RAM_SIZE: usize = 8 * 1024;
    const VRAM_START: u16 = 0x8000;
    const VRAM_SIZE: usize = 8 * 1024;

    fn setup_gameboy(pc: u16) -> gameboy_cpu::CPU {
        let ram = Box::new(gameboy::RAM::<RAM_SIZE>::create(RAM_START));
        let vram = Box::new(gameboy::RAM::<VRAM_SIZE>::create(VRAM_START));
        let gpu = Box::new(gameboy::GPU::create(vram));
        let bus = Box::new(gameboy::Bus::create(ram, gpu));
        let mut cpu = gameboy::CPU::create(4194304, bus);
        cpu.PC = pc.into();
        cpu
    }

    #[test]
    fn test_cpu_ADD() {
        let mut cpu = setup_gameboy(RAM_START);

        cpu.bus_apply(|bus| {
            const AB_ADD: u8 = 0x80;
            const AIMM8_ADD: u8 = 0xC6;
            const HLBC_ADD: u8 = 0x09;
            const LD_B_IMM8: u8 = 0x06;

            // Let the first addition be A += Imm8(10)
            bus.write_byte(RAM_START, AIMM8_ADD)
                .expect("AImm8 addition to be written to RAM");
            bus.write_byte(RAM_START + 1, 10)
                .expect("Imm8 value to be written to RAM");
            // LD 10 into B to prepare for some additions
            bus.write_byte(RAM_START + 2, LD_B_IMM8)
                .expect("LD B Imm8 operation to be written to RAM");
            bus.write_byte(RAM_START + 3, 10)
                .expect("LD B Imm8 value (10) to be written to RAM");
            // Let the following 2 additions be A += B
            bus.write_byte(RAM_START + 4, AB_ADD)
                .expect("AB addition to be written to RAM");
            bus.write_byte(RAM_START + 5, AB_ADD)
                .expect("AB addition to be written to RAM");
            // Let the following 3 additions be HL += BC
            bus.write_byte(RAM_START + 6, HLBC_ADD)
                .expect("HLBC addition to be written to RAM");
            bus.write_byte(RAM_START + 7, HLBC_ADD)
                .expect("HLBC addition to be written to RAM");
            bus.write_byte(RAM_START + 8, HLBC_ADD)
                .expect("HLBC addition to be written to RAM");
        });

        // Set the "Subtraction flag before we perform the addition
        cpu.AF.set_low(Flag::N.mask());

        // Assert that we have set the subtraction flag N
        assert_eq!(cpu.AF.get_low() & Flag::N.mask(), Flag::N.mask());

        cpu.step().expect("CPU to step once");

        // Assert our addition into register A (0) from B (10)
        assert_eq!(cpu.AF.get_high(), 10);
        // Assert that the subtraction flag has been reset after addition
        assert_ne!(cpu.AF.get_low() & Flag::N.mask(), Flag::N.mask());

        // Assert that PC has moved after Imm8 addition (which is 2 bytes wide)
        assert_eq!(u16::from(cpu.PC), RAM_START + 2);

        // Assert that we did not 4-bit overflow by checking flag H
        assert_ne!(cpu.AF.get_low() & Flag::H.mask(), Flag::H.mask());

        // LD: B <- 10
        cpu.step().expect("CPU to step twice");

        // Check that we loaded 10 into B and moved 2 steps (width of the LD Imm8 instr)
        assert_eq!(cpu.BC.get_high(), 10);
        assert_eq!(u16::from(cpu.PC), RAM_START + 4);

        cpu.step().expect("CPU to step twice");

        // Assert that we did 4-bit overflow by checking flag H
        assert_eq!(cpu.AF.get_low() & Flag::H.mask(), Flag::H.mask());
        // Assert our second addition into register A (10) from B (10)
        assert_eq!(cpu.AF.get_high(), 20);
        // Assert that we did NOT 8-bit overflow by checking flag C
        assert_ne!(cpu.AF.get_low() & Flag::C.mask(), Flag::C.mask());

        cpu.BC.set_high(236); // FIXME: Use LD
        cpu.step().expect("CPU to step thrice");

        // Assert our third addition into register A (20) from B (236)
        assert_eq!(cpu.AF.get_high(), 0);
        // Assert that we did an 8-bit overflow by checking flag C
        assert_eq!(cpu.AF.get_low() & Flag::C.mask(), Flag::C.mask());
        // Assert that we did hit Zero by checking flag Z
        assert_eq!(cpu.AF.get_low() & Flag::Z.mask(), Flag::Z.mask());

        // Initial BC state
        cpu.BC = Word::from(0x1FFFu16);
        // Initial HL state
        assert_eq!(cpu.HL, Word::nil());
        // Reset flags
        cpu.AF = Word::nil();

        cpu.step().expect("CPU to step");

        // Assert that HLBC addition succeeded
        assert_eq!(cpu.HL, Word::from(0x1FFFu16));
        // Assert that 16-bit half-carry was set
        assert_eq!(cpu.AF.get_low() & Flag::H.mask(), Flag::H.mask());

        cpu.step().expect("CPU to step");

        // Assert that HLBC addition succeeded
        assert_eq!(cpu.HL, Word::from(0x3FFEu16));
        // Check that we do not get the half-carry bit again (already carried)
        assert_ne!(cpu.AF.get_low() & Flag::H.mask(), Flag::H.mask());

        // Force an overflow resulting in carry bit set
        cpu.BC = Word::from(0xFFFFu16);

        cpu.step().expect("CPU to step");

        // Assert that HLBC overflow addition succeeded
        assert_eq!(cpu.HL, Word::from(0x3FFDu16));
        // Check that we do not get the half-carry bit again (already carried)
        assert_ne!(cpu.AF.get_low() & Flag::H.mask(), Flag::H.mask());
        // Check that we got the overflow
        assert_eq!(cpu.AF.get_low() & Flag::C.mask(), Flag::C.mask());
    }

    #[test]
    fn test_cpu_NOP() {
        let mut cpu = setup_gameboy(RAM_START);

        // Since RAM is zeroed we should be able to step through NOP instructions
        let prev_pc = cpu.PC;
        cpu.step().expect("nop step");
        assert_eq!(prev_pc + 1u8.into(), cpu.PC);
        cpu.step().expect("nop step");
        cpu.step().expect("nop step");
        cpu.step().expect("nop step");
        assert_eq!(prev_pc + 4u8.into(), cpu.PC);
    }
}
