use crate::bus;

pub trait CPU<'a, ADDRSPACE, MEMWIDTH>: Sized {
    fn create(
        bus: &'a dyn bus::Bus<'a, Addr = ADDRSPACE, Data = MEMWIDTH>
    ) -> Self;

    /// Executes the instruction at PC and returns cycles spent
    fn step(&mut self) -> usize;

    /// Pushes any interrupt onto the stack if any were available
    fn interrupt(&mut self) -> Option<()>;
}
