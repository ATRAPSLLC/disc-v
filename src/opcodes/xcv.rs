// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! CORE-V vendor extensions (XCValu, XCVbi, XCVbitmanip, XCVelw, XCVmac,
//! XCVmem, XCVsimd), for RV32 in the custom-0 to custom-3 opcode spaces.
//!
//! The encodings are from LLVM's RISCVInstrInfoXCV.td (Apache-2.0 WITH
//! LLVM-exception), which follows the CV32E40P user manual, and the operand
//! syntax is objdump's.

use super::{Codec, MaskedOpcode, Opcode, masked};
use crate::Extension;

pub(crate) static CUSTOM: [MaskedOpcode; 312] = [
    masked(
        0x5000302b,
        0xfff0707f,
        Opcode::new("cv.abs", Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6000302b,
        0xfff0707f,
        Opcode::new("cv.exths", Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6200302b,
        0xfff0707f,
        Opcode::new("cv.exthz", Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6400302b,
        0xfff0707f,
        Opcode::new("cv.extbs", Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x6600302b,
        0xfff0707f,
        Opcode::new("cv.extbz", Codec::R, "0,1")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5200302b,
        0xfe00707f,
        Opcode::new("cv.sle", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5400302b,
        0xfe00707f,
        Opcode::new("cv.sleu", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5600302b,
        0xfe00707f,
        Opcode::new("cv.min", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5800302b,
        0xfe00707f,
        Opcode::new("cv.minu", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5a00302b,
        0xfe00707f,
        Opcode::new("cv.max", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x5c00302b,
        0xfe00707f,
        Opcode::new("cv.maxu", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7400302b,
        0xfe00707f,
        Opcode::new("cv.clipr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7600302b,
        0xfe00707f,
        Opcode::new("cv.clipur", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8000302b,
        0xfe00707f,
        Opcode::new("cv.addnr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8200302b,
        0xfe00707f,
        Opcode::new("cv.addunr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8400302b,
        0xfe00707f,
        Opcode::new("cv.addrnr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8600302b,
        0xfe00707f,
        Opcode::new("cv.addurnr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8800302b,
        0xfe00707f,
        Opcode::new("cv.subnr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8a00302b,
        0xfe00707f,
        Opcode::new("cv.subunr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8c00302b,
        0xfe00707f,
        Opcode::new("cv.subrnr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8e00302b,
        0xfe00707f,
        Opcode::new("cv.suburnr", Codec::R, "0,1,2")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7000302b,
        0xfe00707f,
        Opcode::new("cv.clip", Codec::ISh5, "0,1,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x7200302b,
        0xfe00707f,
        Opcode::new("cv.clipu", Codec::ISh5, "0,1,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x0000205b,
        0xc000707f,
        Opcode::new("cv.addn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x4000205b,
        0xc000707f,
        Opcode::new("cv.addun", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8000205b,
        0xc000707f,
        Opcode::new("cv.addrn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0xc000205b,
        0xc000707f,
        Opcode::new("cv.addurn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x0000305b,
        0xc000707f,
        Opcode::new("cv.subn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x4000305b,
        0xc000707f,
        Opcode::new("cv.subun", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x8000305b,
        0xc000707f,
        Opcode::new("cv.subrn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0xc000305b,
        0xc000707f,
        Opcode::new("cv.suburn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCValu])
            .rv32_only(),
    ),
    masked(
        0x0000600b,
        0x0000707f,
        Opcode::new("cv.beqimm", Codec::BImm5, "1,j,o")
            .requires(&[Extension::XCVbi])
            .rv32_only(),
    ),
    masked(
        0x0000700b,
        0x0000707f,
        Opcode::new("cv.bneimm", Codec::BImm5, "1,j,o")
            .requires(&[Extension::XCVbi])
            .rv32_only(),
    ),
    masked(
        0x0000005b,
        0xc000707f,
        Opcode::new("cv.extract", Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4000005b,
        0xc000707f,
        Opcode::new("cv.extractu", Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x8000005b,
        0xc000707f,
        Opcode::new("cv.insert", Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x0000105b,
        0xc000707f,
        Opcode::new("cv.bclr", Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4000105b,
        0xc000707f,
        Opcode::new("cv.bset", Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0xc000105b,
        0xf800707f,
        Opcode::new("cv.bitrev", Codec::ImmPair5, "0,1,i,j")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3000302b,
        0xfe00707f,
        Opcode::new("cv.extractr", Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3200302b,
        0xfe00707f,
        Opcode::new("cv.extractur", Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3400302b,
        0xfe00707f,
        Opcode::new("cv.insertr", Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3800302b,
        0xfe00707f,
        Opcode::new("cv.bclrr", Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x3a00302b,
        0xfe00707f,
        Opcode::new("cv.bsetr", Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4000302b,
        0xfe00707f,
        Opcode::new("cv.ror", Codec::R, "0,1,2")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4200302b,
        0xfff0707f,
        Opcode::new("cv.ff1", Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4400302b,
        0xfff0707f,
        Opcode::new("cv.fl1", Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4600302b,
        0xfff0707f,
        Opcode::new("cv.clb", Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x4800302b,
        0xfff0707f,
        Opcode::new("cv.cnt", Codec::R, "0,1")
            .requires(&[Extension::XCVbitmanip])
            .rv32_only(),
    ),
    masked(
        0x0000300b,
        0x0000707f,
        Opcode::new("cv.elw", Codec::I, "0,i(1)")
            .requires(&[Extension::XCVelw])
            .rv32_only(),
    ),
    masked(
        0x9000302b,
        0xfe00707f,
        Opcode::new("cv.mac", Codec::R, "0,1,2")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x9200302b,
        0xfe00707f,
        Opcode::new("cv.msu", Codec::R, "0,1,2")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000405b,
        0xc000707f,
        Opcode::new("cv.mulsn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000405b,
        0xc000707f,
        Opcode::new("cv.mulhhsn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000405b,
        0xc000707f,
        Opcode::new("cv.mulsrn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000405b,
        0xc000707f,
        Opcode::new("cv.mulhhsrn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000505b,
        0xc000707f,
        Opcode::new("cv.mulun", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000505b,
        0xc000707f,
        Opcode::new("cv.mulhhun", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000505b,
        0xc000707f,
        Opcode::new("cv.mulurn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000505b,
        0xc000707f,
        Opcode::new("cv.mulhhurn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000605b,
        0xc000707f,
        Opcode::new("cv.macsn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000605b,
        0xc000707f,
        Opcode::new("cv.machhsn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000605b,
        0xc000707f,
        Opcode::new("cv.macsrn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000605b,
        0xc000707f,
        Opcode::new("cv.machhsrn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000705b,
        0xc000707f,
        Opcode::new("cv.macun", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x4000705b,
        0xc000707f,
        Opcode::new("cv.machhun", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x8000705b,
        0xc000707f,
        Opcode::new("cv.macurn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0xc000705b,
        0xc000707f,
        Opcode::new("cv.machhurn", Codec::RImm5, "0,1,2,i")
            .requires(&[Extension::XCVmac])
            .rv32_only(),
    ),
    masked(
        0x0000000b,
        0x0000707f,
        Opcode::new("cv.lb", Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000100b,
        0x0000707f,
        Opcode::new("cv.lh", Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000200b,
        0x0000707f,
        Opcode::new("cv.lw", Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000400b,
        0x0000707f,
        Opcode::new("cv.lbu", Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000500b,
        0x0000707f,
        Opcode::new("cv.lhu", Codec::I, "0,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000302b,
        0xfe00707f,
        Opcode::new("cv.lb", Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1000302b,
        0xfe00707f,
        Opcode::new("cv.lbu", Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0200302b,
        0xfe00707f,
        Opcode::new("cv.lh", Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1200302b,
        0xfe00707f,
        Opcode::new("cv.lhu", Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0400302b,
        0xfe00707f,
        Opcode::new("cv.lw", Codec::R, "0,(1),2")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0800302b,
        0xfe00707f,
        Opcode::new("cv.lb", Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1800302b,
        0xfe00707f,
        Opcode::new("cv.lbu", Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0a00302b,
        0xfe00707f,
        Opcode::new("cv.lh", Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x1a00302b,
        0xfe00707f,
        Opcode::new("cv.lhu", Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0c00302b,
        0xfe00707f,
        Opcode::new("cv.lw", Codec::R, "0,2(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000002b,
        0x0000707f,
        Opcode::new("cv.sb", Codec::S, "2,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000102b,
        0x0000707f,
        Opcode::new("cv.sh", Codec::S, "2,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000202b,
        0x0000707f,
        Opcode::new("cv.sw", Codec::S, "2,(1),i")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2000302b,
        0xfe00707f,
        Opcode::new("cv.sb", Codec::R, "2,(1),0")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2200302b,
        0xfe00707f,
        Opcode::new("cv.sh", Codec::R, "2,(1),0")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2400302b,
        0xfe00707f,
        Opcode::new("cv.sw", Codec::R, "2,(1),0")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2800302b,
        0xfe00707f,
        Opcode::new("cv.sb", Codec::R, "2,0(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2a00302b,
        0xfe00707f,
        Opcode::new("cv.sh", Codec::R, "2,0(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x2c00302b,
        0xfe00707f,
        Opcode::new("cv.sw", Codec::R, "2,0(1)")
            .requires(&[Extension::XCVmem])
            .rv32_only(),
    ),
    masked(
        0x0000007b,
        0xfe00707f,
        Opcode::new("cv.add.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000107b,
        0xfe00707f,
        Opcode::new("cv.add.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000407b,
        0xfe00707f,
        Opcode::new("cv.add.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000507b,
        0xfe00707f,
        Opcode::new("cv.add.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000607b,
        0xfc00707f,
        Opcode::new("cv.add.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0000707b,
        0xfc00707f,
        Opcode::new("cv.add.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800007b,
        0xfe00707f,
        Opcode::new("cv.sub.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800107b,
        0xfe00707f,
        Opcode::new("cv.sub.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800407b,
        0xfe00707f,
        Opcode::new("cv.sub.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800507b,
        0xfe00707f,
        Opcode::new("cv.sub.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800607b,
        0xfc00707f,
        Opcode::new("cv.sub.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0800707b,
        0xfc00707f,
        Opcode::new("cv.sub.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000007b,
        0xfe00707f,
        Opcode::new("cv.avg.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000107b,
        0xfe00707f,
        Opcode::new("cv.avg.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000407b,
        0xfe00707f,
        Opcode::new("cv.avg.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000507b,
        0xfe00707f,
        Opcode::new("cv.avg.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000607b,
        0xfc00707f,
        Opcode::new("cv.avg.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1000707b,
        0xfc00707f,
        Opcode::new("cv.avg.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800007b,
        0xfe00707f,
        Opcode::new("cv.avgu.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800107b,
        0xfe00707f,
        Opcode::new("cv.avgu.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800407b,
        0xfe00707f,
        Opcode::new("cv.avgu.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800507b,
        0xfe00707f,
        Opcode::new("cv.avgu.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800607b,
        0xfc00707f,
        Opcode::new("cv.avgu.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1800707b,
        0xfc00707f,
        Opcode::new("cv.avgu.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000007b,
        0xfe00707f,
        Opcode::new("cv.min.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000107b,
        0xfe00707f,
        Opcode::new("cv.min.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000407b,
        0xfe00707f,
        Opcode::new("cv.min.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000507b,
        0xfe00707f,
        Opcode::new("cv.min.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000607b,
        0xfc00707f,
        Opcode::new("cv.min.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2000707b,
        0xfc00707f,
        Opcode::new("cv.min.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800007b,
        0xfe00707f,
        Opcode::new("cv.minu.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800107b,
        0xfe00707f,
        Opcode::new("cv.minu.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800407b,
        0xfe00707f,
        Opcode::new("cv.minu.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800507b,
        0xfe00707f,
        Opcode::new("cv.minu.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800607b,
        0xfc00707f,
        Opcode::new("cv.minu.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2800707b,
        0xfc00707f,
        Opcode::new("cv.minu.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000007b,
        0xfe00707f,
        Opcode::new("cv.max.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000107b,
        0xfe00707f,
        Opcode::new("cv.max.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000407b,
        0xfe00707f,
        Opcode::new("cv.max.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000507b,
        0xfe00707f,
        Opcode::new("cv.max.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000607b,
        0xfc00707f,
        Opcode::new("cv.max.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3000707b,
        0xfc00707f,
        Opcode::new("cv.max.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800007b,
        0xfe00707f,
        Opcode::new("cv.maxu.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800107b,
        0xfe00707f,
        Opcode::new("cv.maxu.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800407b,
        0xfe00707f,
        Opcode::new("cv.maxu.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800507b,
        0xfe00707f,
        Opcode::new("cv.maxu.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800607b,
        0xfc00707f,
        Opcode::new("cv.maxu.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3800707b,
        0xfc00707f,
        Opcode::new("cv.maxu.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000007b,
        0xfe00707f,
        Opcode::new("cv.srl.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000107b,
        0xfe00707f,
        Opcode::new("cv.srl.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000407b,
        0xfe00707f,
        Opcode::new("cv.srl.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000507b,
        0xfe00707f,
        Opcode::new("cv.srl.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000607b,
        0xfc00707f,
        Opcode::new("cv.srl.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4000707b,
        0xfc00707f,
        Opcode::new("cv.srl.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800007b,
        0xfe00707f,
        Opcode::new("cv.sra.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800107b,
        0xfe00707f,
        Opcode::new("cv.sra.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800407b,
        0xfe00707f,
        Opcode::new("cv.sra.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800507b,
        0xfe00707f,
        Opcode::new("cv.sra.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800607b,
        0xfc00707f,
        Opcode::new("cv.sra.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4800707b,
        0xfc00707f,
        Opcode::new("cv.sra.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000007b,
        0xfe00707f,
        Opcode::new("cv.sll.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000107b,
        0xfe00707f,
        Opcode::new("cv.sll.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000407b,
        0xfe00707f,
        Opcode::new("cv.sll.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000507b,
        0xfe00707f,
        Opcode::new("cv.sll.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000607b,
        0xfc00707f,
        Opcode::new("cv.sll.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5000707b,
        0xfc00707f,
        Opcode::new("cv.sll.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800007b,
        0xfe00707f,
        Opcode::new("cv.or.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800107b,
        0xfe00707f,
        Opcode::new("cv.or.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800407b,
        0xfe00707f,
        Opcode::new("cv.or.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800507b,
        0xfe00707f,
        Opcode::new("cv.or.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800607b,
        0xfc00707f,
        Opcode::new("cv.or.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5800707b,
        0xfc00707f,
        Opcode::new("cv.or.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000007b,
        0xfe00707f,
        Opcode::new("cv.xor.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000107b,
        0xfe00707f,
        Opcode::new("cv.xor.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000407b,
        0xfe00707f,
        Opcode::new("cv.xor.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000507b,
        0xfe00707f,
        Opcode::new("cv.xor.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000607b,
        0xfc00707f,
        Opcode::new("cv.xor.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6000707b,
        0xfc00707f,
        Opcode::new("cv.xor.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800007b,
        0xfe00707f,
        Opcode::new("cv.and.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800107b,
        0xfe00707f,
        Opcode::new("cv.and.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800407b,
        0xfe00707f,
        Opcode::new("cv.and.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800507b,
        0xfe00707f,
        Opcode::new("cv.and.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800607b,
        0xfc00707f,
        Opcode::new("cv.and.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6800707b,
        0xfc00707f,
        Opcode::new("cv.and.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000007b,
        0xfe00707f,
        Opcode::new("cv.dotup.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000107b,
        0xfe00707f,
        Opcode::new("cv.dotup.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000407b,
        0xfe00707f,
        Opcode::new("cv.dotup.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000507b,
        0xfe00707f,
        Opcode::new("cv.dotup.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000607b,
        0xfc00707f,
        Opcode::new("cv.dotup.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8000707b,
        0xfc00707f,
        Opcode::new("cv.dotup.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800007b,
        0xfe00707f,
        Opcode::new("cv.dotusp.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800107b,
        0xfe00707f,
        Opcode::new("cv.dotusp.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800407b,
        0xfe00707f,
        Opcode::new("cv.dotusp.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800507b,
        0xfe00707f,
        Opcode::new("cv.dotusp.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800607b,
        0xfc00707f,
        Opcode::new("cv.dotusp.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x8800707b,
        0xfc00707f,
        Opcode::new("cv.dotusp.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000007b,
        0xfe00707f,
        Opcode::new("cv.dotsp.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000107b,
        0xfe00707f,
        Opcode::new("cv.dotsp.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000407b,
        0xfe00707f,
        Opcode::new("cv.dotsp.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000507b,
        0xfe00707f,
        Opcode::new("cv.dotsp.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000607b,
        0xfc00707f,
        Opcode::new("cv.dotsp.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9000707b,
        0xfc00707f,
        Opcode::new("cv.dotsp.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800007b,
        0xfe00707f,
        Opcode::new("cv.sdotup.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800107b,
        0xfe00707f,
        Opcode::new("cv.sdotup.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800407b,
        0xfe00707f,
        Opcode::new("cv.sdotup.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800507b,
        0xfe00707f,
        Opcode::new("cv.sdotup.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800607b,
        0xfc00707f,
        Opcode::new("cv.sdotup.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x9800707b,
        0xfc00707f,
        Opcode::new("cv.sdotup.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000007b,
        0xfe00707f,
        Opcode::new("cv.sdotusp.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000107b,
        0xfe00707f,
        Opcode::new("cv.sdotusp.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000407b,
        0xfe00707f,
        Opcode::new("cv.sdotusp.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000507b,
        0xfe00707f,
        Opcode::new("cv.sdotusp.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000607b,
        0xfc00707f,
        Opcode::new("cv.sdotusp.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa000707b,
        0xfc00707f,
        Opcode::new("cv.sdotusp.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800007b,
        0xfe00707f,
        Opcode::new("cv.sdotsp.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800107b,
        0xfe00707f,
        Opcode::new("cv.sdotsp.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800407b,
        0xfe00707f,
        Opcode::new("cv.sdotsp.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800507b,
        0xfe00707f,
        Opcode::new("cv.sdotsp.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800607b,
        0xfc00707f,
        Opcode::new("cv.sdotsp.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xa800707b,
        0xfc00707f,
        Opcode::new("cv.sdotsp.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7000007b,
        0xfff0707f,
        Opcode::new("cv.abs.h", Codec::R, "0,1")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7000107b,
        0xfff0707f,
        Opcode::new("cv.abs.b", Codec::R, "0,1")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800007b,
        0xfc00707f,
        Opcode::new("cv.extract.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800107b,
        0xfc00707f,
        Opcode::new("cv.extract.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800207b,
        0xfc00707f,
        Opcode::new("cv.extractu.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800307b,
        0xfc00707f,
        Opcode::new("cv.extractu.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800407b,
        0xfc00707f,
        Opcode::new("cv.insert.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xb800507b,
        0xfc00707f,
        Opcode::new("cv.insert.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000007b,
        0xfe00707f,
        Opcode::new("cv.shuffle.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000107b,
        0xfe00707f,
        Opcode::new("cv.shuffle.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000607b,
        0xfc00707f,
        Opcode::new("cv.shuffle.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc000707b,
        0xfc00707f,
        Opcode::new("cv.shufflei0.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xc800707b,
        0xfc00707f,
        Opcode::new("cv.shufflei1.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xd000707b,
        0xfc00707f,
        Opcode::new("cv.shufflei2.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xd800707b,
        0xfc00707f,
        Opcode::new("cv.shufflei3.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xe000007b,
        0xfe00707f,
        Opcode::new("cv.shuffle2.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xe000107b,
        0xfe00707f,
        Opcode::new("cv.shuffle2.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xf000007b,
        0xfe00707f,
        Opcode::new("cv.pack", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xf200007b,
        0xfe00707f,
        Opcode::new("cv.pack.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xf800107b,
        0xfe00707f,
        Opcode::new("cv.packlo.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0xfa00107b,
        0xfe00707f,
        Opcode::new("cv.packhi.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400007b,
        0xfe00707f,
        Opcode::new("cv.cmpeq.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400107b,
        0xfe00707f,
        Opcode::new("cv.cmpeq.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400407b,
        0xfe00707f,
        Opcode::new("cv.cmpeq.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400507b,
        0xfe00707f,
        Opcode::new("cv.cmpeq.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400607b,
        0xfc00707f,
        Opcode::new("cv.cmpeq.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0400707b,
        0xfc00707f,
        Opcode::new("cv.cmpeq.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00007b,
        0xfe00707f,
        Opcode::new("cv.cmpne.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00107b,
        0xfe00707f,
        Opcode::new("cv.cmpne.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00407b,
        0xfe00707f,
        Opcode::new("cv.cmpne.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00507b,
        0xfe00707f,
        Opcode::new("cv.cmpne.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00607b,
        0xfc00707f,
        Opcode::new("cv.cmpne.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x0c00707b,
        0xfc00707f,
        Opcode::new("cv.cmpne.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400007b,
        0xfe00707f,
        Opcode::new("cv.cmpgt.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400107b,
        0xfe00707f,
        Opcode::new("cv.cmpgt.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400407b,
        0xfe00707f,
        Opcode::new("cv.cmpgt.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400507b,
        0xfe00707f,
        Opcode::new("cv.cmpgt.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400607b,
        0xfc00707f,
        Opcode::new("cv.cmpgt.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1400707b,
        0xfc00707f,
        Opcode::new("cv.cmpgt.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00007b,
        0xfe00707f,
        Opcode::new("cv.cmpge.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00107b,
        0xfe00707f,
        Opcode::new("cv.cmpge.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00407b,
        0xfe00707f,
        Opcode::new("cv.cmpge.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00507b,
        0xfe00707f,
        Opcode::new("cv.cmpge.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00607b,
        0xfc00707f,
        Opcode::new("cv.cmpge.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x1c00707b,
        0xfc00707f,
        Opcode::new("cv.cmpge.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400007b,
        0xfe00707f,
        Opcode::new("cv.cmplt.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400107b,
        0xfe00707f,
        Opcode::new("cv.cmplt.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400407b,
        0xfe00707f,
        Opcode::new("cv.cmplt.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400507b,
        0xfe00707f,
        Opcode::new("cv.cmplt.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400607b,
        0xfc00707f,
        Opcode::new("cv.cmplt.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2400707b,
        0xfc00707f,
        Opcode::new("cv.cmplt.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00007b,
        0xfe00707f,
        Opcode::new("cv.cmple.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00107b,
        0xfe00707f,
        Opcode::new("cv.cmple.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00407b,
        0xfe00707f,
        Opcode::new("cv.cmple.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00507b,
        0xfe00707f,
        Opcode::new("cv.cmple.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00607b,
        0xfc00707f,
        Opcode::new("cv.cmple.sci.h", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x2c00707b,
        0xfc00707f,
        Opcode::new("cv.cmple.sci.b", Codec::Simm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400007b,
        0xfe00707f,
        Opcode::new("cv.cmpgtu.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400107b,
        0xfe00707f,
        Opcode::new("cv.cmpgtu.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400407b,
        0xfe00707f,
        Opcode::new("cv.cmpgtu.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400507b,
        0xfe00707f,
        Opcode::new("cv.cmpgtu.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400607b,
        0xfc00707f,
        Opcode::new("cv.cmpgtu.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3400707b,
        0xfc00707f,
        Opcode::new("cv.cmpgtu.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00007b,
        0xfe00707f,
        Opcode::new("cv.cmpgeu.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00107b,
        0xfe00707f,
        Opcode::new("cv.cmpgeu.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00407b,
        0xfe00707f,
        Opcode::new("cv.cmpgeu.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00507b,
        0xfe00707f,
        Opcode::new("cv.cmpgeu.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00607b,
        0xfc00707f,
        Opcode::new("cv.cmpgeu.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x3c00707b,
        0xfc00707f,
        Opcode::new("cv.cmpgeu.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400007b,
        0xfe00707f,
        Opcode::new("cv.cmpltu.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400107b,
        0xfe00707f,
        Opcode::new("cv.cmpltu.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400407b,
        0xfe00707f,
        Opcode::new("cv.cmpltu.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400507b,
        0xfe00707f,
        Opcode::new("cv.cmpltu.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400607b,
        0xfc00707f,
        Opcode::new("cv.cmpltu.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4400707b,
        0xfc00707f,
        Opcode::new("cv.cmpltu.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00007b,
        0xfe00707f,
        Opcode::new("cv.cmpleu.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00107b,
        0xfe00707f,
        Opcode::new("cv.cmpleu.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00407b,
        0xfe00707f,
        Opcode::new("cv.cmpleu.sc.h", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00507b,
        0xfe00707f,
        Opcode::new("cv.cmpleu.sc.b", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00607b,
        0xfc00707f,
        Opcode::new("cv.cmpleu.sci.h", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x4c00707b,
        0xfc00707f,
        Opcode::new("cv.cmpleu.sci.b", Codec::Uimm6, "0,1,i")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400007b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.r", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600007b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.i", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400207b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.r.div2", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600207b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.i.div2", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400407b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.r.div4", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600407b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.i.div4", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5400607b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.r.div8", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5600607b,
        0xfe00707f,
        Opcode::new("cv.cplxmul.i.div8", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x5c00007b,
        0xfff0707f,
        Opcode::new("cv.cplxconj", Codec::R, "0,1")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400007b,
        0xfe00707f,
        Opcode::new("cv.subrotmj", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400207b,
        0xfe00707f,
        Opcode::new("cv.subrotmj.div2", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400407b,
        0xfe00707f,
        Opcode::new("cv.subrotmj.div4", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6400607b,
        0xfe00707f,
        Opcode::new("cv.subrotmj.div8", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6c00207b,
        0xfe00707f,
        Opcode::new("cv.add.div2", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7400207b,
        0xfe00707f,
        Opcode::new("cv.sub.div2", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6c00407b,
        0xfe00707f,
        Opcode::new("cv.add.div4", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7400407b,
        0xfe00707f,
        Opcode::new("cv.sub.div4", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x6c00607b,
        0xfe00707f,
        Opcode::new("cv.add.div8", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
    masked(
        0x7400607b,
        0xfe00707f,
        Opcode::new("cv.sub.div8", Codec::R, "0,1,2")
            .requires(&[Extension::XCVsimd])
            .rv32_only(),
    ),
];
