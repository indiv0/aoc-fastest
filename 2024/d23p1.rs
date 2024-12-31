// Original by: giooschi
#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::arch::x86_64::*;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

// pub fn run(input: &str) -> &'static str {
//     part2(input)
// }

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> &'static str {
    unsafe { inner_part2(input) }
}

static LUT1: [(u64, u64); 26] = {
    let mut lut = [(u64::MAX, u64::MAX); 26];

    let off = (26 * (b't' - b'a') as usize) % 64;

    let mut i = 0;
    while i < 26 {
        let mut j = 0;
        while j < i {
            if off + j < 64 {
                lut[i].0 &= !(1 << (off + j));
            } else {
                lut[i].1 &= !(1 << (off + j - 64));
            }

            j += 1;
        }

        i += 1;
    }

    lut
};

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    const L: usize = 11;
    let mut sets = [[0u64; L]; 26 * 26 + 1];

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len() - 18);
    loop {
        let b = ptr.cast::<u8x16>().read_unaligned() - u8x16::splat(b'a');
        let mut b = simd_swizzle!(b, [0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 0, 0, 0, 0, 0]);
        b[11] = *ptr.add(16) - b'a';
        let b = u16x8::from(_mm_maddubs_epi16(
            b.into(),
            u8x16::from_array([26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 0, 0, 0, 0]).into(),
        ));

        let (n1, n2) = (b[0] as usize, b[1] as usize);
        *sets.get_unchecked_mut(n1).get_unchecked_mut(n2 / 64) |= 1 << (n2 % 64);
        *sets.get_unchecked_mut(n2).get_unchecked_mut(n1 / 64) |= 1 << (n1 % 64);

        let (n1, n2) = (b[2] as usize, b[3] as usize);
        *sets.get_unchecked_mut(n1).get_unchecked_mut(n2 / 64) |= 1 << (n2 % 64);
        *sets.get_unchecked_mut(n2).get_unchecked_mut(n1 / 64) |= 1 << (n1 % 64);

        let (n1, n2) = (b[4] as usize, b[5] as usize);
        *sets.get_unchecked_mut(n1).get_unchecked_mut(n2 / 64) |= 1 << (n2 % 64);
        *sets.get_unchecked_mut(n2).get_unchecked_mut(n1 / 64) |= 1 << (n1 % 64);

        ptr = ptr.add(18);
        if ptr > end {
            break;
        }
    }

    let end = input.as_ptr().add(input.len());
    while ptr != end {
        let b1 = *ptr.add(0) as usize - b'a' as usize;
        let b2 = *ptr.add(1) as usize - b'a' as usize;
        let b3 = *ptr.add(3) as usize - b'a' as usize;
        let b4 = *ptr.add(4) as usize - b'a' as usize;

        let n1 = 26 * b1 + b2;
        let n2 = 26 * b3 + b4;

        *sets.get_unchecked_mut(n1).get_unchecked_mut(n2 / 64) |= 1 << (n2 % 64);
        *sets.get_unchecked_mut(n2).get_unchecked_mut(n1 / 64) |= 1 << (n1 % 64);

        ptr = ptr.add(6);
    }

    let mut count = u16x16::splat(0);
    for b2 in 0..26 {
        let i = 26 * (b't' - b'a') as usize + b2;

        let mut s1 = sets.as_ptr().add(i).cast::<[u64; 12]>().read();
        let (m1, m2) = LUT1[b2];
        s1[7] &= m1;
        s1[8] &= m2;
        s1[11] = 0;

        let mut acc = u64x4::splat(0);

        for si in 0..L {
            while s1[si] != 0 {
                let o = s1[si].trailing_zeros() as usize;
                let j = 64 * si + o;
                s1[si] ^= 1 << o;

                let s2 = sets.as_ptr().add(j).cast::<[u64; 12]>().read();

                let sa = u64x4::from_slice(&s1[4 * 0..4 * 1]);
                let sb = u64x4::from_slice(&s1[4 * 1..4 * 2]);
                let sc = u64x4::from_slice(&s1[4 * 2..4 * 3]);
                let s2a = u64x4::from_slice(&s2[4 * 0..4 * 1]);
                let s2b = u64x4::from_slice(&s2[4 * 1..4 * 2]);
                let s2c = u64x4::from_slice(&s2[4 * 2..4 * 3]);

                let xa = sa & s2a;
                let xb = sb & s2b;
                let xc = sc & s2c;

                let m1 = u64x4::splat(u64::from_ne_bytes([0xAA; 8]));
                let (xah, xal) = ((xa & m1) >> 1, xa & (m1 >> 1));
                let (xbh, xbl) = ((xb & m1) >> 1, xb & (m1 >> 1));
                let (xch, xcl) = ((xc & m1) >> 1, xc & (m1 >> 1));
                let (xh, xl) = (xah + xbh + xch, xal + xbl + xcl);

                let m2 = u64x4::splat(u64::from_ne_bytes([0xCC; 8]));
                let (xhh, xhl) = ((xh & m2) >> 2, xh & (m2 >> 2));
                let (xlh, xll) = ((xl & m2) >> 2, xl & (m2 >> 2));
                let tot4 = xhh + xhl + xlh + xll;

                let m4 = u64x4::splat(u64::from_ne_bytes([0xF0; 8]));
                let (t4h, t4l) = ((tot4 & m4) >> 4, tot4 & (m4 >> 4));
                let tot8 = t4h + t4l;

                acc += tot8;
            }
        }

        let mhhhh = u64x4::splat(0xFF00FF00FF00FF00);
        let mllll = mhhhh >> 8;
        let (acch, accl) = ((acc & mhhhh) >> 8, acc & mllll);
        count += std::mem::transmute::<u64x4, u16x16>(acch + accl);
    }
    count.reduce_sum() as u64
}

