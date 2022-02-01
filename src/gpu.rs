use crate::addressable::Addressable;
use crate::ram::RAM;
use crate::timed::Timed;

pub trait GPU: Addressable + Timed + std::fmt::Debug {
    fn create(vram: Box<dyn RAM<Addr = Self::Addr, Data = Self::Data>>) -> Self
    where
        Self: Sized;

    fn deep_copy(&self) -> Vec<Self::Data>;
}
