use super::{Register, Instruction, RegisterPair, ConditionCodes};

fn lohi(lo: u8, hi: u8) -> u16 {
    ((hi as u16) << 8) | lo as u16
}

impl Instruction {
      pub fn decode_one(buf: &[u8]) -> (usize, Instruction) {
        use Instruction::*;
        use ConditionCodes::*;
        let (count, instr) = match *buf {
            // NOP
            [0x00, ..] => (1, Nop),

            // HLT: 0x76
            [0x76, ..] => (1, Hlt),

            // RIM: 00100000
            // SIM: 00110000
            [0x20, ..] => (1, Rim),
            [0x30, ..] => (1, Sim),

            // EI: 11111011
            // DI: 11110011
            [0b1111_1_011, ..] => (1, Ei),
            [0b1111_0_011, ..] => (1, Di),

            // IN:  11011011
            // OUT: 11010011
            [0b1101_1_011, port, ..] => (2, In  { port }),
            [0b1101_0_011, port, ..] => (2, Out { port }),

            // LXI: 00RP0001
            [0b00_00_0001, lo, hi, ..] => (3, Lxi { reg: RegisterPair::BC, value: lohi(lo, hi) }),
            [0b00_01_0001, lo, hi, ..] => (3, Lxi { reg: RegisterPair::DE, value: lohi(lo, hi) }),
            [0b00_10_0001, lo, hi, ..] => (3, Lxi { reg: RegisterPair::HL, value: lohi(lo, hi) }),
            [0b00_11_0001, lo, hi, ..] => (3, Lxi { reg: RegisterPair::SP, value: lohi(lo, hi) }),

            // STAX: 00RP0010
            [0b00_00_0010, ..] => (1, Stax { ptr: RegisterPair::BC }),
            [0b00_01_0010, ..] => (1, Stax { ptr: RegisterPair::DE }),

            // SHLD: 00100010 lb hb
            // LHLD: 00101010 lb hb
            [0b0010_0_010, lo, hi, ..] => (3, Shld { addr: lohi(lo, hi) }),
            [0b0010_1_010, lo, hi, ..] => (3, Lhld { addr: lohi(lo, hi) }),

            // STA: 00110010 lb hb
            // LDA: 00111010 lb hb
            [0b0011_0_010, lo, hi, ..] => (3, Sta { addr: lohi(lo, hi) }),
            [0b0011_1_010, lo, hi, ..] => (3, Lda { addr: lohi(lo, hi) }),

            // INX: 00RP0011
            [0b00_00_0011, ..] => (1, Inx { reg_pair: RegisterPair::BC }),
            [0b00_01_0011, ..] => (1, Inx { reg_pair: RegisterPair::DE }),
            [0b00_10_0011, ..] => (1, Inx { reg_pair: RegisterPair::HL }),
            [0b00_11_0011, ..] => (1, Inx { reg_pair: RegisterPair::SP }),

            // INR: 00DDD100
            [0b00_111_100, ..] => (1, Inr { reg: Register::A }),
            [0b00_000_100, ..] => (1, Inr { reg: Register::B }),
            [0b00_001_100, ..] => (1, Inr { reg: Register::C }),
            [0b00_010_100, ..] => (1, Inr { reg: Register::D }),
            [0b00_011_100, ..] => (1, Inr { reg: Register::E }),
            [0b00_100_100, ..] => (1, Inr { reg: Register::H }),
            [0b00_101_100, ..] => (1, Inr { reg: Register::L }),
            [0b00_110_100, ..] => (1, Inr { reg: Register::Mem }),

            // DCR: 00RRR101
            [0b00_111_101, ..] => (1, Dcr { reg: Register::A }),
            [0b00_000_101, ..] => (1, Dcr { reg: Register::B }),
            [0b00_001_101, ..] => (1, Dcr { reg: Register::C }),
            [0b00_010_101, ..] => (1, Dcr { reg: Register::D }),
            [0b00_011_101, ..] => (1, Dcr { reg: Register::E }),
            [0b00_100_101, ..] => (1, Dcr { reg: Register::H }),
            [0b00_101_101, ..] => (1, Dcr { reg: Register::L }),
            [0b00_110_101, ..] => (1, Dcr { reg: Register::Mem }),

            // MVI: 00RRR110
            [0b00_111_110, v, ..] => (2, Mvi { reg: Register::A, value: v }),
            [0b00_000_110, v, ..] => (2, Mvi { reg: Register::B, value: v }),
            [0b00_001_110, v, ..] => (2, Mvi { reg: Register::C, value: v }),
            [0b00_010_110, v, ..] => (2, Mvi { reg: Register::D, value: v }),
            [0b00_011_110, v, ..] => (2, Mvi { reg: Register::E, value: v }),
            [0b00_100_110, v, ..] => (2, Mvi { reg: Register::H, value: v }),
            [0b00_101_110, v, ..] => (2, Mvi { reg: Register::L, value: v }),
            [0b00_110_110, v, ..] => (2, Mvi { reg: Register::Mem, value: v }),

            // RLC: 00000111
            [0b000_0_0_111, ..] => (1, Rlc),
            // RRC: 00000111
            [0b000_0_1_111, ..] => (1, Rrc),
            // RAL: 00000111
            [0b000_1_0_111, ..] => (1, Ral),
            // RAR: 00000111
            [0b000_1_1_111, ..] => (1, Rar),

            [0x27, ..] => (1, Daa),
            [0x37, ..] => (1, Stc),
            [0x2f, ..] => (1, Cma),
            [0x3f, ..] => (1, Cmc),

            // DAD: 00RP1001
            [0b00_00_1001, ..] => (1, Dad { reg_pair: RegisterPair::BC }),
            [0b00_01_1001, ..] => (1, Dad { reg_pair: RegisterPair::DE }),
            [0b00_10_1001, ..] => (1, Dad { reg_pair: RegisterPair::HL }),
            [0b00_11_1001, ..] => (1, Dad { reg_pair: RegisterPair::SP }),

            // LDAX: 00RP1010
            [0b00_00_1010, ..] => (1, Ldax { ptr: RegisterPair::BC }),
            [0b00_01_1010, ..] => (1, Ldax { ptr: RegisterPair::DE }),

            // DCX: 00RP1011
            [0b00_00_1011, ..] => (1, Dcx { reg_pair: RegisterPair::BC }),
            [0b00_01_1011, ..] => (1, Dcx { reg_pair: RegisterPair::DE }),
            [0b00_10_1011, ..] => (1, Dcx { reg_pair: RegisterPair::HL }),
            [0b00_11_1011, ..] => (1, Dcx { reg_pair: RegisterPair::SP }),

            // ADD: 10000RRR
            [0b10000_111, ..] => (1, Add { reg: Register::A }),
            [0b10000_000, ..] => (1, Add { reg: Register::B }),
            [0b10000_001, ..] => (1, Add { reg: Register::C }),
            [0b10000_010, ..] => (1, Add { reg: Register::D }),
            [0b10000_011, ..] => (1, Add { reg: Register::E }),
            [0b10000_100, ..] => (1, Add { reg: Register::H }),
            [0b10000_101, ..] => (1, Add { reg: Register::L }),
            [0b10000_110, ..] => (1, Add { reg: Register::Mem }),

            // ADC: 10001RRR
            [0b10001_111, ..] => (1, Adc { reg: Register::A }),
            [0b10001_000, ..] => (1, Adc { reg: Register::B }),
            [0b10001_001, ..] => (1, Adc { reg: Register::C }),
            [0b10001_010, ..] => (1, Adc { reg: Register::D }),
            [0b10001_011, ..] => (1, Adc { reg: Register::E }),
            [0b10001_100, ..] => (1, Adc { reg: Register::H }),
            [0b10001_101, ..] => (1, Adc { reg: Register::L }),
            [0b10001_110, ..] => (1, Adc { reg: Register::Mem }),

            // SUB: 10010RRR
            [0b10010_111, ..] => (1, Sub { reg: Register::A }),
            [0b10010_000, ..] => (1, Sub { reg: Register::B }),
            [0b10010_001, ..] => (1, Sub { reg: Register::C }),
            [0b10010_010, ..] => (1, Sub { reg: Register::D }),
            [0b10010_011, ..] => (1, Sub { reg: Register::E }),
            [0b10010_100, ..] => (1, Sub { reg: Register::H }),
            [0b10010_101, ..] => (1, Sub { reg: Register::L }),
            [0b10010_110, ..] => (1, Sub { reg: Register::Mem }),

            // SUB: 10011RRR
            [0b10011_111, ..] => (1, Sbb { reg: Register::A }),
            [0b10011_000, ..] => (1, Sbb { reg: Register::B }),
            [0b10011_001, ..] => (1, Sbb { reg: Register::C }),
            [0b10011_010, ..] => (1, Sbb { reg: Register::D }),
            [0b10011_011, ..] => (1, Sbb { reg: Register::E }),
            [0b10011_100, ..] => (1, Sbb { reg: Register::H }),
            [0b10011_101, ..] => (1, Sbb { reg: Register::L }),
            [0b10011_110, ..] => (1, Sbb { reg: Register::Mem }),

            // ANA: 10100RRR
            [0b10100_111, ..] => (1, Ana { reg: Register::A }),
            [0b10100_000, ..] => (1, Ana { reg: Register::B }),
            [0b10100_001, ..] => (1, Ana { reg: Register::C }),
            [0b10100_010, ..] => (1, Ana { reg: Register::D }),
            [0b10100_011, ..] => (1, Ana { reg: Register::E }),
            [0b10100_100, ..] => (1, Ana { reg: Register::H }),
            [0b10100_101, ..] => (1, Ana { reg: Register::L }),
            [0b10100_110, ..] => (1, Ana { reg: Register::Mem }),

            // ORA: 10100RRR
            [0b10110_111, ..] => (1, Ora { reg: Register::A }),
            [0b10110_000, ..] => (1, Ora { reg: Register::B }),
            [0b10110_001, ..] => (1, Ora { reg: Register::C }),
            [0b10110_010, ..] => (1, Ora { reg: Register::D }),
            [0b10110_011, ..] => (1, Ora { reg: Register::E }),
            [0b10110_100, ..] => (1, Ora { reg: Register::H }),
            [0b10110_101, ..] => (1, Ora { reg: Register::L }),
            [0b10110_110, ..] => (1, Ora { reg: Register::Mem }),

            // XRA: 10101RRR
            [0b10101_111, ..] => (1, Xra { reg: Register::A }),
            [0b10101_000, ..] => (1, Xra { reg: Register::B }),
            [0b10101_001, ..] => (1, Xra { reg: Register::C }),
            [0b10101_010, ..] => (1, Xra { reg: Register::D }),
            [0b10101_011, ..] => (1, Xra { reg: Register::E }),
            [0b10101_100, ..] => (1, Xra { reg: Register::H }),
            [0b10101_101, ..] => (1, Xra { reg: Register::L }),
            [0b10101_110, ..] => (1, Xra { reg: Register::Mem }),

            // CMP: 10111RRR
            [0b10111_111, ..] => (1, Cmp { reg: Register::A }),
            [0b10111_000, ..] => (1, Cmp { reg: Register::B }),
            [0b10111_001, ..] => (1, Cmp { reg: Register::C }),
            [0b10111_010, ..] => (1, Cmp { reg: Register::D }),
            [0b10111_011, ..] => (1, Cmp { reg: Register::E }),
            [0b10111_100, ..] => (1, Cmp { reg: Register::H }),
            [0b10111_101, ..] => (1, Cmp { reg: Register::L }),
            [0b10111_110, ..] => (1, Cmp { reg: Register::Mem }),


            // MOV: 01DDDSSS
            [0b01111111, ..] => (1, Mov { src: Register::A, dest: Register::A }),
            [0b01000111, ..] => (1, Mov { src: Register::A, dest: Register::B }),
            [0b01001111, ..] => (1, Mov { src: Register::A, dest: Register::C }),
            [0b01010111, ..] => (1, Mov { src: Register::A, dest: Register::D }),
            [0b01011111, ..] => (1, Mov { src: Register::A, dest: Register::E }),
            [0b01100111, ..] => (1, Mov { src: Register::A, dest: Register::H }),
            [0b01101111, ..] => (1, Mov { src: Register::A, dest: Register::L }),
            [0b01110111, ..] => (1, Mov { src: Register::A, dest: Register::Mem }),
            [0b01111000, ..] => (1, Mov { src: Register::B, dest: Register::A }),
            [0b01000000, ..] => (1, Mov { src: Register::B, dest: Register::B }),
            [0b01001000, ..] => (1, Mov { src: Register::B, dest: Register::C }),
            [0b01010000, ..] => (1, Mov { src: Register::B, dest: Register::D }),
            [0b01011000, ..] => (1, Mov { src: Register::B, dest: Register::E }),
            [0b01100000, ..] => (1, Mov { src: Register::B, dest: Register::H }),
            [0b01101000, ..] => (1, Mov { src: Register::B, dest: Register::L }),
            [0b01110000, ..] => (1, Mov { src: Register::B, dest: Register::Mem }),
            [0b01111001, ..] => (1, Mov { src: Register::C, dest: Register::A }),
            [0b01000001, ..] => (1, Mov { src: Register::C, dest: Register::B }),
            [0b01001001, ..] => (1, Mov { src: Register::C, dest: Register::C }),
            [0b01010001, ..] => (1, Mov { src: Register::C, dest: Register::D }),
            [0b01011001, ..] => (1, Mov { src: Register::C, dest: Register::E }),
            [0b01100001, ..] => (1, Mov { src: Register::C, dest: Register::H }),
            [0b01101001, ..] => (1, Mov { src: Register::C, dest: Register::L }),
            [0b01110001, ..] => (1, Mov { src: Register::C, dest: Register::Mem }),
            [0b01111010, ..] => (1, Mov { src: Register::D, dest: Register::A }),
            [0b01000010, ..] => (1, Mov { src: Register::D, dest: Register::B }),
            [0b01001010, ..] => (1, Mov { src: Register::D, dest: Register::C }),
            [0b01010010, ..] => (1, Mov { src: Register::D, dest: Register::D }),
            [0b01011010, ..] => (1, Mov { src: Register::D, dest: Register::E }),
            [0b01100010, ..] => (1, Mov { src: Register::D, dest: Register::H }),
            [0b01101010, ..] => (1, Mov { src: Register::D, dest: Register::L }),
            [0b01110010, ..] => (1, Mov { src: Register::D, dest: Register::Mem }),
            [0b01111011, ..] => (1, Mov { src: Register::E, dest: Register::A }),
            [0b01000011, ..] => (1, Mov { src: Register::E, dest: Register::B }),
            [0b01001011, ..] => (1, Mov { src: Register::E, dest: Register::C }),
            [0b01010011, ..] => (1, Mov { src: Register::E, dest: Register::D }),
            [0b01011011, ..] => (1, Mov { src: Register::E, dest: Register::E }),
            [0b01100011, ..] => (1, Mov { src: Register::E, dest: Register::H }),
            [0b01101011, ..] => (1, Mov { src: Register::E, dest: Register::L }),
            [0b01110011, ..] => (1, Mov { src: Register::E, dest: Register::Mem }),
            [0b01111100, ..] => (1, Mov { src: Register::H, dest: Register::A }),
            [0b01000100, ..] => (1, Mov { src: Register::H, dest: Register::B }),
            [0b01001100, ..] => (1, Mov { src: Register::H, dest: Register::C }),
            [0b01010100, ..] => (1, Mov { src: Register::H, dest: Register::D }),
            [0b01011100, ..] => (1, Mov { src: Register::H, dest: Register::E }),
            [0b01100100, ..] => (1, Mov { src: Register::H, dest: Register::H }),
            [0b01101100, ..] => (1, Mov { src: Register::H, dest: Register::L }),
            [0b01110100, ..] => (1, Mov { src: Register::H, dest: Register::Mem }),
            [0b01111101, ..] => (1, Mov { src: Register::L, dest: Register::A }),
            [0b01000101, ..] => (1, Mov { src: Register::L, dest: Register::B }),
            [0b01001101, ..] => (1, Mov { src: Register::L, dest: Register::C }),
            [0b01010101, ..] => (1, Mov { src: Register::L, dest: Register::D }),
            [0b01011101, ..] => (1, Mov { src: Register::L, dest: Register::E }),
            [0b01100101, ..] => (1, Mov { src: Register::L, dest: Register::H }),
            [0b01101101, ..] => (1, Mov { src: Register::L, dest: Register::L }),
            [0b01110101, ..] => (1, Mov { src: Register::L, dest: Register::Mem }),
            [0b01111110, ..] => (1, Mov { src: Register::Mem, dest: Register::A }),
            [0b01000110, ..] => (1, Mov { src: Register::Mem, dest: Register::B }),
            [0b01001110, ..] => (1, Mov { src: Register::Mem, dest: Register::C }),
            [0b01010110, ..] => (1, Mov { src: Register::Mem, dest: Register::D }),
            [0b01011110, ..] => (1, Mov { src: Register::Mem, dest: Register::E }),
            [0b01100110, ..] => (1, Mov { src: Register::Mem, dest: Register::H }),
            [0b01101110, ..] => (1, Mov { src: Register::Mem, dest: Register::L }),
            // nb: MOV M, M is just HLT!

            // RET: 11001001
            [0b11001001, ..] => (1, Ret { condition: None }),

            // Rccc: 11CCC000
            [0b11_000_000, ..] => (1, Ret { condition: Some(NZ) }),
            [0b11_001_000, ..] => (1, Ret { condition: Some(Z) }),
            [0b11_010_000, ..] => (1, Ret { condition: Some(NC) }),
            [0b11_011_000, ..] => (1, Ret { condition: Some(C) }),
            [0b11_100_000, ..] => (1, Ret { condition: Some(PO) }),
            [0b11_101_000, ..] => (1, Ret { condition: Some(PE) }),
            [0b11_110_000, ..] => (1, Ret { condition: Some(P) }),
            [0b11_111_000, ..] => (1, Ret { condition: Some(M) }),

            // JMP: 11000011 lb hb
            [0b11000011, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: None }),

            // Jccc: 11CCC010 lb hb
            [0b11_000_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(NZ) }),
            [0b11_001_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(Z) }),
            [0b11_010_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(NC) }),
            [0b11_011_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(C) }),
            [0b11_100_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(PO) }),
            [0b11_101_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(PE) }),
            [0b11_110_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(P) }),
            [0b11_111_010, lo, hi, ..] => (3, Jmp { addr: lohi(lo, hi), condition: Some(M) }),

            // CALL: 11001101 lb hb
            [0b11001101, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: None }),

            // Cccc: 11CCC010 lb hb
            [0b11_000_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(NZ) }),
            [0b11_001_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(Z) }),
            [0b11_010_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(NC) }),
            [0b11_011_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(C) }),
            [0b11_100_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(PO) }),
            [0b11_101_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(PE) }),
            [0b11_110_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(P) }),
            [0b11_111_100, lo, hi, ..] => (3, Call { addr: lohi(lo, hi), condition: Some(M) }),

            // ADI: 11000110 x
            // ACI: 11001110 x
            // SUI: 11010110 x
            // SBI: 11011110 x
            [0b110_00_110, x, ..] => (2, Adi { value: x }),
            [0b110_01_110, x, ..] => (2, Aci { value: x }),
            [0b110_10_110, x, ..] => (2, Sui { value: x }),
            [0b110_11_110, x, ..] => (2, Sbi { value: x }),

            // ANI: 11100110 x
            // ORI: 11110110 x
            // XRI: 11101110 x
            // CPI: 11111110 x
            [0b111_00_110, x, ..] => (2, Ani { value: x }),
            [0b111_10_110, x, ..] => (2, Ori { value: x }),
            [0b111_01_110, x, ..] => (2, Xri { value: x }),
            [0b111_11_110, x, ..] => (2, Cpi { value: x }),

            // POP: 11RP0001
            [0b11_00_0001, ..] => (1, Pop { reg_pair: RegisterPair::BC }),
            [0b11_01_0001, ..] => (1, Pop { reg_pair: RegisterPair::DE }),
            [0b11_10_0001, ..] => (1, Pop { reg_pair: RegisterPair::HL }),
            [0b11_11_0001, ..] => (1, Pop { reg_pair: RegisterPair::PSW }),
            // PUSH: 11RP0101
            [0b11_00_0101, ..] => (1, Push { reg_pair: RegisterPair::BC }),
            [0b11_01_0101, ..] => (1, Push { reg_pair: RegisterPair::DE }),
            [0b11_10_0101, ..] => (1, Push { reg_pair: RegisterPair::HL }),
            [0b11_11_0101, ..] => (1, Push { reg_pair: RegisterPair::PSW }),

            // XTHL: 11100011
            [0b11100011, ..] => (1, Xthl),
            // XCHG: 11101011
            [0b11101011, ..] => (1, Xchg),

            // PCHL: 11101001
            [0b11101001, ..] => (1, Pchl),
            // SPHL: 11111001
            [0b11111001, ..] => (1, Sphl),

            // RST: 11vvv111
            [0b11_000_111, ..] => (1, Rst { index: 0 }),
            [0b11_001_111, ..] => (1, Rst { index: 1 }),
            [0b11_010_111, ..] => (1, Rst { index: 2 }),
            [0b11_011_111, ..] => (1, Rst { index: 3 }),
            [0b11_100_111, ..] => (1, Rst { index: 4 }),
            [0b11_101_111, ..] => (1, Rst { index: 5 }),
            [0b11_110_111, ..] => (1, Rst { index: 6 }),
            [0b11_111_111, ..] => (1, Rst { index: 7 }),

            [0x10, ..] => (1, Arhl),

            [0x08, ..] => (1, Dsub),
            [0x18, ..] => (1, Rdel),

            [0x28, imm, ..] => (2, Ldhi { imm } ),
            [0x38, imm, ..] => (2, Ldsi { imm } ),

            [0xd9, ..] => (1, Shlx),
            [0xed, ..] => (1, Lhlx),

            [0xdd, lo, hi, ..] => (3, Jnk { addr: lohi(lo, hi) }),
            [0xfd, lo, hi, ..] => (3, Jk  { addr: lohi(lo, hi) }),

            [0xcb, ..] => (1, Rstv),

            _ => {
                eprintln!("FAILED TO PARSE: {:x?}", buf.iter().take(3).collect::<Vec<_>>());
                todo!();
            }
        };

        (count, instr)
    }
}
