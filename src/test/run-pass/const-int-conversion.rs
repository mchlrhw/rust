// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(const_int_conversion, const_int_ops, reverse_bits)]

const REVERSE: u32 = 0x12345678_u32.reverse_bits();
const FROM_BE_BYTES: i32 = i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]);
const FROM_LE_BYTES: i32 = i32::from_le_bytes([0x12, 0x34, 0x56, 0x78]);
const FROM_NE_BYTES: i32 = i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0]));
const TO_BE_BYTES: [u8; 4] = 0x12_34_56_78_i32.to_be_bytes();
const TO_LE_BYTES: [u8; 4] = 0x12_34_56_78_i32.to_le_bytes();
const TO_NE_BYTES: [u8; 4] = i32::min_value().to_be().to_ne_bytes();

fn ident<T>(ident: T) -> T {
    ident
}

fn main() {
    assert_eq!(REVERSE, ident(0x1e6a2c48));
    assert_eq!(FROM_BE_BYTES, ident(0x12_34_56_78));
    assert_eq!(FROM_LE_BYTES, ident(0x78_56_34_12));
    assert_eq!(FROM_NE_BYTES, ident(i32::min_value()));
    assert_eq!(TO_BE_BYTES, ident([0x12, 0x34, 0x56, 0x78]));
    assert_eq!(TO_LE_BYTES, ident([0x78, 0x56, 0x34, 0x12]));
    assert_eq!(TO_NE_BYTES, ident([0x80, 0, 0, 0]));
}

