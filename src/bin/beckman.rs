use std::fs;
use anyhow::Result;
use ripntear::i8085;
use std::path::PathBuf;


fn main() -> Result<()> {
    let rom = fs::read("../239056r2-3.bin")?;
    let mut i = 0;
    while i < rom.len() {
        let (cnt, inst) = i8085::Instruction::decode_one(&rom[i..]);
        println!("${:04x}    {:x?}           {}", i, &rom[i..i+cnt], inst.raw_asm());
        i += cnt;
    }


    Ok(())
}
