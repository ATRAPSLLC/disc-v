// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.
//
// Derived from riscv-disassembler, Copyright (c) 2016-2017 Michael Clark
// and Copyright (c) 2017-2018 SiFive, Inc., under the MIT license; see NOTICE.

//! "C" extension: compressed instructions.

use super::Constraint::*;
use super::{Codec, Extra, Opcode, Pseudo, d, f, fmt, i, pseudo, system, zicbo};
use crate::Extension;
use crate::reg;

pub(crate) static C_ADDI4SPN: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        illegal_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new("c.addi4spn", Codec::Ciw4spn, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_FLD: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FLD), Some(&d::FLD), None],
        ..Extra::NONE
    },
    ..Opcode::new("c.fld", Codec::ClLd, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcd])
};
pub(crate) static C_LW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LW), Some(&i::LW), Some(&i::LW)],
        ..Extra::NONE
    },
    ..Opcode::new("c.lw", Codec::ClLw, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FLW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FLW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new("c.flw", Codec::ClLw, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcf])
};
pub(crate) static C_FSD: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FSD), Some(&d::FSD), None],
        ..Extra::NONE
    },
    ..Opcode::new("c.fsd", Codec::CsSd, fmt::FRS2_OFFSET_RS1).requires(&[Extension::Zcd])
};
pub(crate) static C_SW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SW), Some(&i::SW), Some(&i::SW)],
        ..Extra::NONE
    },
    ..Opcode::new("c.sw", Codec::CsSw, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FSW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FSW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new("c.fsw", Codec::CsSw, fmt::FRS2_OFFSET_RS1).requires(&[Extension::Zcf])
};
/// `c.nop` with a nonzero immediate, which is a HINT.
pub(crate) static C_NOP_HINT: Opcode =
    Opcode::new("c.nop", Codec::Ci, fmt::IMM).requires(&[Extension::Zca]);
