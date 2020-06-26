use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

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

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum RegisterPair {
    BC = 00,
    DE = 01,
    HL = 10,
    SP = 11,
    // PSW, // 11
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Hlt,
    Lxi { to: RegisterPair, value: u16 },
    Stax { ptr: RegisterPair },
    Ldax { ptr: RegisterPair },
    Inx { reg_pair: RegisterPair },
    Inr { reg: Register },
    Dcr { reg: Register },
    Mvi { reg: Register, value: u8 },
    Dad { reg_pair: RegisterPair },
    Rlc, Ral,
    Rrc, Rar,
}

fn lohi(lo: u8, hi: u8) -> u16 {
    ((hi as u16) << 8) | lo as u16
}


pub fn decode_instruction_from_stream(buf: &[u8]) -> Instruction {
    use Instruction::*;
    match *buf {
        // NOP
        [0x00, ..] => Nop,

        // HLT: 0x76
        [0x76, ..] => Hlt,

        // LXI: 00RP0001
        [0b00_00_0001, lo, hi, ..] => Lxi { to: RegisterPair::BC, value: lohi(lo, hi) },
        [0b00_01_0001, lo, hi, ..] => Lxi { to: RegisterPair::DE, value: lohi(lo, hi) },
        [0b00_10_0001, lo, hi, ..] => Lxi { to: RegisterPair::HL, value: lohi(lo, hi) },
        [0b00_11_0001, lo, hi, ..] => Lxi { to: RegisterPair::SP, value: lohi(lo, hi) },

        // STAX: 00RP0010
        [0b00_00_0010, ..] => Stax { ptr: RegisterPair::BC },
        [0b00_01_0010, ..] => Stax { ptr: RegisterPair::DE },

        // INX: 00RP0011
        [0b00_00_0011, ..] => Inx { reg_pair: RegisterPair::BC },
        [0b00_01_0011, ..] => Inx { reg_pair: RegisterPair::DE },
        [0b00_10_0011, ..] => Inx { reg_pair: RegisterPair::HL },
        [0b00_11_0011, ..] => Inx { reg_pair: RegisterPair::SP },

        // INR: 00DDD100
        [0b00_111_100, ..] => Inr { reg: Register::A },
        [0b00_000_100, ..] => Inr { reg: Register::B },
        [0b00_001_100, ..] => Inr { reg: Register::C },
        [0b00_010_100, ..] => Inr { reg: Register::D },
        [0b00_011_100, ..] => Inr { reg: Register::E },
        [0b00_100_100, ..] => Inr { reg: Register::H },
        [0b00_101_100, ..] => Inr { reg: Register::L },
        [0b00_110_100, ..] => Inr { reg: Register::Mem },

        // DCR: 00RRR101
        [0b00_111_101, ..] => Dcr { reg: Register::A },
        [0b00_000_101, ..] => Dcr { reg: Register::B },
        [0b00_001_101, ..] => Dcr { reg: Register::C },
        [0b00_010_101, ..] => Dcr { reg: Register::D },
        [0b00_011_101, ..] => Dcr { reg: Register::E },
        [0b00_100_101, ..] => Dcr { reg: Register::H },
        [0b00_101_101, ..] => Dcr { reg: Register::L },
        [0b00_110_101, ..] => Dcr { reg: Register::Mem },

        // MVI: 00RRR110
        [0b00_111_110, v, ..] => Mvi { reg: Register::A, value: v },
        [0b00_000_110, v, ..] => Mvi { reg: Register::B, value: v },
        [0b00_001_110, v, ..] => Mvi { reg: Register::C, value: v },
        [0b00_010_110, v, ..] => Mvi { reg: Register::D, value: v },
        [0b00_011_110, v, ..] => Mvi { reg: Register::E, value: v },
        [0b00_100_110, v, ..] => Mvi { reg: Register::H, value: v },
        [0b00_101_110, v, ..] => Mvi { reg: Register::L, value: v },
        [0b00_110_110, v, ..] => Mvi { reg: Register::Mem, value: v },

        // RLC: 00000111
        [0b000_0_0_111, ..] => Rlc,
        // RRC: 00000111
        [0b000_0_1_111, ..] => Rrc,
        // RAL: 00000111
        [0b000_1_0_111, ..] => Ral,
        // RAR: 00000111
        [0b000_1_1_111, ..] => Rar,

        // DAD: 00RP1001
        [0b00_00_1001, ..] => Dad { reg_pair: RegisterPair::BC },
        [0b00_01_1001, ..] => Dad { reg_pair: RegisterPair::DE },
        [0b00_10_1001, ..] => Dad { reg_pair: RegisterPair::HL },
        [0b00_11_1001, ..] => Dad { reg_pair: RegisterPair::SP },

        // LDAX: 00RP1010
        [0b00_00_1010, ..] => Ldax { ptr: RegisterPair::BC },
        [0b00_01_1010, ..] => Ldax { ptr: RegisterPair::DE },

        // DCX: 00RP1011
        [0b00_00_1011, ..] => Dad { reg_pair: RegisterPair::BC },
        [0b00_01_1011, ..] => Dad { reg_pair: RegisterPair::DE },
        [0b00_10_1011, ..] => Dad { reg_pair: RegisterPair::HL },
        [0b00_11_1011, ..] => Dad { reg_pair: RegisterPair::SP },

        _ => {
            todo!();
        }
    }
}
