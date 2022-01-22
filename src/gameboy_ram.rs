use crate::addressable::{AddressError, Addressable};
use crate::ram;

// Gameboy RAM; 16-bit address space, 8-bit memory width
pub struct RAM<const SIZE: usize> {
    /// Inclusive start and end addresses
    start_addr: u16,
    end_addr: u16,
    mem: [u8; SIZE],
}

impl<const SIZE: usize> Addressable for RAM<SIZE> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(
        &self,
        addr: Self::Addr,
    ) -> Result<Self::Data, AddressError<Self::Addr>> {
        if addr < self.start_addr || addr > self.end_addr {
            return Err(AddressError::OutOfBounds(addr))
        }

        let offset = addr - self.start_addr;

        Ok(self.mem[offset as usize])
    }

    fn write_byte(
        &mut self,
        addr: Self::Addr,
        data: Self::Data,
    ) -> Result<(), AddressError<Self::Addr>> {
        if addr < self.start_addr || addr > self.end_addr {
            return Err(AddressError::OutOfBounds(addr))
        }

        let offset = addr - self.start_addr;

        self.mem[offset as usize] = data;

        Ok(())
    }

}

impl<const SIZE: usize> ram::RAM for RAM<SIZE> {
    fn create(start: Self::Addr) -> Self {
        let size: Self::Addr = SIZE.try_into().expect("RAM size overflowed addspace");
        RAM {
            start_addr: start,
            end_addr: start + size - 1,
            mem: [0u8; SIZE],
        }
    }
}
