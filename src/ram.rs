use crate::addressable::Addressable;

pub trait RAM: Addressable + std::fmt::Debug {
    fn create(start: Self::Addr) -> Self
    where
        Self: Sized;

    fn deep_copy(&self) -> Vec<Self::Data>;
}
