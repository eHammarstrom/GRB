use crate::addressable::Addressable;

pub trait RAM: Addressable
{
    fn create() -> Self where Self: Sized;
}
