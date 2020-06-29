use std::io::{self, Write};
use colored::Colorize;

pub trait Print {
    fn print<W>(&self, w: &mut W) -> io::Result<()> where W: Write;
}

type Address = usize;
pub enum AddressWidth {
    Bits16,
    Bits32,
    Bits64,
}

pub struct Printer<I> where I: Print {
    instructions: Vec<(Address, I)>,
    address_width: AddressWidth,
    color: bool,
}

impl<I> Printer<I> where I: Print {
    pub fn new(instructions: Vec<(Address, I)>, address_width: AddressWidth) -> Printer<I> {
        Printer {
            instructions,
            address_width,
            color: false,
        }
    }

    pub fn with_color(mut self) -> Printer<I> {
        self.color = true;
        self
    }

    pub fn print<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
        for (addr, instr) in &self.instructions {
            match self.address_width {
                AddressWidth::Bits16 => write!(w, "{:04x}", addr)?,
                AddressWidth::Bits32 => write!(w, "{:08x}", addr)?,
                AddressWidth::Bits64 => write!(w, "{:016x}", addr)?,
            }

            write!(w, "    ")?;
            instr.print(w)?;

            writeln!(w)?;
        }

        Ok(())
    }
}