static mut PART2_OUT: [u8; 13 * 2 + 12] = [b','; 13 * 2 + 12];

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> &'static str {
    let input = input.as_bytes();

    const L: usize = 11;
    let mut sets = [[0u64; L]; 26 * 26 + 1];

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());
    loop {
        let b1 = *ptr.add(0) as usize - b'a' as usize;
        let b2 = *ptr.add(1) as usize - b'a' as usize;
        let b3 = *ptr.add(3) as usize - b'a' as usize;
        let b4 = *ptr.add(4) as usize - b'a' as usize;

        let n1 = 26 * b1 + b2;
        let n2 = 26 * b3 + b4;

        *sets.get_unchecked_mut(n1).get_unchecked_mut(n2 / 64) |= 1 << (n2 % 64);
        *sets.get_unchecked_mut(n2).get_unchecked_mut(n1 / 64) |= 1 << (n1 % 64);

        ptr = ptr.add(6);
        if ptr == end {
            break;
        }
    }

    for i in 0..26 * 26 {
        let mut s1 = sets.as_ptr().add(i).cast::<[u64; 12]>().read();
        s1[11] = 0;
        if s1 == [0; 12] {
            continue;
        }

        macro_rules! handle {
            ($other:expr) => {
                'handle: {
                    let other = $other;

                    let s2 = sets.as_ptr().add(other).cast::<[u64; 12]>().read();

                    let sa = u64x4::from_slice(&s1[4 * 0..4 * 1]);
                    let sb = u64x4::from_slice(&s1[4 * 1..4 * 2]);
                    let sc = u64x4::from_slice(&s1[4 * 2..4 * 3]);
                    let s2a = u64x4::from_slice(&s2[4 * 0..4 * 1]);
                    let s2b = u64x4::from_slice(&s2[4 * 1..4 * 2]);
                    let s2c = u64x4::from_slice(&s2[4 * 2..4 * 3]);

                    let xa = sa & s2a;
                    let xb = sb & s2b;
                    let xc = sc & s2c;

                    let m1 = u64x4::splat(u64::from_ne_bytes([0xAA; 8]));
                    let (xah, xal) = ((xa & m1) >> 1, xa & (m1 >> 1));
                    let (xbh, xbl) = ((xb & m1) >> 1, xb & (m1 >> 1));
                    let (xch, xcl) = ((xc & m1) >> 1, xc & (m1 >> 1));
                    let (xh, xl) = (xah + xbh + xch, xal + xbl + xcl);

                    let m2 = u64x4::splat(u64::from_ne_bytes([0xCC; 8]));
                    let (xhh, xhl) = ((xh & m2) >> 2, xh & (m2 >> 2));
                    let (xlh, xll) = ((xl & m2) >> 2, xl & (m2 >> 2));
                    let tot4 = xhh + xhl + xlh + xll;

                    let m4 = u64x4::splat(u64::from_ne_bytes([0xF0; 8]));
                    let (t4h, t4l) = ((tot4 & m4) >> 4, tot4 & (m4 >> 4));
                    let tot8 = t4h + t4l;

                    let count = std::mem::transmute::<_, u8x32>(tot8).reduce_sum();

                    if count != 11 {
                        break 'handle;
                    }

                    let mut common = std::mem::transmute::<_, [u64; 12]>([xa, xb, xc]);

                    for i in 0..L {
                        let mut b = common[i];
                        while b != 0 {
                            let o = b.trailing_zeros() as usize;
                            let j = 64 * i + o;
                            b ^= 1 << o;

                            let s2 = sets.as_ptr().add(j).cast::<[u64; 12]>().read();

                            let sa = u64x4::from_slice(&s1[4 * 0..4 * 1]);
                            let sb = u64x4::from_slice(&s1[4 * 1..4 * 2]);
                            let sc = u64x4::from_slice(&s1[4 * 2..4 * 3]);
                            let s2a = u64x4::from_slice(&s2[4 * 0..4 * 1]);
                            let s2b = u64x4::from_slice(&s2[4 * 1..4 * 2]);
                            let s2c = u64x4::from_slice(&s2[4 * 2..4 * 3]);

                            let xa = sa & s2a;
                            let xb = sb & s2b;
                            let xc = sc & s2c;

                            let m1 = u64x4::splat(u64::from_ne_bytes([0xAA; 8]));
                            let (xah, xal) = ((xa & m1) >> 1, xa & (m1 >> 1));
                            let (xbh, xbl) = ((xb & m1) >> 1, xb & (m1 >> 1));
                            let (xch, xcl) = ((xc & m1) >> 1, xc & (m1 >> 1));
                            let (xh, xl) = (xah + xbh + xch, xal + xbl + xcl);

                            let m2 = u64x4::splat(u64::from_ne_bytes([0xCC; 8]));
                            let (xhh, xhl) = ((xh & m2) >> 2, xh & (m2 >> 2));
                            let (xlh, xll) = ((xl & m2) >> 2, xl & (m2 >> 2));
                            let tot4 = xhh + xhl + xlh + xll;

                            let m4 = u64x4::splat(u64::from_ne_bytes([0xF0; 8]));
                            let (t4h, t4l) = ((tot4 & m4) >> 4, tot4 & (m4 >> 4));
                            let tot8 = t4h + t4l;

                            let count = std::mem::transmute::<_, u8x32>(tot8).reduce_sum();

                            if count != 11 {
                                break 'handle;
                            }
                        }
                    }

                    *common.get_unchecked_mut(i / 64) |= 1 << (i % 64);
                    *common.get_unchecked_mut(other / 64) |= 1 << (other % 64);
                    let mut pos = 0;
                    for i in 0..L {
                        let mut b = common[i];
                        while b != 0 {
                            let o = b.trailing_zeros() as usize;
                            let j = 64 * i + o;
                            b ^= 1 << o;

                            *PART2_OUT.get_unchecked_mut(pos) = b'a' + (j / 26) as u8;
                            *PART2_OUT.get_unchecked_mut(pos + 1) = b'a' + (j % 26) as u8;
                            pos += 3;
                        }
                    }
                    return std::str::from_utf8_unchecked(&PART2_OUT);
                }
            };
        }

        let mut j = 0;
        let mut b = *s1.get_unchecked(j);

        while b == 0 {
            j += 1;
            b = *s1.get_unchecked(j);
        }
        handle!(64 * j + b.trailing_zeros() as usize);

        b &= !(1 << b.trailing_zeros());

        while b == 0 {
            j += 1;
            b = *s1.get_unchecked(j);
        }
        handle!(64 * j + b.trailing_zeros() as usize);
    }

    std::hint::unreachable_unchecked();
}
