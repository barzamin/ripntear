use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

pub struct Tracer<'a> {
    mem: &'a [u8],
}

bitflags! {
    struct BankFlags: u8 {
        const HADR2 = 1 << 3;
        const HADR1 = 1 << 2;
        const LADR2 = 1 << 1;
        const LADR1 = 1 << 0;
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
enum LowBank {
    Rom0 = 0b00,
    Rom1 = 0b01,
    Ram2 = 0b10,
    Ram3 = 0b11,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
enum HighBank {
    StdRam = 0b00,
    Unused = 0b01,
    Ram2 = 0b10,
    Ram3 = 0b11,
}

impl BankFlags {
    fn low(&self) -> LowBank {
        LowBank::try_from((*self & (BankFlags::LADR1 | BankFlags::LADR2)).bits()).unwrap()
    }

    fn high(&self) -> HighBank {
        HighBank::try_from((*self & (BankFlags::HADR1 | BankFlags::HADR2)).bits()).unwrap()
    }
}

pub struct ProcessorState {
    pc: u16,
    // flag dirtiness?
    // bank switching?
    bankflags: BankFlags,
}
