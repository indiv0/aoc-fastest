// Original by: giooschi
#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::mem::transmute;
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

const LEFT: usize = -1isize as usize;
const RIGHT: usize = 1isize as usize;
const UP: usize = -142isize as usize;
const DOWN: usize = 142isize as usize;

#[inline(always)]
unsafe fn find_start_end(input: &[u8]) -> (usize, usize) {
    let mut offset = 0;
    let p1 = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_ge(u8x64::splat(b'E')).to_bitmask();
        if mask != 0 {
            break offset + mask.trailing_zeros() as usize;
        }
        offset += 64;
    };

    let b = (b'E' + b'S') - *input.get_unchecked(p1);

    offset = p1 + 1;
    let p2 = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b)).to_bitmask();
        if mask != 0 {
            break offset + mask.trailing_zeros() as usize;
        }
        offset += 64;
    };

    let (s, e) = if b == b'S' { (p1, p2) } else { (p2, p1) };

    (s, e)
}

unsafe fn part1_rec<const DIR: usize>(
    input: &[u8; 141 * 142],
    seen: &mut [u16; 142 * 143],
    curr: usize,
    mut n: u16,
    mut count: u64,
) -> u64 {
    macro_rules! count {
        ($($d:ident),*) => {$(
            if $d != -(DIR as isize) as usize {
                if *seen.get_unchecked((curr + 142).wrapping_add($d).wrapping_add($d)) >= n + 101 {
                    count += 1;
                }
            }
        )*};
    }

    count!(LEFT, RIGHT, UP, DOWN);

    *seen.get_unchecked_mut(curr + 142) = n;
    n -= 1;

    macro_rules! next {
        ($($d:ident),*) => {$(
            if $d != -(DIR as isize) as usize {
                let cand = curr.wrapping_add($d);
                if *input.get_unchecked(cand) != b'#' {
                    // TODO: use become
                    return part1_rec::<$d>(input, seen, cand, n, count)
                }
            }
        )*};
    }

    next!(LEFT, RIGHT, UP, DOWN);

    count
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input: &[u8; 141 * 142] = input.as_bytes().try_into().unwrap_unchecked();

    let (s, _e) = find_start_end(input);

    let mut seen = [0; 142 * 143];
    let mut n = u16::MAX - 102;
    *seen.get_unchecked_mut(s + 142) = n;
    n -= 1;

    macro_rules! next {
        ($($d:ident),*) => {$(
            let cand = s.wrapping_add($d);
            if *input.get_unchecked(cand) != b'#' {
                return part1_rec::<$d>(input, &mut seen, cand, n, 0);
            }
        )*};
    }

    next!(LEFT, RIGHT, UP, DOWN);

    std::hint::unreachable_unchecked()
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    const SLINE: usize = 139 + 28;

    #[inline(always)]
    unsafe fn next(input: &[u8], iprev: usize, icurr: usize, scurr: usize) -> (usize, usize) {
        const SUP: usize = -(SLINE as isize) as usize;
        const SDOWN: usize = SLINE;

        let mut inext = icurr.wrapping_add(LEFT);
        let mut snext = scurr.wrapping_add(LEFT);
        for (id, sd) in [(RIGHT, RIGHT), (UP, SUP), (DOWN, SDOWN)] {
            let cand = icurr.wrapping_add(id);
            if *input.get_unchecked(cand) != b'#' && cand != iprev {
                inext = icurr.wrapping_add(id);
                snext = scurr.wrapping_add(sd);
            }
        }
        (inext, snext)
    }

    let input = input.as_bytes();

    let (s, e) = find_start_end(input);

    let mut seen = [0; 20 + (139 + 40) * SLINE];

    let mut count = u32x16::splat(0);
    let mut iprev = 0;
    let mut icurr = s;
    let mut scurr = 20 + SLINE * (s / 142 - 1 + 20) + (s % 142 - 1);
    let mut n = u16::MAX / 2;

    loop {
        debug_assert_eq!(seen[scurr], 255);
        *seen.get_unchecked_mut(scurr) = n;

        (iprev, (icurr, scurr)) = (icurr, next(input, iprev, icurr, scurr));
        n -= 1;

        const DISTS: [[u16x16; 3]; 41] = {
            let mut dists = [[u16::MAX / 2; 16 * 3]; 41];

            let mut y = 0;
            while y <= 40usize {
                let dy = y.abs_diff(20);
                let mut x = 0;
                while x <= 40usize {
                    let dx = x.abs_diff(20);
                    if dx + dy <= 20 {
                        dists[y][x] = 100 + (dx + dy) as u16;
                    }
                    x += 1;
                }
                y += 1;
            }

            unsafe { transmute(dists) }
        };

        let mut offset = scurr - 20 - 20 * SLINE;
        let mut tmp_count = u16x16::splat(0);
        for line in 0..41 {
            for i in 0..3 {
                let b =
                    u16x16::from_slice(seen.get_unchecked(offset + 16 * i..offset + 16 * (i + 1)));
                let m = b.simd_ge(u16x16::splat(n) + DISTS[line][i]);
                tmp_count += m.to_int().cast::<u16>() & u16x16::splat(1);
            }

            offset += SLINE;
        }
        count += tmp_count.cast::<u32>();

        if icurr == e {
            break;
        }
    }

    #[cfg(debug_assertions)]
    for y in 0..139 {
        for x in 0..139 {
            if input[142 * (y + 1) + (x + 1)] == b'#' {
                debug_assert_eq!(seen[20 + 20 * SLINE + SLINE * y + x], 0);
            }
        }
    }

    count.cast::<u64>().reduce_sum()
}
