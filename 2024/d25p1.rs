// Original by: giooschi
#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]
#![feature(fn_align)]

use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

#[inline(always)]
#[repr(align(64))]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(_input: &str) -> u64 {
    0
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
#[repr(align(64))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    #[repr(C, align(64))]
    struct Data {
        list0: [u32x8; 256 / 8],
        list1: [u32x8; 256 / 8],
    }

    static mut DATA: Data = Data {
        list0: [u32x8::from_array([u32::MAX; 8]); 256 / 8],
        list1: [u32x8::from_array([u32::MAX; 8]); 256 / 8],
    };
    let data = &mut DATA;
    let mut len0 = 0;
    let mut len1 = 0;

    let mut ptr = input.as_ptr().add(4);
    let end = ptr.add(43 * 500);
    loop {
        let b = ptr.cast::<u8x32>().read_unaligned();
        let m = b.simd_eq(u8x32::splat(b'#')).to_bitmask() as u32;

        if *ptr == b'#' {
            *data.list0.as_mut_ptr().cast::<u32>().add(len0) = m;
            len0 += 1;
        } else {
            *data.list1.as_mut_ptr().cast::<u32>().add(len1) = m;
            len1 += 1;
        }

        ptr = ptr.add(43);
        if ptr == end {
            break;
        }
    }

    let mut count = i32x8::splat(0);

    for i in 0..250 {
        let m = *data.list1.as_ptr().cast::<u32>().add(i);
        for b in &data.list0 {
            count += (b & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
        }
    }

    -count.reduce_sum() as u64
}
