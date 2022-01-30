use crate::addressable::*;
use crate::bus;

use std::fmt;
use std::fmt::{Debug, Display};

pub trait CPU<'a>: Sized + Debug {
    type Addr: Debug + Display + Copy + fmt::UpperHex;
    type Data: Debug + Display + Copy;

    fn create(
        clock: u32,
        bus: &'a mut dyn bus::Bus<'a, Addr = Self::Addr, Data = Self::Data>,
    ) -> Self;

    /// Executes the instruction at PC and returns cycles spent
    fn step(&mut self) -> Result<u32, CPUError<'a, Self>>;

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()>;

    /// Returns the current CPU frequency
    fn frequency(&self) -> u32;
}

#[derive(Debug)]
pub enum CPUError<'a, Cpu: crate::cpu::CPU<'a>> {
    BadRegisterAccess(&'static str),
    AddrErr(AddressError<Cpu::Addr>),
}

impl<'a, Cpu: crate::cpu::CPU<'a>> From<AddressError<Cpu::Addr>> for CPUError<'a, Cpu> {
    fn from(err: AddressError<Cpu::Addr>) -> Self {
        CPUError::AddrErr(err)
    }
}

impl<'a, Cpu: crate::cpu::CPU<'a>> fmt::Display for CPUError<'a, Cpu> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CPUError::*;
        match self {
            BadRegisterAccess(s) => write!(f, "Bad register access: {s}"),
            AddrErr(addr) => write!(f, "Bad instruction access at: {addr}"),
        }
    }
}

impl<'a, Cpu: crate::cpu::CPU<'a> + Debug> std::error::Error for CPUError<'a, Cpu> {}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Word(u16);

impl Word {
    pub fn set_high(&mut self, b: u8) {
        self.0 = (b as u16) << 8 | self.0 & 0xFF;
    }

    pub fn set_low(&mut self, b: u8) {
        self.0 = self.0 & 0xFF00 | (b as u16);
    }

    pub fn get_high(&mut self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn get_low(&mut self) -> u8 {
        (self.0 & 0xFF) as u8
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
