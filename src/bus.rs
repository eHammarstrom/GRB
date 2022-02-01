use crate::addressable::Addressable;
use crate::gpu::GPU;
use crate::ram::RAM;
use crate::timed::Timed;

pub enum CopyOf {
    RAM,
    VRAM,
}

pub trait Bus: Addressable + Timed + std::fmt::Debug {
    fn create(
        ram: Box<dyn RAM<Addr = Self::Addr, Data = Self::Data>>,
        gpu: Box<dyn GPU<Addr = Self::Addr, Data = Self::Data>>,
    ) -> Self
    where
        Self: Sized;

    fn copy_of(&self, target: CopyOf) -> Vec<Self::Data>;
}
