// Original by: alion02
#![feature(thread_local, portable_simd, core_intrinsics, fn_align)]
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
            _mm_movemask_epi8, _mm_packus_epi16, _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pdep_u32,
            _pext_u32, _pext_u64,
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
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut LOCKS: [u32; 250] = [0; 250];
    static mut KEYS: [u32x8; 32] = [Simd::from_array([!0; 8]); 32];

    let locks = LOCKS.as_mut_ptr();
    let keys = KEYS.as_mut_ptr();

    {
        let ptr = s.as_ptr();
        let mut locks = locks;
        let mut keys = keys.cast::<u32>();
        for i in 0..500 {
            let chunk = ptr.add(i * 43 + 3).cast::<u8x32>().read_unaligned();
            let chunk = chunk.simd_eq(Simd::splat(b'#'));
            let mask = chunk.to_bitmask() as u32;
            if mask & 1 == 1 {
                *keys = mask;
                keys = keys.add(1);
            } else {
                *locks = mask;
                locks = locks.add(1);
            }
        }
    }

    let mut sums = i32x8::splat(0);
    for i in 0..250 {
        for j in 0..32 {
            sums += (Simd::splat(*locks.add(i)) & *keys.add(j))
                .simd_eq(Simd::splat(0))
                .to_int();
        }
    }

    -sums.reduce_sum() as u32
}

#[inline]
pub fn run(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}
