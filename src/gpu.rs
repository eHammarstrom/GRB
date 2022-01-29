use crate::addressable::Addressable;
use crate::ram::RAM;
use crate::timed::Timed;

pub trait GPU<'a>: Addressable + Timed {
    fn create(vram: &'a dyn RAM<Addr = Self::Addr, Data = Self::Data>) -> Self
    where
        Self: Sized;
}
