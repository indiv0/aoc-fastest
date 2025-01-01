// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]

use std::simd::cmp::SimdPartialEq;
use std::simd::u8x64;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

pub fn part1(input: &str) -> u32 {
    let mut iter = input.as_bytes().iter();

    #[inline(always)]
    unsafe fn search<const B: bool>(input: &[u8], sum: &mut u32, mut mask: u64) {
        while mask != 0 {
            let pos = mask.trailing_zeros();
            mask &= !(1 << pos);

            let mut i = pos as usize + 1;
            let mut l = 0;
            while (!B || i < input.len()) && input.get_unchecked(i).wrapping_sub(b'0') <= 9 {
                l = 10 * l + input.get_unchecked(i).wrapping_sub(b'0') as u32;
                i += 1;
            }

            if (B && i >= input.len()) || *input.get_unchecked(i) != b',' {
                continue;
            }
            i += 1;

            let mut r = 0;
            while (!B || i < input.len()) && input.get_unchecked(i).wrapping_sub(b'0') <= 9 {
                r = 10 * r + input.get_unchecked(i).wrapping_sub(b'0') as u32;
                i += 1;
            }

            if (B && i >= input.len()) || *input.get_unchecked(i) != b')' {
                continue;
            }

            *sum += l * r;
        }
    }

    let mut sum = 0;

    while iter.len() >= 72 {
        let bytes = u8x64::from_slice(iter.as_slice());

        let m = bytes.simd_eq(u8x64::splat(b'm')).to_bitmask();
        let u = bytes.simd_eq(u8x64::splat(b'u')).to_bitmask();
        let l = bytes.simd_eq(u8x64::splat(b'l')).to_bitmask();
        let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();

        let mask = (m << 3) & (u << 2) & (l << 1) & p;

        unsafe { search::<false>(iter.as_slice(), &mut sum, mask) };

        unsafe { iter = iter.as_slice().get_unchecked(61..).iter() };
    }

    if iter.len() >= 64 {
        let bytes = u8x64::from_slice(iter.as_slice());

        let m = bytes.simd_eq(u8x64::splat(b'm')).to_bitmask();
        let u = bytes.simd_eq(u8x64::splat(b'u')).to_bitmask();
        let l = bytes.simd_eq(u8x64::splat(b'l')).to_bitmask();
        let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();

        let mask = (m << 3) & (u << 2) & (l << 1) & p;

        unsafe { search::<true>(iter.as_slice(), &mut sum, mask) };

        unsafe { iter = iter.as_slice().get_unchecked(61..).iter() };
    }

    {
        let len = iter.len();
        let iter = unsafe { input.as_bytes().get_unchecked(input.len() - 64..).iter() };

        let bytes = u8x64::from_slice(iter.as_slice());

        let m = bytes.simd_eq(u8x64::splat(b'm')).to_bitmask();
        let u = bytes.simd_eq(u8x64::splat(b'u')).to_bitmask();
        let l = bytes.simd_eq(u8x64::splat(b'l')).to_bitmask();
        let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();

        let mask = (m << 3) & (u << 2) & (l << 1) & p & !((1 << (64 + 3 - len)) - 1);

        unsafe { search::<true>(iter.as_slice(), &mut sum, mask) };
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let mut iter = input.as_bytes().iter();

    #[inline(always)]
    unsafe fn search<const B: bool>(
        iter: std::slice::Iter<u8>,
        sum: &mut u32,
        enabled: &mut bool,
        mul_mask: u64,
        dont_mask: u64,
    ) -> usize {
        let mut mul_dont_mask = mul_mask | dont_mask;

        let mut pos = 0;

        while mul_dont_mask != 0 {
            let iter = iter.as_slice();

            pos = mul_dont_mask.trailing_zeros() as usize;
            mul_dont_mask &= !(1 << pos);

            if mul_mask & (1 << pos) != 0 {
                pos += 1;

                let mut l = 0;
                while (!B || pos < iter.len()) && iter.get_unchecked(pos).wrapping_sub(b'0') <= 9 {
                    l = 10 * l + iter.get_unchecked(pos).wrapping_sub(b'0') as u32;
                    pos += 1;
                }

                if (B && pos >= iter.len()) || *iter.get_unchecked(pos) != b',' {
                    continue;
                }
                pos += 1;

                let mut r = 0;
                while (!B || pos < iter.len()) && iter.get_unchecked(pos).wrapping_sub(b'0') <= 9 {
                    r = 10 * r + iter.get_unchecked(pos).wrapping_sub(b'0') as u32;
                    pos += 1;
                }

                if (B && pos >= iter.len()) || *iter.get_unchecked(pos) != b')' {
                    continue;
                }
                pos += 1;

                *sum += l * r;
            } else {
                *enabled = false;
                return pos as usize + 1;
            }
        }

        std::cmp::max(58, pos as usize)
    }

    let mut sum = 0;
    let mut enabled = true;

    while iter.len() > 6 {
        while enabled && iter.len() >= 69 {
            let bytes = u8x64::from_slice(iter.as_slice());

            let m = bytes.simd_eq(u8x64::splat(b'm')).to_bitmask();
            let u = bytes.simd_eq(u8x64::splat(b'u')).to_bitmask();
            let l = bytes.simd_eq(u8x64::splat(b'l')).to_bitmask();
            let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();
            let d = bytes.simd_eq(u8x64::splat(b'd')).to_bitmask();
            let o = bytes.simd_eq(u8x64::splat(b'o')).to_bitmask();
            let n = bytes.simd_eq(u8x64::splat(b'n')).to_bitmask();
            let a = bytes.simd_eq(u8x64::splat(b'\'')).to_bitmask();
            let t = bytes.simd_eq(u8x64::splat(b't')).to_bitmask();
            let q = bytes.simd_eq(u8x64::splat(b')')).to_bitmask();

            let mul_mask = (m << 3) & (u << 2) & (l << 1) & p & ((1 << (64 - 3)) - 1);
            let dont_mask = (d << 6) & (o << 5) & (n << 4) & (a << 3) & (t << 2) & (p << 1) & q;

            let offset = unsafe {
                search::<false>(iter.clone(), &mut sum, &mut enabled, mul_mask, dont_mask)
            };

            unsafe { iter = iter.as_slice().get_unchecked(offset..).iter() };
        }

        if enabled {
            if iter.len() >= 64 {
                let bytes = u8x64::from_slice(iter.as_slice());

                let m = bytes.simd_eq(u8x64::splat(b'm')).to_bitmask();
                let u = bytes.simd_eq(u8x64::splat(b'u')).to_bitmask();
                let l = bytes.simd_eq(u8x64::splat(b'l')).to_bitmask();
                let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();
                let d = bytes.simd_eq(u8x64::splat(b'd')).to_bitmask();
                let o = bytes.simd_eq(u8x64::splat(b'o')).to_bitmask();
                let n = bytes.simd_eq(u8x64::splat(b'n')).to_bitmask();
                let a = bytes.simd_eq(u8x64::splat(b'\'')).to_bitmask();
                let t = bytes.simd_eq(u8x64::splat(b't')).to_bitmask();
                let q = bytes.simd_eq(u8x64::splat(b')')).to_bitmask();

                let mul_mask = (m << 3) & (u << 2) & (l << 1) & p & ((1 << (64 - 3)) - 1);
                let dont_mask = (d << 6) & (o << 5) & (n << 4) & (a << 3) & (t << 2) & (p << 1) & q;

                let offset = unsafe {
                    search::<true>(iter.clone(), &mut sum, &mut enabled, mul_mask, dont_mask)
                };

                unsafe { iter = iter.as_slice().get_unchecked(offset..).iter() };
            }

            while enabled && iter.len() >= 8 {
                let len = iter.len();
                let iter2 = unsafe { input.as_bytes().get_unchecked(input.len() - 64..).iter() };
                let mask = !((1 << (64 + 3 - len)) - 1);

                let bytes = u8x64::from_slice(iter2.as_slice());

                let m = bytes.simd_eq(u8x64::splat(b'm')).to_bitmask();
                let u = bytes.simd_eq(u8x64::splat(b'u')).to_bitmask();
                let l = bytes.simd_eq(u8x64::splat(b'l')).to_bitmask();
                let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();
                let d = bytes.simd_eq(u8x64::splat(b'd')).to_bitmask();
                let o = bytes.simd_eq(u8x64::splat(b'o')).to_bitmask();
                let n = bytes.simd_eq(u8x64::splat(b'n')).to_bitmask();
                let a = bytes.simd_eq(u8x64::splat(b'\'')).to_bitmask();
                let t = bytes.simd_eq(u8x64::splat(b't')).to_bitmask();
                let q = bytes.simd_eq(u8x64::splat(b')')).to_bitmask();

                let mul_mask = (m << 3) & (u << 2) & (l << 1) & p & mask;
                let dont_mask =
                    (d << 6) & (o << 5) & (n << 4) & (a << 3) & (t << 2) & (p << 1) & q & mask;

                let offset = unsafe {
                    search::<true>(iter2.clone(), &mut sum, &mut enabled, mul_mask, dont_mask)
                };

                unsafe { iter = iter2.as_slice().get_unchecked(offset..).iter() };
            }
        }

        while !enabled && iter.len() >= 64 {
            let bytes = u8x64::from_slice(iter.as_slice());

            let d = bytes.simd_eq(u8x64::splat(b'd')).to_bitmask();
            let o = bytes.simd_eq(u8x64::splat(b'o')).to_bitmask();
            let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();
            let q = bytes.simd_eq(u8x64::splat(b')')).to_bitmask();
            let do_mask = (d << 3) & (o << 2) & (p << 1) & q;

            if do_mask != 0 {
                enabled = true;

                let pos = do_mask.trailing_zeros();
                unsafe { iter = iter.as_slice().get_unchecked(pos as usize + 1..).iter() };
            } else {
                unsafe { iter = iter.as_slice().get_unchecked(61..).iter() };
            }
        }

        if !enabled {
            let len = iter.len();
            let iter2 = unsafe { input.as_bytes().get_unchecked(input.len() - 64..).iter() };
            let mask = !((1 << (64 + 3 - len)) - 1);

            let bytes = u8x64::from_slice(iter2.as_slice());

            let d = bytes.simd_eq(u8x64::splat(b'd')).to_bitmask();
            let o = bytes.simd_eq(u8x64::splat(b'o')).to_bitmask();
            let p = bytes.simd_eq(u8x64::splat(b'(')).to_bitmask();
            let q = bytes.simd_eq(u8x64::splat(b')')).to_bitmask();
            let do_mask = (d << 3) & (o << 2) & (p << 1) & q & mask;

            if do_mask != 0 {
                enabled = true;

                let pos = do_mask.trailing_zeros();
                unsafe { iter = iter2.as_slice().get_unchecked(pos as usize + 1..).iter() };
            }
        }
    }

    sum
}
