// Rust port of expat's ascii.h
//
// Original C code:
//   Copyright (c) 1999-2000 Thai Open Source Software Center Ltd
//   Copyright (c) 2000      Clark Cooper <coopercc@users.sourceforge.net>
//   Copyright (c) 2002      Fred L. Drake, Jr. <fdrake@users.sourceforge.net>
//   Copyright (c) 2007      Karl Waclawek <karl@waclawek.net>
//   Copyright (c) 2017      Sebastian Pipping <sebastian@pipping.org>
//
// Rust port:
//   Copyright (c) 2026 Rob Ennals <rob@ennals.org>
//
// Licensed under the MIT license (see LICENSE file).

//! ASCII character byte constants, ported from expat's `ascii.h`.
//!
//! Provides named constants for every ASCII letter, digit, and punctuation character
//! used by the tokenizer and parser. Values are raw `u8` bytes, enabling
//! encoding-independent comparisons against input data.

pub const A: u8 = 0x41;
pub const B: u8 = 0x42;
pub const C: u8 = 0x43;
pub const D: u8 = 0x44;
pub const E: u8 = 0x45;
pub const F: u8 = 0x46;
pub const G: u8 = 0x47;
pub const H: u8 = 0x48;
pub const I: u8 = 0x49;
pub const J: u8 = 0x4A;
pub const K: u8 = 0x4B;
pub const L: u8 = 0x4C;
pub const M: u8 = 0x4D;
pub const N: u8 = 0x4E;
pub const O: u8 = 0x4F;
pub const P: u8 = 0x50;
pub const Q: u8 = 0x51;
pub const R: u8 = 0x52;
pub const S: u8 = 0x53;
pub const T: u8 = 0x54;
pub const U: u8 = 0x55;
pub const V: u8 = 0x56;
pub const W: u8 = 0x57;
pub const X: u8 = 0x58;
pub const Y: u8 = 0x59;
pub const Z: u8 = 0x5A;

pub const A_LOWER: u8 = 0x61;
pub const B_LOWER: u8 = 0x62;
pub const C_LOWER: u8 = 0x63;
pub const D_LOWER: u8 = 0x64;
pub const E_LOWER: u8 = 0x65;
pub const F_LOWER: u8 = 0x66;
pub const G_LOWER: u8 = 0x67;
pub const H_LOWER: u8 = 0x68;
pub const I_LOWER: u8 = 0x69;
pub const J_LOWER: u8 = 0x6A;
pub const K_LOWER: u8 = 0x6B;
pub const L_LOWER: u8 = 0x6C;
pub const M_LOWER: u8 = 0x6D;
pub const N_LOWER: u8 = 0x6E;
pub const O_LOWER: u8 = 0x6F;
pub const P_LOWER: u8 = 0x70;
pub const Q_LOWER: u8 = 0x71;
pub const R_LOWER: u8 = 0x72;
pub const S_LOWER: u8 = 0x73;
pub const T_LOWER: u8 = 0x74;
pub const U_LOWER: u8 = 0x75;
pub const V_LOWER: u8 = 0x76;
pub const W_LOWER: u8 = 0x77;
pub const X_LOWER: u8 = 0x78;
pub const Y_LOWER: u8 = 0x79;
pub const Z_LOWER: u8 = 0x7A;

pub const DIGIT_0: u8 = 0x30;
pub const DIGIT_1: u8 = 0x31;
pub const DIGIT_2: u8 = 0x32;
pub const DIGIT_3: u8 = 0x33;
pub const DIGIT_4: u8 = 0x34;
pub const DIGIT_5: u8 = 0x35;
pub const DIGIT_6: u8 = 0x36;
pub const DIGIT_7: u8 = 0x37;
pub const DIGIT_8: u8 = 0x38;
pub const DIGIT_9: u8 = 0x39;

pub const TAB: u8 = 0x09;
pub const SPACE: u8 = 0x20;
pub const EXCL: u8 = 0x21;
pub const QUOT: u8 = 0x22;
pub const AMP: u8 = 0x26;
pub const APOS: u8 = 0x27;
pub const MINUS: u8 = 0x2D;
pub const PERIOD: u8 = 0x2E;
pub const COLON: u8 = 0x3A;
pub const SEMI: u8 = 0x3B;
pub const LT: u8 = 0x3C;
pub const EQUALS: u8 = 0x3D;
pub const GT: u8 = 0x3E;
pub const LSQB: u8 = 0x5B;
pub const RSQB: u8 = 0x5D;
pub const UNDERSCORE: u8 = 0x5F;
pub const LPAREN: u8 = 0x28;
pub const RPAREN: u8 = 0x29;
pub const FF: u8 = 0x0C;
pub const SLASH: u8 = 0x2F;
pub const HASH: u8 = 0x23;
pub const PIPE: u8 = 0x7C;
pub const COMMA: u8 = 0x2C;
