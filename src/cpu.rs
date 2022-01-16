use crate::bus;
use crate::ram;

pub trait CPU<ADDRSPACE, MEMWIDTH, B, R>: Sized
where
    R: ram::RAM<Addr = ADDRSPACE, Data = MEMWIDTH>,
    B: bus::Bus<R, Addr = ADDRSPACE, Data = MEMWIDTH>,
{
    fn create(bus: B, ram: R) -> Self;

    /// Executes the instruction at PC and returns cycles spent
    fn step(&mut self) -> usize;

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()>;
}
