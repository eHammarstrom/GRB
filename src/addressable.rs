use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

pub trait Addressable {
    type Addr: Debug + Display;
    type Data: Debug + Display + Copy;

    fn read_byte(&self, addr: Self::Addr) -> Result<Self::Data, AddressError<Self::Addr>>;
}

#[derive(Debug)]
pub enum AddressError<ADDRSPACE: Debug + Display> {
    /// Carries the out-of-bounds address
    OutOfBounds(ADDRSPACE),
}

impl<ADDRSPACE: Debug + Display> Display for AddressError<ADDRSPACE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressError::OutOfBounds(addr) => {
                write!(f, "Out of bounds access at {}", addr)
            }
        }
    }
}

impl<ADDRSPACE: Debug + Display> Error for AddressError<ADDRSPACE> {
}
