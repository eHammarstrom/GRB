use crate::addressable::*;
use crate::bus;

use std::fmt;
use std::fmt::{Debug, Display};

pub trait CPU: Sized + Debug {
    type Addr: Debug + Display + Copy + fmt::UpperHex;
    type Data: Debug + Display + Copy;

    fn create(
        clock: u32,
        bus: Box<dyn bus::Bus<Addr = Self::Addr, Data = Self::Data>>,
    ) -> Self;

    /// Executes the instruction at PC and returns cycles spent
    fn step(&mut self) -> Result<u32, CPUError<Self>>;

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()>;

    /// Returns the current CPU frequency
    fn frequency(&self) -> u32;
}

#[derive(Debug)]
pub enum CPUError<Cpu: crate::cpu::CPU> {
    BadRegisterAccess(&'static str),
    AddrErr(AddressError<Cpu::Addr>),
}

impl<Cpu: crate::cpu::CPU> From<AddressError<Cpu::Addr>> for CPUError<Cpu> {
    fn from(err: AddressError<Cpu::Addr>) -> Self {
        CPUError::AddrErr(err)
    }
}

impl<Cpu: crate::cpu::CPU> fmt::Display for CPUError<Cpu> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CPUError::*;
        match self {
            BadRegisterAccess(s) => write!(f, "Bad register access: {s}"),
            AddrErr(addr) => write!(f, "Bad instruction access at: {addr}"),
        }
    }
}

impl<Cpu: crate::cpu::CPU + Debug> std::error::Error for CPUError<Cpu> {}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Word(u16);

impl Word {
    pub fn nil() -> Self {
        Word(0)
    }

    pub fn set_high(&mut self, b: u8) {
        self.0 = (b as u16) << 8 | self.0 & 0xFF;
    }

    pub fn set_low(&mut self, b: u8) {
        self.0 = self.0 & 0xFF00 | (b as u16);
    }

    pub fn get_high(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn get_low(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub fn set_bit(&mut self, bit: u8) {
        assert!(bit < 16);
        self.0 |= 1 << bit;
    }

    pub fn unset_bit(&mut self, bit: u8) {
        assert!(bit < 16);
        self.0 &= !(1 << bit);
    }

    pub fn get_bit(&self, bit: u8) -> u16 {
        assert!(bit < 16);
        self.0 & 1 << bit
    }

    pub fn is_bit_set(&self, bit: u8) -> bool {
        assert!(bit < 16);
        (self.0 & 1 << bit) != 0
    }
}

impl From<u8> for Word {
    fn from(val: u8) -> Self {
        Word(val as u16)
    }
}

impl From<u16> for Word {
    fn from(val: u16) -> Self {
        Word(val)
    }
}

impl From<Word> for u16 {
    fn from(val: Word) -> Self {
        val.0
    }
}

impl std::ops::AddAssign for Word {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::ops::Add for Word {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        (self.0 + rhs.0).into()
    }
}
