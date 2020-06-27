use std::fs;
use anyhow::Result;
use ripntear::i8051::decode_instruction_from_stream;

fn main() -> Result<()> {
	let rom = fs::read("testdata/8085EXER.COM")?;
    // let rom = vec![0x03, 0x13, 0x23, 0x33];
    let mut i = 0;
    loop {
        let (cnt, inst) = decode_instruction_from_stream(&rom[i..]);
        println!("{:x}: {:x?}", i, inst);
        i += cnt;
    }


	Ok(())
}
