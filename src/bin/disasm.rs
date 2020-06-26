use std::fs;
use anyhow::Result;
use ripntear::i8051::decode_instruction_from_stream;

fn main() -> Result<()> {
	let rom = fs::read("testdata/8085EXER.COM")?;
    println!("{:?}", decode_instruction_from_stream(&rom));

	Ok(())
}
