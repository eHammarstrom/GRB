use crate::addressable::Addressable;
use crate::timed::Timed;
use crate::ram::RAM;

pub trait GPU<'a>: Addressable + Timed {
    fn create(vram: &'a dyn RAM<Addr=Self::Addr, Data=Self::Data>) -> Self where Self: Sized;
}
