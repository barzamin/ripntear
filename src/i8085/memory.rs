use bitflags::bitflags;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
// TODO: make (more) generic (ie, remove the PC-8300 specific parts)

bitflags! {
    pub struct BankFlags: u8 {
        const HADR2 = 1 << 3;
        const HADR1 = 1 << 2;
        const LADR2 = 1 << 1;
        const LADR1 = 1 << 0;
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum LowBank {
    Rom0 = 0b00,
    Rom1 = 0b01,
    Ram2 = 0b10,
    Ram3 = 0b11,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum HighBank {
    StdRam = 0b00,
    Unused = 0b01,
    Ram2 = 0b10,
    Ram3 = 0b11,
}

impl BankFlags {
    pub fn low(&self) -> LowBank {
        LowBank::try_from((*self & (BankFlags::LADR1 | BankFlags::LADR2)).bits()).unwrap()
    }

    pub fn high(&self) -> HighBank {
        HighBank::try_from((*self & (BankFlags::HADR1 | BankFlags::HADR2)).bits()).unwrap()
    }
}
