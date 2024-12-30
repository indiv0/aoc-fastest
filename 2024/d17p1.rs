// Original by: alion02
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
            __m128i, __m256i, _bextr2_u32, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_movemask_epi8,
            _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16, _mm_minpos_epu16,
            _mm_movemask_epi8, _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pext_u32,
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

#[inline]
unsafe fn inner1(s: &[u8]) -> &str {
    static mut BUF: [u8; 17] = [b','; 17];

    let chunk = s.as_ptr().add(12).cast::<u8x16>().read_unaligned();
    let chunk = chunk - Simd::splat(b'0');
    let chunk = _mm_maddubs_epi16(
        chunk.into(),
        u8x16::from_array([10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1]).into(),
    );
    let chunk = _mm_madd_epi16(chunk, u16x8::from_array([100, 1, 100, 1, 100, 1, 100, 1]).into());
    let chunk = _mm_packus_epi32(chunk, chunk);
    let chunk = _mm_madd_epi16(
        chunk,
        u16x8::from_array([10000, 1, 10000, 1, 10000, 1, 10000, 1]).into(),
    );
    let mut a = u32x4::from(chunk)[0];
    let imm1 = *s.get_unchecked(65) as u32 - b'0' as u32;
    let chunk = s.as_ptr().add(64).cast::<u8x16>().read_unaligned();
    let chunk = chunk.simd_eq(Simd::from_array([
        0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0, b'1',
    ]));
    let mask = chunk.to_bitmask() as u32;
    let offset = mask.trailing_zeros() as usize;

    let imm2 = *s.get_unchecked(64 + offset + 2) as u32 - b'0' as u32;

    let buf = &mut BUF;
    let mut len = s.len();
    loop {
        let b = a % 8 ^ imm1;
        *buf.get_unchecked_mut(len - 91) = ((a >> b ^ b ^ imm2) % 8 + b'0' as u32) as u8;
        a >>= 3;
        len += 2;
        if a == 0 {
            break;
        }
    }

    std::str::from_utf8_unchecked(buf)
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u32 {
    0
}

#[inline]
pub fn run(s: &str) -> &str {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> u32 {
    unsafe { inner2(s.as_bytes()) }
