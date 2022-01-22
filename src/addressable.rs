use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, UpperHex };

pub trait Addressable {
    type Addr: Debug + Display + UpperHex + Copy;
    type Data: Debug + Display + UpperHex + Copy;

    fn read_byte(&self, addr: Self::Addr) -> Result<Self::Data, AddressError<Self::Addr>>;
    fn write_byte(&mut self, addr: Self::Addr, data: Self::Data) -> Result<(), AddressError<Self::Addr>>;
}

#[derive(Debug)]
pub enum AddressError<ADDRSPACE: Debug + Copy> {
    /// Carries the out-of-bounds address
    OutOfBounds(ADDRSPACE),
    IllegalInstr(ADDRSPACE),
}

impl<ADDRSPACE: Debug + Copy + Display + UpperHex> Display for AddressError<ADDRSPACE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AddressError::*;
        match self {
            OutOfBounds(addr) => {
                write!(f, "Out of bounds access at {:#X}", addr)
            }
            IllegalInstr(addr) => {
                write!(f, "Illegal instruction at {:#X}", addr)
            }
        }
    }
}

impl<ADDRSPACE: Debug + Copy + Display + UpperHex> Error for AddressError<ADDRSPACE> {
}
