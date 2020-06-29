use anyhow::Result;
use ripntear::i8085;
use ripntear::{AddressWidth, Printer};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let rom = fs::read("../239056r2-3.bin")?;
    let mut i = 0;
    let mut instructions = Vec::new();
    while i < rom.len() {
        let (cnt, inst) = i8085::Instruction::decode_one(&rom[i..]);
        instructions.push((i, inst));
        i += cnt;
        // print!("${:04x}    ", i);
        // let mut width = 2 * 3 + 3;
        // for b in &rom[i..i+cnt] {
        //     print!("{}", format!("{:02x} ", b).green());
        //     width -= 3;
        // }
        // for _ in 0..width {
        //     print!(" ");
        // }
        // println!("    {}", inst.raw_asm().purple());
        // // {:x?}           {}", i, &rom[i..i+cnt], inst.raw_asm());
        // i += cnt;
    }

    Printer::new(instructions, AddressWidth::Bits16).with_color().print(&mut std::io::stdout()).unwrap();

    Ok(())
}
