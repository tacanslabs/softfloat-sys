// SPDX-License-Identifier: BSD-3-Clause
// See Notices.txt for copyright information

extern crate cc;
extern crate cc_version;

use cc_version::{cc_version, Version};
use std::path::Path;

#[cfg(feature = "8086")]
const SPECIALIZED_PATH: &str = "8086";
#[cfg(feature = "8086-sse")]
const SPECIALIZED_PATH: &str = "8086-SSE";
#[cfg(feature = "arm-vfpv2")]
const SPECIALIZED_PATH: &str = "ARM-VFPv2";
#[cfg(feature = "arm-vfpv2-defaultnan")]
const SPECIALIZED_PATH: &str = "ARM-VFPv2-defaultNaN";
#[cfg(feature = "riscv")]
const SPECIALIZED_PATH: &str = "RISCV";

fn main() {
    let mut builder = cc::Build::new();

    let tool = builder.get_compiler();
    let thread_local = if tool.is_like_gnu() {
        let version = cc_version(&tool).expect("Failed to detect GCC version");

        // GCC 4.9 supports _Thread_local
        if version >= Version::parse("4.9").unwrap() {
            Some("_Thread_local")
        } else {
            Some("__thread")
        }
    } else {
        Some("_Thread_local")
    };

    let softfloat_base = Path::new("berkeley-softfloat-3");
    let softfloat_source = softfloat_base.join(Path::new("source"));
    let softfloat_build = softfloat_base.join(Path::new("build"));

    let primitive_sources = [
        "s_eq128.c",
        "s_le128.c",
        "s_lt128.c",
        "s_shortShiftLeft128.c",
        "s_shortShiftRight128.c",
        "s_shortShiftRightJam64.c",
        "s_shortShiftRightJam64Extra.c",
        "s_shortShiftRightJam128.c",
        "s_shortShiftRightJam128Extra.c",
        "s_shiftRightJam32.c",
        "s_shiftRightJam64.c",
        "s_shiftRightJam64Extra.c",
        "s_shiftRightJam128.c",
        "s_shiftRightJam128Extra.c",
        "s_shiftRightJam256M.c",
        "s_countLeadingZeros8.c",
        "s_countLeadingZeros16.c",
        "s_countLeadingZeros32.c",
        "s_countLeadingZeros64.c",
        "s_add128.c",
        "s_add256M.c",
        "s_sub128.c",
        "s_sub256M.c",
        "s_mul64ByShifted32To128.c",
        "s_mul64To128.c",
        "s_mul128By32.c",
        "s_mul128To256M.c",
        "s_approxRecip_1Ks.c",
        "s_approxRecip32_1.c",
        "s_approxRecipSqrt_1Ks.c",
        "s_approxRecipSqrt32_1.c",
    ];
    let specialize_sources = [
        "softfloat_raiseFlags.c",
        "s_f64UIToCommonNaN.c",
        "s_commonNaNToF64UI.c",
        "s_propagateNaNF64UI.c",
    ];
    let other_sources = [
        "s_roundToUI32.c",
        "s_roundToUI64.c",
        "s_roundToI32.c",
        "s_roundToI64.c",
        "s_normSubnormalF64Sig.c",
        "s_roundPackToF64.c",
        "s_normRoundPackToF64.c",
        "s_addMagsF64.c",
        "s_subMagsF64.c",
        "s_mulAddF64.c",
        "softfloat_state.c",
        "ui32_to_f64.c",
        "ui64_to_f64.c",
        "i32_to_f64.c",
        "i64_to_f64.c",
        "f64_to_ui32.c",
        "f64_to_ui64.c",
        "f64_to_i32.c",
        "f64_to_i64.c",
        "f64_to_ui32_r_minMag.c",
        "f64_to_ui64_r_minMag.c",
        "f64_to_i32_r_minMag.c",
        "f64_to_i64_r_minMag.c",
        "f64_roundToInt.c",
        "f64_add.c",
        "f64_sub.c",
        "f64_mul.c",
        "f64_mulAdd.c",
        "f64_div.c",
        "f64_rem.c",
        "f64_sqrt.c",
        "f64_eq.c",
        "f64_le.c",
        "f64_lt.c",
        "f64_eq_signaling.c",
        "f64_le_quiet.c",
        "f64_lt_quiet.c",
        "f64_isSignalingNaN.c",
    ];
    let specialized_source_path = softfloat_source.join(Path::new(SPECIALIZED_PATH));
    builder
        .include(softfloat_build.join(Path::new("Wasm-Clang")))
        .include(&specialized_source_path)
        .define("SOFTFLOAT_ROUND_ODD", None)
        .define("INLINE_LEVEL", Some("5"))
        .define("SOFTFLOAT_FAST_DIV32TO16", None)
        .define("SOFTFLOAT_FAST_DIV64TO32", None)
        .define("SOFTFLOAT_FAST_INT64", None)
        .define("THREAD_LOCAL", thread_local)
        .files(
            primitive_sources
                .iter()
                .chain(other_sources.iter())
                .map(|file| softfloat_source.join(Path::new(file))),
        )
        .files(
            specialize_sources
                .iter()
                .map(|file| specialized_source_path.join(Path::new(file))),
        );
    builder
        .include(softfloat_source.join(Path::new("include")))
        .file(Path::new("helper.c"))
        .warnings(false)
        .compile("softfloat-sys");
}
