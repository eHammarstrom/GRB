use crate::addressable::{AddressError, Addressable};
use crate::gpu;
use crate::ram::RAM;
use crate::timed::{CycleTime, Timed};

#[derive(Debug)]
pub struct GPU {
    vram: Box<dyn RAM<Addr = u16, Data = u8>>,
}

impl gpu::GPU for GPU {
    fn create(vram: Box<dyn RAM<Addr = Self::Addr, Data = Self::Data>>) -> Self {
        GPU { vram }
    }

    fn deep_copy(&self) -> Vec<Self::Data> {
        self.vram.deep_copy()
    }
}

impl Addressable for GPU {
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

impl Timed for GPU {
    fn catchup(&mut self, time: CycleTime) {}
}
