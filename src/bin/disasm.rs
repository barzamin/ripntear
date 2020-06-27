use std::fs;
use anyhow::Result;
use ripntear::i8085::Instruction;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
	let rom = fs::read(opt.file)?;
    let mut i = 0;
    while i < rom.len() {
        let (cnt, inst) = Instruction::from_buf(&rom[i..]);
        println!("${:04x}   {:x?}    {:x?}", i, &rom[i..i+cnt], inst);
        i += cnt;
    }


	Ok(())
}
