// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! T-Head vendor extensions (XTheadBa, XTheadBb, XTheadBs, XTheadCmo,
//! XTheadCondMov, XTheadFMemIdx, XTheadFmv, XTheadInt, XTheadMac,
//! XTheadMemIdx, XTheadMemPair, XTheadSync), all in the custom-0 opcode
//! space.
//!
//! The encodings are from the T-Head extension specification
//! (<https://github.com/XUANTIE-RV/thead-extension-spec>, commit
//! d16533b5a9cd790c369d849ddf9a48159a5a235c, Apache-2.0), with the RV64-only
//! instructions as in LLVM, and the operand syntax is objdump's.

use super::Constraint::*;
use super::{Codec, MaskedOpcode, Opcode, masked};
use crate::Extension;

pub(crate) static CUSTOM_0: [MaskedOpcode; 106] = [
    masked(
        0x0000100b,
        0xf800707f,
        Opcode::new("th.addsl", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadBa]),
    ),
    masked(
        0x1000100b,
        0xfc00707f,
        Opcode::new("th.srri", Codec::IUimm6, "0,1,i").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x1400100b,
        0xfe00707f,
        Opcode::new("th.srriw", Codec::ISh5, "0,1,i")
            .requires(&[Extension::XTheadBb])
            .rv64(),
    ),
    masked(
        0x0000200b,
        0x0000707f,
        Opcode::new("th.ext", Codec::ImmPair6, "0,1,i,j").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x0000300b,
        0x0000707f,
        Opcode::new("th.extu", Codec::ImmPair6, "0,1,i,j").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8400100b,
        0xfff0707f,
        Opcode::new("th.ff0", Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8600100b,
        0xfff0707f,
        Opcode::new("th.ff1", Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8200100b,
        0xfff0707f,
        Opcode::new("th.rev", Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x8000100b,
        0xfff0707f,
        Opcode::new("th.tstnbz", Codec::R, "0,1").requires(&[Extension::XTheadBb]),
    ),
    masked(
        0x9000100b,
        0xfff0707f,
        Opcode::new("th.revw", Codec::R, "0,1")
            .requires(&[Extension::XTheadBb])
            .rv64(),
    ),
    masked(
        0x8800100b,
        0xfc00707f,
        Opcode::new("th.tst", Codec::IUimm6, "0,1,i").requires(&[Extension::XTheadBs]),
    ),
    masked(
        0x0210000b,
        0xfff07fff,
        Opcode::new("th.dcache.csw", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0220000b,
        0xfff07fff,
        Opcode::new("th.dcache.isw", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0230000b,
        0xfff07fff,
        Opcode::new("th.dcache.cisw", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0240000b,
        0xfff07fff,
        Opcode::new("th.dcache.cval1", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0250000b,
        0xfff07fff,
        Opcode::new("th.dcache.cva", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0260000b,
        0xfff07fff,
        Opcode::new("th.dcache.iva", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0270000b,
        0xfff07fff,
        Opcode::new("th.dcache.civa", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0280000b,
        0xfff07fff,
        Opcode::new("th.dcache.cpal1", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0290000b,
        0xfff07fff,
        Opcode::new("th.dcache.cpa", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x02a0000b,
        0xfff07fff,
        Opcode::new("th.dcache.ipa", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x02b0000b,
        0xfff07fff,
        Opcode::new("th.dcache.cipa", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0300000b,
        0xfff07fff,
        Opcode::new("th.icache.iva", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0380000b,
        0xfff07fff,
        Opcode::new("th.icache.ipa", Codec::R, "1").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0010000b,
        0xffffffff,
        Opcode::new("th.dcache.call", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0020000b,
        0xffffffff,
        Opcode::new("th.dcache.iall", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0030000b,
        0xffffffff,
        Opcode::new("th.dcache.ciall", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0100000b,
        0xffffffff,
        Opcode::new("th.icache.iall", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0110000b,
        0xffffffff,
        Opcode::new("th.icache.ialls", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0150000b,
        0xffffffff,
        Opcode::new("th.l2cache.call", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0160000b,
        0xffffffff,
        Opcode::new("th.l2cache.iall", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x0170000b,
        0xffffffff,
        Opcode::new("th.l2cache.ciall", Codec::None, "").requires(&[Extension::XTheadCmo]),
    ),
    masked(
        0x4000100b,
        0xfe00707f,
        Opcode::new("th.mveqz", Codec::R, "0,1,2").requires(&[Extension::XTheadCondMov]),
    ),
    masked(
        0x4200100b,
        0xfe00707f,
        Opcode::new("th.mvnez", Codec::R, "0,1,2").requires(&[Extension::XTheadCondMov]),
    ),
    masked(
        0x4000600b,
        0xf800707f,
        Opcode::new("th.flrw", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x6000600b,
        0xf800707f,
        Opcode::new("th.flrd", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0x5000600b,
        0xf800707f,
        Opcode::new("th.flurw", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x7000600b,
        0xf800707f,
        Opcode::new("th.flurd", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0x4000700b,
        0xf800707f,
        Opcode::new("th.fsrw", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x6000700b,
        0xf800707f,
        Opcode::new("th.fsrd", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0x5000700b,
        0xf800707f,
        Opcode::new("th.fsurw", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::F])
            .f_registers(),
    ),
    masked(
        0x7000700b,
        0xf800707f,
        Opcode::new("th.fsurd", Codec::RImm2, "3,1,2,i")
            .requires(&[Extension::XTheadFMemIdx, Extension::D])
            .f_registers(),
    ),
    masked(
        0xa000100b,
        0xfff0707f,
        Opcode::new("th.fmv.hw.x", Codec::R, "3,1")
            .requires(&[Extension::XTheadFmv, Extension::D])
            .rv32_only()
            .f_registers(),
    ),
    masked(
        0xc000100b,
        0xfff0707f,
        Opcode::new("th.fmv.x.hw", Codec::R, "0,4")
            .requires(&[Extension::XTheadFmv, Extension::D])
            .rv32_only()
            .f_registers(),
    ),
    masked(
        0x0040000b,
        0xffffffff,
        Opcode::new("th.ipush", Codec::None, "").requires(&[Extension::XTheadInt]),
    ),
    masked(
        0x0050000b,
        0xffffffff,
        Opcode::new("th.ipop", Codec::None, "").requires(&[Extension::XTheadInt]),
    ),
    masked(
        0x2000100b,
        0xfe00707f,
        Opcode::new("th.mula", Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2200100b,
        0xfe00707f,
        Opcode::new("th.muls", Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2800100b,
        0xfe00707f,
        Opcode::new("th.mulah", Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2a00100b,
        0xfe00707f,
        Opcode::new("th.mulsh", Codec::R, "0,1,2").requires(&[Extension::XTheadMac]),
    ),
    masked(
        0x2400100b,
        0xfe00707f,
        Opcode::new("th.mulaw", Codec::R, "0,1,2")
            .requires(&[Extension::XTheadMac])
            .rv64(),
    ),
    masked(
        0x2600100b,
        0xfe00707f,
        Opcode::new("th.mulsw", Codec::R, "0,1,2")
            .requires(&[Extension::XTheadMac])
            .rv64(),
    ),
    masked(
        0x1800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lbia", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x0800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lbib", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x9800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lbuia", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x8800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lbuib", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x3800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lhia", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x2800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lhib", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0xb800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lhuia", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0xa800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lhuib", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x5800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lwia", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0x4800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lwib", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
        },
    ),
    masked(
        0xd800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lwuia", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0xc800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.lwuib", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0x7800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.ldia", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0x6800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1],
            ..Opcode::new("th.ldib", Codec::ImmUpdate, "0,(1),i,j")
                .requires(&[Extension::XTheadMemIdx])
                .rv64()
        },
    ),
    masked(
        0x1800500b,
        0xf800707f,
        Opcode::new("th.sbia", Codec::ImmUpdate, "0,(1),i,j").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x0800500b,
        0xf800707f,
        Opcode::new("th.sbib", Codec::ImmUpdate, "0,(1),i,j").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x3800500b,
        0xf800707f,
        Opcode::new("th.shia", Codec::ImmUpdate, "0,(1),i,j").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x2800500b,
        0xf800707f,
        Opcode::new("th.shib", Codec::ImmUpdate, "0,(1),i,j").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x5800500b,
        0xf800707f,
        Opcode::new("th.swia", Codec::ImmUpdate, "0,(1),i,j").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x4800500b,
        0xf800707f,
        Opcode::new("th.swib", Codec::ImmUpdate, "0,(1),i,j").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x7800500b,
        0xf800707f,
        Opcode::new("th.sdia", Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x6800500b,
        0xf800707f,
        Opcode::new("th.sdib", Codec::ImmUpdate, "0,(1),i,j")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x0000400b,
        0xf800707f,
        Opcode::new("th.lrb", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x8000400b,
        0xf800707f,
        Opcode::new("th.lrbu", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x1000400b,
        0xf800707f,
        Opcode::new("th.lurb", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x9000400b,
        0xf800707f,
        Opcode::new("th.lurbu", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x2000400b,
        0xf800707f,
        Opcode::new("th.lrh", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0xa000400b,
        0xf800707f,
        Opcode::new("th.lrhu", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x3000400b,
        0xf800707f,
        Opcode::new("th.lurh", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0xb000400b,
        0xf800707f,
        Opcode::new("th.lurhu", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x4000400b,
        0xf800707f,
        Opcode::new("th.lrw", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x5000400b,
        0xf800707f,
        Opcode::new("th.lurw", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0xc000400b,
        0xf800707f,
        Opcode::new("th.lrwu", Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0xd000400b,
        0xf800707f,
        Opcode::new("th.lurwu", Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x6000400b,
        0xf800707f,
        Opcode::new("th.lrd", Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x7000400b,
        0xf800707f,
        Opcode::new("th.lurd", Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x0000500b,
        0xf800707f,
        Opcode::new("th.srb", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x1000500b,
        0xf800707f,
        Opcode::new("th.surb", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x2000500b,
        0xf800707f,
        Opcode::new("th.srh", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x3000500b,
        0xf800707f,
        Opcode::new("th.surh", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x4000500b,
        0xf800707f,
        Opcode::new("th.srw", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x5000500b,
        0xf800707f,
        Opcode::new("th.surw", Codec::RImm2, "0,1,2,i").requires(&[Extension::XTheadMemIdx]),
    ),
    masked(
        0x6000500b,
        0xf800707f,
        Opcode::new("th.srd", Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0x7000500b,
        0xf800707f,
        Opcode::new("th.surd", Codec::RImm2, "0,1,2,i")
            .requires(&[Extension::XTheadMemIdx])
            .rv64(),
    ),
    masked(
        0xe000400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1, Rs2EqRs1, RdEqRs2],
            ..Opcode::new("th.lwd", Codec::Pair, "0,2,(1),i,j")
                .requires(&[Extension::XTheadMemPair])
        },
    ),
    masked(
        0xf000400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1, Rs2EqRs1, RdEqRs2],
            ..Opcode::new("th.lwud", Codec::Pair, "0,2,(1),i,j")
                .requires(&[Extension::XTheadMemPair])
        },
    ),
    masked(
        0xf800400b,
        0xf800707f,
        Opcode {
            illegal_if: &[RdEqRs1, Rs2EqRs1, RdEqRs2],
            ..Opcode::new("th.ldd", Codec::Pair, "0,2,(1),i,j")
                .requires(&[Extension::XTheadMemPair])
                .rv64()
        },
    ),
    masked(
        0xe000500b,
        0xf800707f,
        Opcode::new("th.swd", Codec::Pair, "0,2,(1),i,j").requires(&[Extension::XTheadMemPair]),
    ),
    masked(
        0xf800500b,
        0xf800707f,
        Opcode::new("th.sdd", Codec::Pair, "0,2,(1),i,j")
            .requires(&[Extension::XTheadMemPair])
            .rv64(),
    ),
    masked(
        0x0400000b,
        0xfe007fff,
        Opcode::new("th.sfence.vmas", Codec::R, "1,2").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x0180000b,
        0xffffffff,
        Opcode::new("th.sync", Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x0190000b,
        0xffffffff,
        Opcode::new("th.sync.s", Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x01a0000b,
        0xffffffff,
        Opcode::new("th.sync.i", Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
    masked(
        0x01b0000b,
        0xffffffff,
        Opcode::new("th.sync.is", Codec::None, "").requires(&[Extension::XTheadSync]),
    ),
];
