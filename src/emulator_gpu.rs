use crate::addressable::{Addressable, AddressError};
use crate::timed::{Timed, CycleTime};
use crate::gpu::GPU;
use crate::ram::RAM;

pub struct EmuGPU<'a> {
    vram: &'a dyn RAM<Addr=u16, Data=u8>
}

impl<'a> GPU<'a> for EmuGPU<'a> {
    fn create(vram: &'a dyn RAM<Addr=Self::Addr, Data=Self::Data>) -> Self {
        EmuGPU {
            vram,
        }
    }
}

impl<'a> Addressable for EmuGPU<'a> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(&self, addr: Self::Addr) -> Result<Self::Data, AddressError<Self::Addr>> {
        Ok(0)
    }

    fn write_byte(&self, addr: Self::Addr, data: Self::Data) -> Result<(), AddressError<Self::Addr>> {
        Ok(())
    }
}

impl<'a> Timed for EmuGPU<'a> {
    fn catchup(&self, time: CycleTime) {
    }
}
