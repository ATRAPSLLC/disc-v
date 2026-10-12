// Copyright (c) 2026 Christopher Swenson
// Licensed under the Apache-2.0 license.

//! Checks the Cargo features: the vector and vendor instructions decode
//! only when their feature is enabled, and everything else decodes either
//! way.

use disc_v::{Decoder, Extensions, Isa};

/// The text for `inst`, or `.insn` if `enabled` is false.
fn expect(dec: Decoder, inst: u64, text: &str, enabled: bool) {
    let want = if enabled {
        text.to_string()
    } else {
        format!(".insn 4, {inst:#010x}")
    };
    assert_eq!(dec.decode(0, inst).to_string(), want);
}

#[test]
fn base_instructions_decode() {
    let dec = Decoder::new(Isa::Rv32);
    assert_eq!(dec.decode(0, 0x02b50533).to_string(), "mul a0,a0,a1");
    assert_eq!(dec.decode(0, 0x4505).to_string(), "li a0,1");
    assert_eq!(dec.decode(0, 0x00b57553).to_string(), "fadd.s fa0,fa0,fa1");
}

#[test]
fn vector() {
    let dec = Decoder::new(Isa::Rv64);
    let vector = cfg!(feature = "vector");
    expect(dec, 0x022081d7, "vadd.vv v3,v2,v1", vector);
    expect(dec, 0x0c207557, "vsetvli a0,zero,e8,m4,ta,ma", vector);
    expect(dec, 0x0205e507, "vle32.v v10,(a1)", vector);
}

#[test]
fn vendor() {
    let thead = Decoder::with_only(Isa::Rv64, Extensions::GC | Extensions::XTHEAD).unwrap();
    expect(
        thead,
        0x02c5950b,
        "th.addsl a0,a1,a2,1",
        cfg!(feature = "vendor-thead"),
    );
    let corev = Decoder::with_only(Isa::Rv32, Extensions::GC | Extensions::XCV).unwrap();
    expect(
        corev,
        0x00c5857b,
        "cv.add.h a0,a1,a2",
        cfg!(feature = "vendor-corev"),
    );
}
