use crate::addressable::{AddressError, Addressable};
use crate::ram::RAM;

// Gameboy RAM; 16-bit address space, 8-bit memory width
pub struct GameBoyRAM<const SIZE: usize> {
    mem: [u8; SIZE],
}

impl<const SIZE: usize> Addressable for GameBoyRAM<SIZE> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(
        &self,
        addr: Self::Addr,
    ) -> Result<Self::Data, AddressError<Self::Addr>> {
        Err(AddressError::OutOfBounds(addr))
    }

    fn write_byte(
        &self,
        addr: Self::Addr,
        data: Self::Data,
    ) -> Result<(), AddressError<Self::Addr>> {
        Err(AddressError::OutOfBounds(addr))
    }

}

impl<const SIZE: usize> RAM for GameBoyRAM<SIZE> {
    fn create() -> Self {
        GameBoyRAM { mem: [0u8; SIZE] }
    }
}