use crate::addressable::Addressable;
use crate::gpu::GPU;
use crate::ram::RAM;

pub trait Bus<'a>: Addressable {
    fn create(
        ram: &'a mut dyn RAM<Addr = Self::Addr, Data = Self::Data>,
        gpu: &'a mut dyn GPU<'a, Addr = Self::Addr, Data = Self::Data>,
    ) -> Self
    where
        Self: Sized;
}
