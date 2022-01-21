use crate::addressable::{AddressError, Addressable};
use crate::ram::RAM;

// Gameboy RAM; 16-bit address space, 8-bit memory width
pub struct GameBoyRAM<const SIZE: usize> {
    /// Inclusive start and end addresses
    start_addr: u16,
    end_addr: u16,
    mem: [u8; SIZE],
}

impl<const SIZE: usize> Addressable for GameBoyRAM<SIZE> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(
        &self,
        addr: Self::Addr,
    ) -> Result<Self::Data, AddressError<Self::Addr>> {
        if addr < self.start_addr || addr > self.end_addr {
            return Err(AddressError::OutOfBounds(addr))
        }

        Ok(0)
    }

    fn write_byte(
        &self,
        addr: Self::Addr,
        data: Self::Data,
    ) -> Result<(), AddressError<Self::Addr>> {
        if addr < self.start_addr || addr > self.end_addr {
            return Err(AddressError::OutOfBounds(addr))
        }

        Ok(())
    }

}

impl<const SIZE: usize> RAM for GameBoyRAM<SIZE> {
    fn create(start: Self::Addr, end: Self::Addr) -> Self {
        GameBoyRAM {
            start_addr: start,
            end_addr: end,
            mem: [0u8; SIZE],
        }
    }
}
