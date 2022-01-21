use crate::addressable::Addressable;
use crate::ram::RAM;
use crate::gpu::GPU;

pub trait Bus<'a>: Addressable {
    fn create(
        ram: &'a mut dyn RAM<Addr = Self::Addr, Data = Self::Data>,
        gpu: &'a mut dyn GPU<'a, Addr = Self::Addr, Data = Self::Data>,
    ) -> Self
    where
        Self: Sized;
}
