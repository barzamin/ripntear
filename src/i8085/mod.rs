use std::fmt;
use std::io::{self, Write};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::printer::Print;

mod decode;
mod trace;
mod memory;

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Register {
    A = 0b111,
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    /// (HL)
    Mem = 0b110,
}

#[derive(Debug)]
#[repr(u8)]
pub enum RegisterPair {
    BC, // 00
    DE, // 01
    HL, // 10
    SP, // 11
    PSW, // 11
}

#[derive(Debug)]
#[repr(u8)]
pub enum ConditionCodes {
    NZ = 0b000,
    Z = 0b001,
    NC = 0b010,
    C = 0b011,
    PO = 0b100,
    PE = 0b101,
    P = 0b110,
    M = 0b111,
}

impl fmt::Display for RegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RegisterPair::*;
        match self {
            BC => write!(f, "bc"),
            DE => write!(f, "de"),
            HL => write!(f, "hl"),
            SP => write!(f, "sp"),
            PSW => write!(f, "psw"),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Register::*;
        match self {
            A => write!(f, "a"),
            B => write!(f, "b"),
            C => write!(f, "c"),
            D => write!(f, "d"),
            E => write!(f, "e"),
            H => write!(f, "h"),
            L => write!(f, "l"),
            Mem => write!(f, "m"),
        }
    }
}

impl fmt::Display for ConditionCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ConditionCodes::*;
        match self {
            NZ => write!(f, "nz"),
            Z => write!(f, "z"),
            NC => write!(f, "nc"),
            C => write!(f, "c"),
            PO => write!(f, "po"),
            PE => write!(f, "pe"),
            P => write!(f, "p"),
            M => write!(f, "m"),
        }
    }
}


#[derive(Debug)]
pub enum Instruction {
    Nop,
    Hlt,

    Lxi { reg: RegisterPair, value: u16 },

    Stax { ptr: RegisterPair },
    Ldax { ptr: RegisterPair },

    Inx { reg_pair: RegisterPair },
    Dcx { reg_pair: RegisterPair },

    Inr { reg: Register },
    Dcr { reg: Register },

    Mvi { reg: Register, value: u8 },

    Dad { reg_pair: RegisterPair },

    Rlc, Ral,
    Rrc, Rar,

    Ei, Di,

    Add { reg: Register },
    Adc { reg: Register },
    Sub { reg: Register },
    Sbb { reg: Register },
    Ana { reg: Register },
    Ora { reg: Register },
    Xra { reg: Register },
    Cmp { reg: Register },

    Mov { src: Register, dest: Register },

    Rim, Sim,

    In { port: u8 },
    Out { port: u8 },

    Lda { addr: u16 },
    Sta { addr: u16 },

    Lhld { addr: u16 },
    Shld { addr: u16 },

    Daa,
    Stc,
    Cma,
    Cmc,

    Jmp { addr: u16, condition: Option<ConditionCodes> },
    Call { addr: u16, condition: Option<ConditionCodes> },
    Ret { condition: Option<ConditionCodes> },

    Adi { value: u8 },
    Aci { value: u8 },
    Sui { value: u8 },
    Sbi { value: u8 },
    Ani { value: u8 },
    Ori { value: u8 },
    Xri { value: u8 },
    Cpi { value: u8 },


    Pop { reg_pair: RegisterPair },
    Push { reg_pair: RegisterPair },

    Xthl,
    Xchg,
    Pchl,
    Sphl,

    Rst { index: u8 },

    Dsub,
    Arhl,
    Rdel,

    Ldhi { imm: u8 },
    Ldsi { imm: u8 },

    Shlx,
    Lhlx,

    Jnk { addr: u16 },
    Jk { addr: u16 },

    Rstv,
}

impl Instruction {
    pub fn raw_asm(&self) -> String {
        use Instruction::*;
        match self {
            Nop => format!("nop"),
            Hlt => format!("hlt"),
            Xthl => format!("xthl"),
            Xchg => format!("xchg"),
            Pchl => format!("pchl"),
            Sphl => format!("sphl"),

            Rst {index} => format!("rst {:#x}", index),
            Rstv => format!("rstv"),

            Dsub => format!("dsub"),
            Arhl => format!("arhl"),
            Rdel => format!("rdel"),
            Shlx => format!("shlx"),
            Lhlx => format!("lhlx"),

            Rlc => format!("rlc"),
            Ral => format!("ral"),
            Rrc => format!("rrc"),
            Rar => format!("rar"),
            Ei => format!("ei"),
            Di => format!("di"),

            Daa => format!("daa"),
            Stc => format!("stc"),
            Cma => format!("cma"),
            Cmc => format!("cmc"),
            Rim => format!("rim"),
            Sim => format!("sim"),

            Ldhi { imm } => format!("ldhi {:#x}", imm),
            Ldsi { imm } => format!("ldsi {:#x}", imm),

            Jnk { addr } => format!("jnk {:#x}", addr),
            Jk { addr } => format!("jk {:#x}", addr),

            Mov { src, dest } => format!("mov {}, {}", dest, src),

            Adi { value } => format!("adi {:#x}", value),
            Aci { value } => format!("aci {:#x}", value),
            Sui { value } => format!("sui {:#x}", value),
            Sbi { value } => format!("sbi {:#x}", value),
            Ani { value } => format!("ani {:#x}", value),
            Ori { value } => format!("ori {:#x}", value),
            Xri { value } => format!("xri {:#x}", value),
            Cpi { value } => format!("cpi {:#x}", value),

            Add { reg } => format!("add {}", reg),
            Adc { reg } => format!("adc {}", reg),
            Sub { reg } => format!("sub {}", reg),
            Sbb { reg } => format!("sbb {}", reg),
            Ana { reg } => format!("ana {}", reg),
            Ora { reg } => format!("ora {}", reg),
            Xra { reg } => format!("xra {}", reg),
            Cmp { reg } => format!("cmp {}", reg),

            Pop { reg_pair } => format!("pop {}", reg_pair),
            Push { reg_pair } => format!("push {}", reg_pair),

            Stax { ptr } => format!("stax {}", ptr),
            Ldax { ptr } => format!("ldax {}", ptr),

            Inx { reg_pair } => format!("inx {}", reg_pair),
            Dcx { reg_pair } => format!("dcx {}", reg_pair),


            Inr { reg } => format!("inr {}", reg),
            Dcr { reg } => format!("dcr {}", reg),

            Lxi { reg, value } => format!("lxi {}, {:#x}", reg, value),
            Mvi { reg, value } => format!("mvi {}, {:#x}", reg, value),

            Dad { reg_pair } => format!("dad {}", reg_pair),

            In { port } => format!("in {:#x}", port),
            Out { port } => format!("out {:#x}", port),

            Lda { addr } => format!("lda {:#x}", addr),
            Sta { addr } => format!("sta {:#x}", addr),
            Lhld { addr } => format!("lhld {:#x}", addr),
            Shld { addr } => format!("shld {:#x}", addr),

            Jmp { addr, condition } => match condition {
                None => format!("jmp {:#x}", addr),
                Some(cond) => format!("j{} {:#x}", cond, addr),
            },
            Call { addr, condition } => match condition {
                None => format!("call {:#x}", addr),
                Some(cond) => format!("c{} {:#x}", cond, addr),
            },
            Ret { condition } => match condition {
                None => format!("ret"),
                Some(cond) => format!("r{}", cond),
            },
        }
    }
}

impl Print for Instruction {
    fn print<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
        write!(w, "{}", self.raw_asm())
    }
}
