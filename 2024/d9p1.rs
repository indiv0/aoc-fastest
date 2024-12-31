// Original by: alion02
#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
    clippy::pointers_in_nomem_asm_block,
    clippy::erasing_op,
    static_mut_refs,
    internal_features,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

#[allow(unused)]
use std::{
    arch::{
        asm,
        x86_64::{
            __m256i, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_movemask_epi8,
            _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16,
            _mm_movemask_epi8, _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pext_u32,
        },
    },
    fmt::Display,
    mem::{offset_of, transmute, MaybeUninit},
    simd::prelude::*,
};

unsafe fn inner1(s: &[u8]) -> usize {
    let mut checksum = 0;

    asm!(
    "20:",
        "movzx {len:e}, byte ptr[{s} + {left} * 2]",
        "sub {len:e}, 48",
        "lea {scratch:e}, [{len} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {left}",
        "imul {scratch}, {len}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {len:e}",
        "movzx {rem_dst:e}, byte ptr[{s} + {left} * 2 + 1]",
        "inc {left:e}",
        "sub {rem_dst:e}, 48",
        "jz 20b",
        "cmp {left:e}, {right:e}",
        "je 50f",
    "22:",
        "dec {right:e}",
        "movzx {rem_src:e}, byte ptr[{s} + {right} * 2]",
        "sub {rem_src:e}, 48",
        "cmp {rem_dst}, {rem_src}",
        "ja 40f",
    "21:",
        "lea {scratch:e}, [{rem_dst} + {disk_pos} * 2 - 1]",
        "jb 30f",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_dst}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_dst:e}",
        "cmp {left:e}, {right:e}",
        "jne 20b",
        "jmp 50f",
    "30:",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_dst}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_dst:e}",
        "sub {rem_src:e}, {rem_dst:e}",
    "31:",
        "cmp {left:e}, {right:e}",
        "je 60f",
        "movzx {len:e}, byte ptr[{s} + {left} * 2]",
        "sub {len:e}, 48",
        "lea {scratch:e}, [{len} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {left}",
        "imul {scratch}, {len}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {len:e}",
        "movzx {rem_dst:e}, byte ptr[{s} + {left} * 2 + 1]",
        "inc {left:e}",
        "sub {rem_dst:e}, 48",
        "jz 31b",
        "cmp {rem_dst}, {rem_src}",
        "jbe 21b",
    "40:",
        "lea {scratch:e}, [{rem_src} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_src}",
        "add {checksum}, {scratch}",
        "add {disk_pos:e}, {rem_src:e}",
        "sub {rem_dst:e}, {rem_src:e}",
        "cmp {left:e}, {right:e}",
        "jne 22b",
        "jmp 50f",
    "60:",
        "lea {scratch:e}, [{rem_src} + {disk_pos} * 2 - 1]",
        "imul {scratch}, {right}",
        "imul {scratch}, {rem_src}",
        "add {checksum}, {scratch}",
    "50:",
        "shr {checksum}",
        checksum = inout(reg) checksum,
        s = in(reg) s.as_ptr(),
        left = inout(reg) 0usize => _,
        right = inout(reg) s.len() / 2 => _,
        disk_pos = inout(reg) 0usize => _,
        rem_dst = out(reg) _,
        rem_src = out(reg) _,
        scratch = out(reg) _,
        len = out(reg) _,
        options(nostack, readonly),
    );

    checksum
}

pub fn run(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}
