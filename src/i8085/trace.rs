use super::memory::BankFlags;

pub struct Tracer<'a> {
    mem: &'a [u8],
}

pub struct ProcessorState {
    pc: u16,
    // flag dirtiness?
    // bank switching?
    bankflags: BankFlags,
}
