// SPDX-License-Identifier: BSD-3-Clause
// See Notices.txt for copyright information
#![cfg_attr(not(test), no_std)]
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate c99;

use c99::{int_fast32_t, int_fast64_t, uint_fast32_t, uint_fast64_t, uint_fast8_t};

extern "C" {
    pub fn softfloat_detectTininess_read_helper() -> uint_fast8_t;
    pub fn softfloat_detectTininess_write_helper(v: uint_fast8_t);
    pub fn softfloat_roundingMode_read_helper() -> uint_fast8_t;
    pub fn softfloat_roundingMode_write_helper(v: uint_fast8_t);
    pub fn softfloat_exceptionFlags_read_helper() -> uint_fast8_t;
    pub fn softfloat_exceptionFlags_write_helper(v: uint_fast8_t);
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float32_t {
    pub v: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float64_t {
    pub v: u64,
}

pub const softfloat_tininess_beforeRounding: u8 = 0;
pub const softfloat_tininess_afterRounding: u8 = 1;

pub const softfloat_round_near_even: u8 = 0;
pub const softfloat_round_minMag: u8 = 1;
pub const softfloat_round_min: u8 = 2;
pub const softfloat_round_max: u8 = 3;
pub const softfloat_round_near_maxMag: u8 = 4;
pub const softfloat_round_odd: u8 = 6;

pub const softfloat_flag_inexact: u8 = 1;
pub const softfloat_flag_underflow: u8 = 2;
pub const softfloat_flag_overflow: u8 = 4;
pub const softfloat_flag_infinite: u8 = 8;
pub const softfloat_flag_invalid: u8 = 16;

extern "C" {
    pub fn softfloat_raiseFlags(_: uint_fast8_t);

    pub fn ui32_to_f32(_: u32) -> float32_t;
    pub fn ui32_to_f64(_: u32) -> float64_t;
    pub fn ui64_to_f32(_: u64) -> float32_t;
    pub fn ui64_to_f64(_: u64) -> float64_t;

    pub fn i32_to_f32(_: i32) -> float32_t;
    pub fn i32_to_f64(_: i32) -> float64_t;

    pub fn i64_to_f32(_: i64) -> float32_t;
    pub fn i64_to_f64(_: i64) -> float64_t;

    pub fn f32_to_ui32(_: float32_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn f32_to_ui64(_: float32_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn f32_to_i32(_: float32_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn f32_to_i64(_: float32_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn f32_to_ui32_r_minMag(_: float32_t, _: bool) -> uint_fast32_t;
    pub fn f32_to_ui64_r_minMag(_: float32_t, _: bool) -> uint_fast64_t;
    pub fn f32_to_i32_r_minMag(_: float32_t, _: bool) -> int_fast32_t;
    pub fn f32_to_i64_r_minMag(_: float32_t, _: bool) -> int_fast64_t;
    pub fn f32_to_f64(_: float32_t) -> float64_t;

    pub fn f32_roundToInt(_: float32_t, _: uint_fast8_t, _: bool) -> float32_t;
    pub fn f32_add(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_sub(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_mul(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_mulAdd(_: float32_t, _: float32_t, _: float32_t) -> float32_t;
    pub fn f32_div(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_rem(_: float32_t, _: float32_t) -> float32_t;
    pub fn f32_sqrt(_: float32_t) -> float32_t;
    pub fn f32_eq(_: float32_t, _: float32_t) -> bool;
    pub fn f32_le(_: float32_t, _: float32_t) -> bool;
    pub fn f32_lt(_: float32_t, _: float32_t) -> bool;
    pub fn f32_eq_signaling(_: float32_t, _: float32_t) -> bool;
    pub fn f32_le_quiet(_: float32_t, _: float32_t) -> bool;
    pub fn f32_lt_quiet(_: float32_t, _: float32_t) -> bool;
    pub fn f32_isSignalingNaN(_: float32_t) -> bool;

    pub fn f64_to_ui32(_: float64_t, _: uint_fast8_t, _: bool) -> uint_fast32_t;
    pub fn f64_to_ui64(_: float64_t, _: uint_fast8_t, _: bool) -> uint_fast64_t;
    pub fn f64_to_i32(_: float64_t, _: uint_fast8_t, _: bool) -> int_fast32_t;
    pub fn f64_to_i64(_: float64_t, _: uint_fast8_t, _: bool) -> int_fast64_t;
    pub fn f64_to_ui32_r_minMag(_: float64_t, _: bool) -> uint_fast32_t;
    pub fn f64_to_ui64_r_minMag(_: float64_t, _: bool) -> uint_fast64_t;
    pub fn f64_to_i32_r_minMag(_: float64_t, _: bool) -> int_fast32_t;
    pub fn f64_to_i64_r_minMag(_: float64_t, _: bool) -> int_fast64_t;
    pub fn f64_to_f32(_: float64_t) -> float32_t;

    pub fn f64_roundToInt(_: float64_t, _: uint_fast8_t, _: bool) -> float64_t;
    pub fn f64_add(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_sub(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_mul(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_mulAdd(_: float64_t, _: float64_t, _: float64_t) -> float64_t;
    pub fn f64_div(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_rem(_: float64_t, _: float64_t) -> float64_t;
    pub fn f64_sqrt(_: float64_t) -> float64_t;
    pub fn f64_eq(_: float64_t, _: float64_t) -> bool;
    pub fn f64_le(_: float64_t, _: float64_t) -> bool;
    pub fn f64_lt(_: float64_t, _: float64_t) -> bool;
    pub fn f64_eq_signaling(_: float64_t, _: float64_t) -> bool;
    pub fn f64_le_quiet(_: float64_t, _: float64_t) -> bool;
    pub fn f64_lt_quiet(_: float64_t, _: float64_t) -> bool;
    pub fn f64_isSignalingNaN(_: float64_t) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linking() {
        macro_rules! link_functions {
            ($($name:ident,)*) => {
                $(println!(concat!(stringify!($name), ": {:#X}"), $name as usize);)*
            };
        }
        link_functions!(
            softfloat_detectTininess_read_helper,
            softfloat_detectTininess_write_helper,
            softfloat_roundingMode_read_helper,
            softfloat_roundingMode_write_helper,
            softfloat_exceptionFlags_read_helper,
            softfloat_exceptionFlags_write_helper,
            softfloat_raiseFlags,
            ui32_to_f32,
            ui32_to_f64,
            ui64_to_f32,
            ui64_to_f64,
            i32_to_f32,
            i32_to_f64,
            i64_to_f32,
            i64_to_f64,
            f32_to_ui32,
            f32_to_ui64,
            f32_to_i32,
            f32_to_i64,
            f32_to_ui32_r_minMag,
            f32_to_ui64_r_minMag,
            f32_to_i32_r_minMag,
            f32_to_i64_r_minMag,
            f32_to_f64,
            f32_roundToInt,
            f32_add,
            f32_sub,
            f32_mul,
            f32_mulAdd,
            f32_div,
            f32_rem,
            f32_sqrt,
            f32_eq,
            f32_le,
            f32_lt,
            f32_eq_signaling,
            f32_le_quiet,
            f32_lt_quiet,
            f32_isSignalingNaN,
            f64_to_ui32,
            f64_to_ui64,
            f64_to_i32,
            f64_to_i64,
            f64_to_ui32_r_minMag,
            f64_to_ui64_r_minMag,
            f64_to_i32_r_minMag,
            f64_to_i64_r_minMag,
            f64_to_f32,
            f64_roundToInt,
            f64_add,
            f64_sub,
            f64_mul,
            f64_mulAdd,
            f64_div,
            f64_rem,
            f64_sqrt,
            f64_eq,
            f64_le,
            f64_lt,
            f64_eq_signaling,
            f64_le_quiet,
            f64_lt_quiet,
            f64_isSignalingNaN,
        );
    }
}
