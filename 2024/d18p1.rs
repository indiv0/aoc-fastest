// Original by: alion02
//                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           .
#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
    clippy::precedence,
    clippy::missing_transmute_annotations,
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
            __m128i, __m256i, _bextr2_u32, _mm256_madd_epi16, _mm256_maddubs_epi16,
            _mm256_movemask_epi8, _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16,
            _mm_maddubs_epi16, _mm_minpos_epu16, _mm_movemask_epi8, _mm_packus_epi32,
            _mm_shuffle_epi8, _mm_testc_si128, _pdep_u32, _pext_u32, _pext_u64,
        },
    },
    array,
    fmt::Display,
    hint::assert_unchecked,
    intrinsics::{likely, unlikely},
    mem::{offset_of, transmute, MaybeUninit},
    ptr,
    simd::prelude::*,
    slice,
};

static LUT: [i8x16; 512] = unsafe {
    let mut lut = [[-1i8; 16]; 512];

    let mut idx = 0;
    while idx < 512 {
        let shuffle = &mut lut[idx];

        let mut mask = idx << 2;
        if idx & 1 == 0 {
            mask |= 2;
        }
        mask |= 0x800;

        let mut slot = 0;
        let mut byte = 0;
        while slot < 8 {
            let zeros = mask.trailing_zeros();
            match zeros {
                1 => {
                    shuffle[slot + 1] = byte;
                    byte += 2;
                }
                2 => {
                    shuffle[slot] = byte;
                    shuffle[slot + 1] = byte + 1;
                    byte += 3;
                }
                _ => break,
            }
            mask >>= zeros + 1;
            slot += 2;
        }

        idx += 1;
    }

    transmute(lut)
};

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    let mut ptr = s.as_ptr().cast::<i8x16>();
    let lut = &LUT;

    static mut MAP: [i8; 73 * 72 / 8] = [0; 73 * 72 / 8];

    let map = MAP.as_mut_ptr();
    for i in 0..23 {
        map.add(72 / 8 + i * 72 / 8 * 3)
            .cast::<i8x32>()
            .write_unaligned(i8x32::from_array([
                !0, !0, !0, !0, !0, !0, !0, !0, !-128, !0, !0, !0, !0, !0, !0, !0, !0, !-128, !0,
                !0, !0, !0, !0, !0, !0, !0, !-128, !-1, !-1, !-1, !-1, !-1,
            ]));
    }
    map.add(69 * 72 / 8)
        .cast::<i8x32>()
        .write_unaligned(i8x32::from_array([
            !0, !0, !0, !0, !0, !0, !0, !0, !-128, !0, !0, !0, !0, !0, !0, !0, !0, !-128, !0, !0,
            !0, !0, !0, !0, !0, !0, !-128, !-1, !-1, !-1, !-1, !-1,
        ]));

    macro_rules! btr {
        ($idx:expr) => {
            asm!(
                "btr dword ptr[{map} + {offset}], {idx:e}",
                map = in(reg) map,
                idx = in(reg) $idx,
                offset = const 72 / 8,
                options(nostack),
            );
        };
    }

    for _ in 0..512 {
        let chunk = ptr.read_unaligned();
        let chunk = chunk - Simd::splat(b'0' as _);
        let mask = chunk.simd_lt(Simd::splat(0)).to_bitmask() as u32;
        let step = _pdep_u32(8, mask).trailing_zeros() + 1;
        let shuffle = lut.as_ptr().byte_add(((mask & 0x7FC) * 4) as usize).read();
        let chunk = _mm_shuffle_epi8(chunk.into(), shuffle.into());
        let chunk = _mm_maddubs_epi16(chunk, u16x8::splat(u16::from_ne_bytes([10, 1])).into());
        let chunk: u32x4 = _mm_madd_epi16(
            chunk,
            u16x8::from_array([72, 1, 72, 1, 72, 1, 72, 1]).into(),
        )
        .into();
        let p1 = chunk[0];
        let p2 = chunk[1];
        btr!(p1);
        btr!(p2);
        ptr = ptr.byte_add(step as usize);
    }

    static mut FRONT: [u16; 256] = [0; 256];

    let res: u32;

    asm!(
    "30:",
        "lea {next:e}, [{pos} + 1]",
        "btr dword ptr[{map}], {next:e}",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "adc {j:l}, 0",
        "lea {next:e}, [{pos} + 72]",
        "btr dword ptr[{map}], {next:e}",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "adc {j:l}, 0",
        "lea {next:e}, [{pos} - 1]",
        "btr dword ptr[{map}], {next:e}",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "adc {j:l}, 0",
        "lea {next:e}, [{pos} - 72]",
        "btr dword ptr[{map}], {next:e}",
        "mov word ptr[{front} + {j} * 2], {next:x}",
        "adc {j:l}, 0",
        "cmp {i:l}, {k:l}",
        "jne 20f",
        "mov {k:e}, {j:e}",
        "inc {dist:e}",
    "20:",
        "movzx {pos:e}, word ptr[{front} + {i} * 2]",
        "inc {i:l}",
        "cmp {pos:x}, {end}",
        "jne 30b",
        map = in(reg) map,
        pos = in(reg) 72usize,
        next = out(reg) _,
        front = in(reg) &mut FRONT,
        i = inout(reg) 0usize => _,
        j = inout(reg) 0usize => _,
        k = inout(reg) 0usize => _,
        dist = inout(reg) 0 => res,
        end = const 72 * 72 - 2,
        options(nostack),
    );

    res
}

#[inline]
unsafe fn inner2(s: &[u8]) -> &str {
    ""
}

#[inline]
pub fn run(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> &str {
    unsafe { inner2(s.as_bytes()) }
}
