use crate::bus;
use crate::addressable::*;

use std::fmt::{Debug, Display};

pub trait CPU<'a>: Sized {
    type Addr: Debug + Display + Copy;
    type Data: Debug + Display + Copy;

    fn create(
        clock: u32,
        bus: &'a dyn bus::Bus<'a, Addr = Self::Addr, Data = Self::Data>
    ) -> Self;

    /// Executes the instruction at PC and returns cycles spent
    fn step(&mut self) -> Result<usize, AddressError<Self::Addr>>;

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()>;
}

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
