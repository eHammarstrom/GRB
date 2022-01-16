use crate::ram::RAM;

pub trait Bus<R>
where R: RAM<Addr=Self::Addr, Data=Self::Data>
{
    type Addr;
    type Data;

    fn create(ram: R) -> Self;
}
