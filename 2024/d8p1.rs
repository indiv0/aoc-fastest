// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut positions = [[MaybeUninit::<(u32, u32)>::uninit(); 4]; 128];
    let mut lengths = [0; 128];
    let mut marked = [1u8; 51 * 50];
    let mut count = 0;

    for y in 0..50 {
        let offset = 51 * y;
        let block1 = u8x32::from_slice(input.get_unchecked(offset..offset + 32));
        let mask1 = block1.simd_ne(u8x32::splat(b'.')).to_bitmask();
        let block2 = u8x32::from_slice(input.get_unchecked(offset + 50 - 32..offset + 50));
        let mask2 = block2.simd_ne(u8x32::splat(b'.')).to_bitmask() << 18;
        let mut mask = mask1 | mask2;

        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);

            let b = *input.get_unchecked(offset + x as usize);
            let len = lengths.get_unchecked_mut(b as usize);
            let poss = positions.get_unchecked_mut(b as usize);
            poss.get_unchecked_mut(*len)
                .write((x as u32, offset as u32 + x));
            *len += 1;

            let (xi, oi) = (x as usize, offset + x as usize);

            macro_rules! handle {
                ($p:expr) => {{
                    let p = $p;
                    let (xj, oj) = p.assume_init();
                    let (xj, oj) = (xj as usize, oj as usize);

                    let xa = (2 * xi).wrapping_sub(xj);
                    let oa = (2 * oi).wrapping_sub(oj);
                    if xa < 50 && oa < 51 * 50 {
                        count += *marked.get_unchecked(oa) as u64;
                        *marked.get_unchecked_mut(oa) = 0;
                    }

                    let xa = (2 * xj).wrapping_sub(xi);
                    let oa = (2 * oj).wrapping_sub(oi);
                    if xa < 50 && oa < 51 * 50 {
                        count += *marked.get_unchecked(oa) as u64;
                        *marked.get_unchecked_mut(oa) = 0;
                    }
                }};
            }

            if *len > 1 {
                handle!(poss.get_unchecked(0));
                if *len > 2 {
                    handle!(poss.get_unchecked(1));
                    if *len > 3 {
                        handle!(poss.get_unchecked(2));
                    }
                }
            }
        }
    }

    count
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut positions = [[MaybeUninit::<(u32, u32)>::uninit(); 4]; 128];
    let mut lengths = [0; 128];
    let mut marked = [1u8; 51 * 50];
    let mut count = 0;

    for y in 0..50 {
        let offset = 51 * y;
        let block1 = u8x32::from_slice(input.get_unchecked(offset..offset + 32));
        let mask1 = block1.simd_ne(u8x32::splat(b'.')).to_bitmask();
        let block2 = u8x32::from_slice(input.get_unchecked(offset + 50 - 32..offset + 50));
        let mask2 = block2.simd_ne(u8x32::splat(b'.')).to_bitmask() << 18;
        let mut mask = mask1 | mask2;

        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);

            let b = *input.get_unchecked(offset + x as usize);
            let len = lengths.get_unchecked_mut(b as usize);
            let poss = positions.get_unchecked_mut(b as usize);
            poss.get_unchecked_mut(*len)
                .write((x as u32, offset as u32 + x));
            *len += 1;

            let xi = x;
            let oi = offset + x as usize;

            count += *marked.get_unchecked(oi) as u64;
            *marked.get_unchecked_mut(oi) = 0;

            for pj in poss.get_unchecked(..*len - 1) {
                let (xj, oj) = pj.assume_init();
                let oj = oj as usize;

                let dx = xj.wrapping_sub(xi);
                let dq = oj.wrapping_sub(oi);

                let mut xa = (2 * xi).wrapping_sub(xj);
                let mut oa = (2 * oi).wrapping_sub(oj);
                while xa < 50 && oa < 51 * 50 {
                    count += *marked.get_unchecked(oa) as u64;
                    *marked.get_unchecked_mut(oa) = 0;
                    (oa, xa) = (oa.wrapping_sub(dq), xa.wrapping_sub(dx));
                }

                let mut xa = (2 * xj).wrapping_sub(xi);
                let mut oa = (2 * oj).wrapping_sub(oi);
                while xa < 50 && oa < 51 * 50 {
                    count += *marked.get_unchecked(oa) as u64;
                    *marked.get_unchecked_mut(oa) = 0;
                    (oa, xa) = (oa.wrapping_add(dq), xa.wrapping_add(dx));
                }
            }
        }
    }

    count
}

