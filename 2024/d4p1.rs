// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]

use std::ops::Range;
use std::simd::cmp::SimdPartialEq;
use std::simd::u8x64;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let input = input.as_bytes();

    const R1: Range<usize> = 0..64;
    const R2: Range<usize> = 61..125;
    const R3: Range<usize> = (140 - 64)..140;
    const M2V: u64 = u64::MAX & (!0b111) & ((1 << 61) - 1);
    const M3: u64 = !((1u64 << (64 - (140 - 122))) - 1);

    #[inline(always)]
    fn check_horiz<const N: usize>(mut x: u64, m: u64, a: u64, mut s: u64) -> u32 {
        if N == 3 {
            x &= M3;
            s &= M3;
        }
        let mut tot = 0;
        tot |= x & (m << 1) & (a << 2) & (s << 3);
        tot |= s & (a << 1) & (m << 2) & (x << 3);
        tot.count_ones()
    }

    #[inline(always)]
    fn check_vert<const N: usize>(
        pppx: u64,
        ppps: u64,
        ppm: u64,
        ppa: u64,
        pm: u64,
        pa: u64,
        mut cx: u64,
        mut cs: u64,
    ) -> u32 {
        if N == 2 {
            cx &= M2V;
            cs &= M2V;
        } else if N == 3 {
            cx &= M3;
            cs &= M3;
        }

        ((pppx & ppm & pa & cs) | (ppps & ppa & pm & cx)).count_ones()
    }

    #[inline(always)]
    fn check_diag<const N: usize>(
        mut pppx: u64,
        mut ppps: u64,
        ppm: u64,
        ppa: u64,
        pm: u64,
        pa: u64,
        mut cx: u64,
        mut cs: u64,
    ) -> u32 {
        if N == 3 {
            pppx &= M3;
            ppps &= M3;
            cx &= M3;
            cs &= M3;
        }

        let mut dl = 0;
        dl |= cx & (pm << 1) & (ppa << 2) & (ppps << 3);
        dl |= cs & (pa << 1) & (ppm << 2) & (pppx << 3);

        let mut dr = 0;
        dr |= cx & (pm >> 1) & (ppa >> 2) & (ppps >> 3);
        dr |= cs & (pa >> 1) & (ppm >> 2) & (pppx >> 3);

        dl.count_ones() + dr.count_ones()
    }

    macro_rules! extract {
        ($x:pat, $m:pat, $a:pat, $s:pat = $line:expr) => {
            let line = u8x64::from_slice($line);
            let $x = line.simd_eq(u8x64::splat(b'X')).to_bitmask();
            let $m = line.simd_eq(u8x64::splat(b'M')).to_bitmask();
            let $a = line.simd_eq(u8x64::splat(b'A')).to_bitmask();
            let $s = line.simd_eq(u8x64::splat(b'S')).to_bitmask();
        };
    }

    let mut count = 0;

    let line = &input[0 * 141..][..140];
    extract!(mut pppx1, pppm1, pppa1, mut ppps1 = &line[R1]);
    count += check_horiz::<1>(pppx1, pppm1, pppa1, ppps1);
    extract!(mut pppx2, pppm2, pppa2, mut ppps2 = &line[R2]);
    count += check_horiz::<2>(pppx2, pppm2, pppa2, ppps2);
    extract!(mut pppx3, pppm3, pppa3, mut ppps3 = &line[R3]);
    count += check_horiz::<3>(pppx3, pppm3, pppa3, ppps3);

    let line = &input[1 * 141..][..140];
    extract!(mut ppx1, mut ppm1, mut ppa1, mut pps1 = &line[R1]);
    count += check_horiz::<1>(ppx1, ppm1, ppa1, pps1);
    extract!(mut ppx2, mut ppm2, mut ppa2, mut pps2 = &line[R2]);
    count += check_horiz::<2>(ppx2, ppm2, ppa2, pps2);
    extract!(mut ppx3, mut ppm3, mut ppa3, mut pps3 = &line[R3]);
    count += check_horiz::<3>(ppx3, ppm3, ppa3, pps3);

    let line = &input[2 * 141..][..140];
    extract!(mut px1, mut pm1, mut pa1, mut ps1 = &line[R1]);
    count += check_horiz::<1>(px1, pm1, pa1, ps1);
    extract!(mut px2, mut pm2, mut pa2, mut ps2 = &line[R2]);
    count += check_horiz::<2>(px2, pm2, pa2, ps2);
    extract!(mut px3, mut pm3, mut pa3, mut ps3 = &line[R3]);
    count += check_horiz::<3>(px3, pm3, pa3, ps3);

    for line in input[3 * 141..].chunks_exact(141) {
        extract!(cx1, cm1, ca1, cs1 = &line[R1]);
        count += check_horiz::<1>(cx1, cm1, ca1, cs1);
        count += check_vert::<1>(pppx1, ppps1, ppm1, ppa1, pm1, pa1, cx1, cs1);
        count += check_diag::<1>(pppx1, ppps1, ppm1, ppa1, pm1, pa1, cx1, cs1);
        (pppx1, ppx1, px1) = (ppx1, px1, cx1);
        (ppm1, pm1) = (pm1, cm1);
        (ppa1, pa1) = (pa1, ca1);
        (ppps1, pps1, ps1) = (pps1, ps1, cs1);

        extract!(cx2, cm2, ca2, cs2 = &line[R2]);
        count += check_horiz::<2>(cx2, cm2, ca2, cs2);
        count += check_vert::<2>(pppx2, ppps2, ppm2, ppa2, pm2, pa2, cx2, cs2);
        count += check_diag::<2>(pppx2, ppps2, ppm2, ppa2, pm2, pa2, cx2, cs2);
        (pppx2, ppx2, px2) = (ppx2, px2, cx2);
        (ppm2, pm2) = (pm2, cm2);
        (ppa2, pa2) = (pa2, ca2);
        (ppps2, pps2, ps2) = (pps2, ps2, cs2);

        extract!(cx3, cm3, ca3, cs3 = &line[R3]);
        count += check_horiz::<3>(cx3, cm3, ca3, cs3);
        count += check_vert::<3>(pppx3, ppps3, ppm3, ppa3, pm3, pa3, cx3, cs3);
        count += check_diag::<3>(pppx3, ppps3, ppm3, ppa3, pm3, pa3, cx3, cs3);
        (pppx3, ppx3, px3) = (ppx3, px3, cx3);
        (ppm3, pm3) = (pm3, cm3);
        (ppa3, pa3) = (pa3, ca3);
        (ppps3, pps3, ps3) = (pps3, ps3, cs3);
    }

    count
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    let mut iter = input.as_bytes().iter();

    let mut count = 0;

    while iter.len() >= 2 * 141 + 2 + 64 {
        macro_rules! body {
            () => {{
                let s = iter.as_slice().get_unchecked(..2 * 141 + 2 + 64);

                let tl = u8x64::from_slice(s);
                let tr = u8x64::from_slice(&s[2..]);
                let c = u8x64::from_slice(&s[141 + 1..]);
                let bl = u8x64::from_slice(&s[2 * 141..]);
                let br = u8x64::from_slice(&s[2 * 141 + 2..]);

                let tb = (tl ^ br).simd_eq(u8x64::splat(b'M' ^ b'S'));
                let a = c.simd_eq(u8x64::splat(b'A'));
                let bt = (bl ^ tr).simd_eq(u8x64::splat(b'M' ^ b'S'));

                count += (tb & a & bt).to_bitmask().count_ones();

                iter = iter.as_slice().get_unchecked(64..).iter();
            }};
        }

        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
        body!();
    }

    count
}

