use crate::addressable::{AddressError, Addressable};
use crate::gpu;
use crate::ram::RAM;
use crate::timed::{CycleTime, Timed};

pub struct GPU<'a> {
    vram: &'a dyn RAM<Addr = u16, Data = u8>,
}

impl<'a> gpu::GPU<'a> for GPU<'a> {
    fn create(vram: &'a dyn RAM<Addr = Self::Addr, Data = Self::Data>) -> Self {
        GPU { vram }
    }

    fn deep_copy(&self) -> Vec<Self::Data> {
        self.vram.deep_copy()
    }
}

impl<'a> Addressable for GPU<'a> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(&self, addr: Self::Addr) -> Result<Self::Data, AddressError<Self::Addr>> {
        Ok(0)
    }

    fn write_byte(
        &mut self,
        addr: Self::Addr,
        data: Self::Data,
    ) -> Result<(), AddressError<Self::Addr>> {
        Ok(())
    }
}

impl<'a> Timed for GPU<'a> {
    fn catchup(&mut self, time: CycleTime) {}
}