pub(crate) static C_UNIMP: Opcode = Opcode {
    extra: &Extra {
        pseudo: &[Pseudo::new(&pseudo::UNIMP, &[])],
        ..Extra::NONE
    },
    ..Opcode::new("c.unimp", Codec::None, fmt::NONE).requires(&[Extension::Zca])
};
pub(crate) static C_NOP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        ..Extra::NONE
    },
    ..Opcode::new("c.nop", Codec::CiNone, fmt::NONE).requires(&[Extension::Zca])
};
pub(crate) static C_ADDI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        ..Extra::NONE
    },
    ..Opcode::new("c.addi", Codec::Ci, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_JAL: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JAL), None, None],
        ..Extra::NONE
    },
    ..Opcode::new("c.jal", Codec::CjJal, fmt::RD_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_LI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        hint_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.li", Codec::CiLi, fmt::RD_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_ADDI16SP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        illegal_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new("c.addi16sp", Codec::Ci16sp, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_LUI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LUI), Some(&i::LUI), Some(&i::LUI)],
        illegal_if: &[ImmEq(0)],
        hint_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.lui", Codec::CiLui, fmt::RD_UIMM).requires(&[Extension::Zca])
};
pub(crate) static C_SRLI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SRLI), Some(&i::SRLI), Some(&i::SRLI)],
        hint_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new("c.srli", Codec::CbSh6, fmt::RD_SHAMT).requires(&[Extension::Zca])
};
pub(crate) static C_SRAI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SRAI), Some(&i::SRAI), Some(&i::SRAI)],
        hint_if: &[ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new("c.srai", Codec::CbSh6, fmt::RD_SHAMT).requires(&[Extension::Zca])
};
pub(crate) static C_ANDI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ANDI), Some(&i::ANDI), Some(&i::ANDI)],
        ..Extra::NONE
    },
    ..Opcode::new("c.andi", Codec::CbImm, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_SUB: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SUB), Some(&i::SUB), Some(&i::SUB)],
        ..Extra::NONE
    },
    ..Opcode::new("c.sub", Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_XOR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::XOR), Some(&i::XOR), Some(&i::XOR)],
        ..Extra::NONE
    },
    ..Opcode::new("c.xor", Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_OR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::OR), Some(&i::OR), Some(&i::OR)],
        ..Extra::NONE
    },
    ..Opcode::new("c.or", Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_AND: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::AND), Some(&i::AND), Some(&i::AND)],
        ..Extra::NONE
    },
    ..Opcode::new("c.and", Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_SUBW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SUBW), Some(&i::SUBW), Some(&i::SUBW)],
        ..Extra::NONE
    },
    ..Opcode::new("c.subw", Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_ADDW: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDW), Some(&i::ADDW), Some(&i::ADDW)],
        ..Extra::NONE
    },
    ..Opcode::new("c.addw", Codec::Cs, fmt::RD_RS1_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_J: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JAL), Some(&i::JAL), Some(&i::JAL)],
        ..Extra::NONE
    },
    ..Opcode::new("c.j", Codec::Cj, fmt::RD_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_BEQZ: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::BEQ), Some(&i::BEQ), Some(&i::BEQ)],
        ..Extra::NONE
    },
    ..Opcode::new("c.beqz", Codec::Cb, fmt::RS1_RS2_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_BNEZ: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::BNE), Some(&i::BNE), Some(&i::BNE)],
        ..Extra::NONE
    },
    ..Opcode::new("c.bnez", Codec::Cb, fmt::RS1_RS2_OFFSET).requires(&[Extension::Zca])
};
pub(crate) static C_SLLI: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SLLI), Some(&i::SLLI), Some(&i::SLLI)],
        hint_if: &[RdEq(reg::ZERO), ImmEq(0)],
        ..Extra::NONE
    },
    ..Opcode::new("c.slli", Codec::CiSh6, fmt::RD_SHAMT).requires(&[Extension::Zca])
};
pub(crate) static C_FLDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FLD), Some(&d::FLD), Some(&d::FLD)],
        ..Extra::NONE
    },
    ..Opcode::new("c.fldsp", Codec::CiLdsp, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcd])
};
pub(crate) static C_LWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::LW), Some(&i::LW), Some(&i::LW)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.lwsp", Codec::CiLwsp, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FLWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FLW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new("c.flwsp", Codec::CiLwsp, fmt::FRD_OFFSET_RS1).requires(&[Extension::Zcf])
};
pub(crate) static C_JR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JALR), Some(&i::JALR), Some(&i::JALR)],
        illegal_if: &[Rs1Eq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.jr", Codec::CrJr, fmt::RS1).requires(&[Extension::Zca])
};
pub(crate) static C_MV: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADDI), Some(&i::ADDI), Some(&i::ADDI)],
        hint_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.mv", Codec::CrMv, fmt::RD_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_EBREAK: Opcode = Opcode {
    extra: &Extra {
        decompress: [
            Some(&system::EBREAK),
            Some(&system::EBREAK),
            Some(&system::EBREAK),
        ],
        ..Extra::NONE
    },
    ..Opcode::new("c.ebreak", Codec::CiNone, fmt::NONE).requires(&[Extension::Zca])
};
pub(crate) static C_JALR: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::JALR), Some(&i::JALR), Some(&i::JALR)],
        ..Extra::NONE
    },
    ..Opcode::new("c.jalr", Codec::CrJalr, fmt::RS1).requires(&[Extension::Zca])
};
pub(crate) static C_ADD: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::ADD), Some(&i::ADD), Some(&i::ADD)],
        hint_if: &[RdEq(reg::ZERO)],
        // Aliases of the HINT, which has rd=zero.
        pseudo: &[
            Pseudo::new(&zicbo::NTL_P1, &[Rs2Eq(2)]),
            Pseudo::new(&zicbo::NTL_PALL, &[Rs2Eq(3)]),
            Pseudo::new(&zicbo::NTL_S1, &[Rs2Eq(4)]),
            Pseudo::new(&zicbo::NTL_ALL, &[Rs2Eq(5)]),
        ],
        ..Extra::NONE
    },
    ..Opcode::new("c.add", Codec::Cr, fmt::RD_RS2).requires(&[Extension::Zca])
};
pub(crate) static C_FSDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&d::FSD), Some(&d::FSD), Some(&d::FSD)],
        ..Extra::NONE
    },
    ..Opcode::new("c.fsdsp", Codec::CssSdsp, fmt::FRS2_OFFSET_RS1).requires(&[Extension::Zcd])
};
pub(crate) static C_SWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&i::SW), Some(&i::SW), Some(&i::SW)],
        ..Extra::NONE
    },
    ..Opcode::new("c.swsp", Codec::CssSwsp, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_FSWSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [Some(&f::FSW), None, None],
        ..Extra::NONE
    },
    ..Opcode::new("c.fswsp", Codec::CssSwsp, fmt::FRS2_OFFSET_RS1).requires(&[Extension::Zcf])
};
pub(crate) static C_LD: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::LD), Some(&i::LD)],
        ..Extra::NONE
    },
    ..Opcode::new("c.ld", Codec::ClLd, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SD: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::SD), Some(&i::SD)],
        ..Extra::NONE
    },
    ..Opcode::new("c.sd", Codec::CsSd, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_ADDIW: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::ADDIW), Some(&i::ADDIW)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.addiw", Codec::Ci, fmt::RD_RS1_IMM).requires(&[Extension::Zca])
};
pub(crate) static C_LDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::LD), Some(&i::LD)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.ldsp", Codec::CiLdsp, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SDSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, Some(&i::SD), Some(&i::SD)],
        ..Extra::NONE
    },
    ..Opcode::new("c.sdsp", Codec::CssSdsp, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_LQ: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::LQ)],
        ..Extra::NONE
    },
    ..Opcode::new("c.lq", Codec::ClLq, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SQ: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::SQ)],
        ..Extra::NONE
    },
    ..Opcode::new("c.sq", Codec::CsSq, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_LQSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::LQ)],
        illegal_if: &[RdEq(reg::ZERO)],
        ..Extra::NONE
    },
    ..Opcode::new("c.lqsp", Codec::CiLqsp, fmt::RD_OFFSET_RS1).requires(&[Extension::Zca])
};
pub(crate) static C_SQSP: Opcode = Opcode {
    extra: &Extra {
        decompress: [None, None, Some(&i::SQ)],
        ..Extra::NONE
    },
    ..Opcode::new("c.sqsp", Codec::CssSqsp, fmt::RS2_OFFSET_RS1).requires(&[Extension::Zca])
};
