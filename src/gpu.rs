use crate::addressable::Addressable;
use crate::timed::Timed;

pub trait GPU: Addressable + Timed {
    fn create() -> Self;
}
