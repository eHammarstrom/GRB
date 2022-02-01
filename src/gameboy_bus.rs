use crate::addressable::*;
use crate::bus;
use crate::gpu::GPU;
use crate::ram::RAM;
use crate::timed::*;

#[derive(Debug)]
pub struct Bus {
    ram: Box<dyn RAM<Addr = u16, Data = u8>>,
    gpu: Box<dyn GPU<Addr = u16, Data = u8>>,
}

impl Addressable for Bus {
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

impl Timed for Bus {
    fn catchup(&mut self, time: CycleTime) {
        self.gpu.catchup(time);
        // TODO: self.timer.catchup(time);
    }
}

impl bus::Bus for Bus {
    fn create(
        ram: Box<dyn RAM<Addr = Self::Addr, Data = Self::Data>>,
        gpu: Box<dyn GPU<Addr = Self::Addr, Data = Self::Data>>,
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
