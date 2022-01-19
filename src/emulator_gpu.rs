use crate::addressable::{Addressable, AddressError};
use crate::timed::{Timed, CycleTime};
use crate::gpu::GPU;

struct EmulatorGPU<const SIZE: usize> {
    vram: [u8; SIZE]
}

impl<const SIZE: usize> GPU for EmulatorGPU<SIZE> {
    fn create() -> Self {
        EmulatorGPU {
            vram: [0u8; SIZE],
        }
    }
}

impl<const SIZE: usize> Addressable for EmulatorGPU<SIZE> {
    type Addr = u16;
    type Data = u8;

    fn read_byte(&self, addr: Self::Addr) -> Result<Self::Data, AddressError<Self::Addr>> {
        Ok(0)
    }
}

impl<const SIZE: usize> Timed for EmulatorGPU<SIZE> {
    fn catchup(time: CycleTime) {
    }
}
