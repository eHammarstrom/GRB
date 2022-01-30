use crate::addressable::*;
use crate::bus;
use crate::gpu::GPU;
use crate::ram::RAM;
use crate::timed::*;

pub struct Bus<'a> {
    ram: &'a mut dyn RAM<Addr = u16, Data = u8>,
    gpu: &'a mut dyn GPU<'a, Addr = u16, Data = u8>,
}

impl<'a> Addressable for Bus<'a> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(&self, addr: Self::Addr) -> Result<Self::Data, AddressError<Self::Addr>> {
        self.ram.read_byte(addr).or(self.gpu.read_byte(addr))
    }

    fn write_byte(
        &mut self,
        addr: Self::Addr,
        data: Self::Data,
    ) -> Result<(), AddressError<Self::Addr>> {
        self.ram
            .write_byte(addr, data)
            .or(self.gpu.write_byte(addr, data))
    }
}

impl<'a> Timed for Bus<'a> {
    fn catchup(&mut self, time: CycleTime) {
        self.gpu.catchup(time);
        // TODO: self.timer.catchup(time);
    }
}

impl<'a> bus::Bus<'a> for Bus<'a> {
    fn create(
        ram: &'a mut dyn RAM<Addr = Self::Addr, Data = Self::Data>,
        gpu: &'a mut dyn GPU<'a, Addr = Self::Addr, Data = Self::Data>,
    ) -> Self {
        Bus { ram, gpu }
    }

    fn copy_of(&self, target: bus::CopyOf) -> Vec<Self::Data> {
        match target {
            bus::CopyOf::RAM => self.ram.deep_copy(),
            bus::CopyOf::VRAM => self.gpu.deep_copy(),
        }
    }
}
