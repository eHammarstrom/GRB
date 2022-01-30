use crate::addressable::Addressable;

pub trait RAM: Addressable {
    fn create(start: Self::Addr) -> Self
    where
        Self: Sized;

    fn deep_copy(&self) -> Vec<Self::Data>;
}
