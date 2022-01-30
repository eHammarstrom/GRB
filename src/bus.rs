use crate::addressable::Addressable;
use crate::gpu::GPU;
use crate::ram::RAM;
use crate::timed::Timed;

pub enum CopyOf {
    RAM,
    VRAM,
}

pub trait Bus<'a>: Addressable + Timed + std::fmt::Debug {
    fn create(
        ram: &'a mut dyn RAM<Addr = Self::Addr, Data = Self::Data>,
        gpu: &'a mut dyn GPU<'a, Addr = Self::Addr, Data = Self::Data>,
    ) -> Self
    where
        Self: Sized;

    fn copy_of(&self, target: CopyOf) -> Vec<Self::Data>;
}
