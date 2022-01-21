use crate::addressable::*;
use crate::bus::Bus;
use crate::ram::RAM;
use crate::gpu::GPU;

pub struct EmuBus<'a> {
    ram: &'a mut dyn RAM<Addr = u16, Data = u8>,
    gpu: &'a mut dyn GPU<'a, Addr = u16, Data = u8>,
}

impl<'a> Addressable for EmuBus<'a> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(&self, addr: Self::Addr) -> Result<Self::Data, AddressError<Self::Addr>> {
        self.ram.read_byte(addr)
            .or(self.gpu.read_byte(addr))
    }

    fn write_byte(
        &self,
        addr: Self::Addr,
        data: Self::Data,
    ) -> Result<(), AddressError<Self::Addr>> {
        self.ram.write_byte(addr, data)
            .or(self.gpu.write_byte(addr, data))
    }
}

impl<'a> Bus<'a> for EmuBus<'a> {
    fn create(
        ram: &'a mut dyn RAM<Addr = Self::Addr, Data = Self::Data>,
        gpu: &'a mut dyn GPU<'a, Addr = Self::Addr, Data = Self::Data>,
    ) -> Self {
        EmuBus { ram, gpu }
    }
}
