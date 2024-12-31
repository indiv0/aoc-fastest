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

use rayon::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
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
unsafe fn find_start(input: &[u8]) -> usize {
    let mut offset = 0;
    loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b'S')).to_bitmask();
        if mask != 0 {
            break offset + mask.trailing_zeros() as usize;
        }
        offset += 64;
    }
}

const SLINE: usize = 139 + 28;
const SUP: usize = -(SLINE as isize) as usize;
const SDOWN: usize = SLINE;
const SLEFT: usize = LEFT;
const SRIGHT: usize = RIGHT;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    unsafe fn part2_rec<const IDIR: usize, const SDIR: usize>(
        input: &[u8; 141 * 142],
        seen: &mut [i16; 20 + (139 + 40) * SLINE],
        icurr: usize,
        scurr: usize,
        mut n: i16,
    ) {
        *seen.get_unchecked_mut(scurr) = n;
        n += 1;

        macro_rules! next {
            ($($d1:ident $d2:ident),*) => {$(
                if $d1 != -(IDIR as isize) as usize {
                    let icand = icurr.wrapping_add($d1);
                    let scand = scurr.wrapping_add($d2);
                    if *input.get_unchecked(icand) != b'#' {
                        // TODO: use become
                        part2_rec::<$d1, $d2>(input, seen, icand, scand, n);
                        return
                    }
                }
            )*};
        }

        next!(LEFT SLEFT, RIGHT SRIGHT, UP SUP, DOWN SDOWN);
    }

    let input: &[u8; 141 * 142] = input.as_bytes().try_into().unwrap_unchecked();
    let s = find_start(input);

    let mut seen = [i16::MAX; 20 + (139 + 40) * SLINE];
    let icurr = s;
    let scurr = 20 + SLINE * (s / 142 - 1 + 20) + (s % 142 - 1);
    let n = 0;

    macro_rules! next {
        ($($d1:ident $d2:ident),*) => {$(
            let icand = icurr.wrapping_add($d1);
            if *input.get_unchecked(icand) != b'#' {
                part2_rec::<$d1, $d2>(input, &mut seen, icurr, scurr, n);
            }
        )*};
    }

    next!(LEFT SLEFT, RIGHT SRIGHT, UP SUP, DOWN SDOWN);

    const THREADS: usize = 16;
    const BASE: usize = 143;
    const REAL_LEN: usize = 141 * 142 - 2 * BASE + 8;
    const STEP: usize = REAL_LEN / THREADS;
    const { assert!(REAL_LEN % 16 == 0) }

    (0..THREADS)
        .into_par_iter()
        .with_max_len(1)
        .map(|thread| {
            let start = BASE + thread * STEP;
            let end = start + STEP;

            let mut count = i32x16::splat(0);

            for icurr in start..end {
                if *input.get_unchecked(icurr) == b'#' || *input.get_unchecked(icurr) == b'\n' {
                    continue;
                }

                let scurr = 20 + SLINE * (icurr / 142 - 1 + 20) + (icurr % 142 - 1);
                let n = *seen.get_unchecked(scurr);

                count += sum_cheats(scurr, n, &seen);
            }

            -count.reduce_sum() as u64
        })
        .sum()
}

#[inline(always)]
unsafe fn sum_cheats(scurr: usize, n: i16, seen: &[i16; 20 + (139 + 40) * SLINE]) -> i32x16 {
    const fn offset_distances() -> ([usize; 75], [[i16; 16]; 75]) {
        let mut offs = [0; 75];
        let mut dists = [[i16::MAX; 16]; 75];
        let mut pos = 0;

        let line = SLINE as isize;

        let (mut ls, mut le) = (-line * 20, -line * 20);
        let end = line * 20;
        let mut d = 1;
        while ls <= end {
            let mid = (ls + le) / 2;
            let base = (mid / line).abs();

            let mut ts = ls;
            while ts <= le {
                offs[pos] = ts as usize;

                let mut i = 0;
                while i < 16 && ts + i <= le {
                    let is = ts + i;
                    dists[pos][i as usize] = 100 + base as i16 + is.abs_diff(mid) as i16 - 1;
                    i += 1;
                }

                pos += 1;
                ts += 16;
            }

            if ls == -20 {
                d = -1;
            }

            ls = ls - d + line;
            le = le + d + line;
        }

        (offs, dists)
    }

    const OFFSETS: [usize; 75] = unsafe { transmute(offset_distances().0) };
    const DISTANCES: [i16x16; 75] = unsafe { transmute(offset_distances().1) };

    let mut count = i16x16::splat(0);
    for i in 0..25 {
        let offset = OFFSETS[i];
        let dists = DISTANCES[i];

        let base = scurr.wrapping_add(offset);
        let s = seen.get_unchecked(base..base + 16);
        let m = (i16x16::splat(n) - dists).simd_gt(i16x16::from_slice(s));
        count += m.to_int();
    }
    for i in 25..50 {
        let offset = OFFSETS[i];
        let dists = DISTANCES[i];

        let base = scurr.wrapping_add(offset);
        let s = seen.get_unchecked(base..base + 16);
        let m = (i16x16::splat(n) - dists).simd_gt(i16x16::from_slice(s));
        count += m.to_int();
    }
    for i in 50..75 {
        let offset = OFFSETS[i];
        let dists = DISTANCES[i];

        let base = scurr.wrapping_add(offset);
        let s = seen.get_unchecked(base..base + 16);
        let m = (i16x16::splat(n) - dists).simd_gt(i16x16::from_slice(s));
        count += m.to_int();
    }
    count.cast::<i32>()
}
