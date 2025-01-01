// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]

use std::ops::Range;
use std::simd::cmp::SimdPartialEq;
use std::simd::u8x64;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

pub fn part1(input: &str) -> u32 {
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

    let mut count = 0;

    let line = &input[0 * 141..][..140];
    let ppp1 = u8x64::from_slice(&line[R1]);
    let mut pppx1 = ppp1.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let pppm1 = ppp1.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let pppa1 = ppp1.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut ppps1 = ppp1.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<1>(pppx1, pppm1, pppa1, ppps1);
    let ppp2 = u8x64::from_slice(&line[R2]);
    let mut pppx2 = ppp2.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let pppm2 = ppp2.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let pppa2 = ppp2.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut ppps2 = ppp2.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<2>(pppx2, pppm2, pppa2, ppps2);
    let ppp3 = u8x64::from_slice(&line[R3]);
    let mut pppx3 = ppp3.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let pppm3 = ppp3.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let pppa3 = ppp3.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut ppps3 = ppp3.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<3>(pppx3, pppm3, pppa3, ppps3);

    let line = &input[1 * 141..][..140];
    let pp1 = u8x64::from_slice(&line[R1]);
    let mut ppx1 = pp1.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mut ppm1 = pp1.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut ppa1 = pp1.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut pps1 = pp1.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<1>(ppx1, ppm1, ppa1, pps1);
    let pp2 = u8x64::from_slice(&line[R2]);
    let mut ppx2 = pp2.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mut ppm2 = pp2.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut ppa2 = pp2.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut pps2 = pp2.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<2>(ppx2, ppm2, ppa2, pps2);
    let pp3 = u8x64::from_slice(&line[R3]);
    let mut ppx3 = pp3.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mut ppm3 = pp3.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut ppa3 = pp3.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut pps3 = pp3.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<3>(ppx3, ppm3, ppa3, pps3);

    let line = &input[2 * 141..][..140];
    let p1 = u8x64::from_slice(&line[R1]);
    let mut px1 = p1.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mut pm1 = p1.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut pa1 = p1.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut ps1 = p1.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<1>(px1, pm1, pa1, ps1);
    let p2 = u8x64::from_slice(&line[R2]);
    let mut px2 = p2.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mut pm2 = p2.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut pa2 = p2.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut ps2 = p2.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<2>(px2, pm2, pa2, ps2);
    let p3 = u8x64::from_slice(&line[R3]);
    let mut px3 = p3.simd_eq(u8x64::splat(b'X')).to_bitmask();
    let mut pm3 = p3.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut pa3 = p3.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let mut ps3 = p3.simd_eq(u8x64::splat(b'S')).to_bitmask();
    count += check_horiz::<3>(px3, pm3, pa3, ps3);

    for line in input[3 * 141..].chunks_exact(141) {
        let c1 = u8x64::from_slice(&line[R1]);
        let cx1 = c1.simd_eq(u8x64::splat(b'X')).to_bitmask();
        let cm1 = c1.simd_eq(u8x64::splat(b'M')).to_bitmask();
        let ca1 = c1.simd_eq(u8x64::splat(b'A')).to_bitmask();
        let cs1 = c1.simd_eq(u8x64::splat(b'S')).to_bitmask();
        count += check_horiz::<1>(cx1, cm1, ca1, cs1);
        count += check_vert::<1>(pppx1, ppps1, ppm1, ppa1, pm1, pa1, cx1, cs1);
        count += check_diag::<1>(pppx1, ppps1, ppm1, ppa1, pm1, pa1, cx1, cs1);
        (pppx1, ppx1, px1) = (ppx1, px1, cx1);
        (ppm1, pm1) = (pm1, cm1);
        (ppa1, pa1) = (pa1, ca1);
        (ppps1, pps1, ps1) = (pps1, ps1, cs1);

        let c2 = u8x64::from_slice(&line[R2]);
        let cx2 = c2.simd_eq(u8x64::splat(b'X')).to_bitmask();
        let cm2 = c2.simd_eq(u8x64::splat(b'M')).to_bitmask();
        let ca2 = c2.simd_eq(u8x64::splat(b'A')).to_bitmask();
        let cs2 = c2.simd_eq(u8x64::splat(b'S')).to_bitmask();
        count += check_horiz::<2>(cx2, cm2, ca2, cs2);
        count += check_vert::<2>(pppx2, ppps2, ppm2, ppa2, pm2, pa2, cx2, cs2);
        count += check_diag::<2>(pppx2, ppps2, ppm2, ppa2, pm2, pa2, cx2, cs2);
        (pppx2, ppx2, px2) = (ppx2, px2, cx2);
        (ppm2, pm2) = (pm2, cm2);
        (ppa2, pa2) = (pa2, ca2);
        (ppps2, pps2, ps2) = (pps2, ps2, cs2);

        let c3 = u8x64::from_slice(&line[R3]);
        let cx3 = c3.simd_eq(u8x64::splat(b'X')).to_bitmask();
        let cm3 = c3.simd_eq(u8x64::splat(b'M')).to_bitmask();
        let ca3 = c3.simd_eq(u8x64::splat(b'A')).to_bitmask();
        let cs3 = c3.simd_eq(u8x64::splat(b'S')).to_bitmask();
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

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut count = 0;

    const R1: Range<usize> = 0..64;
    const R2: Range<usize> = 62..126;
    const R3: Range<usize> = (140 - 64)..140;
    const MASK_A3: u64 = !((1u64 << (64 - (140 - 125))) - 1);

    let line = &input[..140];
    let pp1 = u8x64::from_slice(&line[R1]);
    let mut ppm1 = pp1.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut pps1 = pp1.simd_eq(u8x64::splat(b'S')).to_bitmask();
    let pp2 = u8x64::from_slice(&line[R2]);
    let mut ppm2 = pp2.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut pps2 = pp2.simd_eq(u8x64::splat(b'S')).to_bitmask();
    let pp3 = u8x64::from_slice(&line[R3]);
    let mut ppm3 = pp3.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut pps3 = pp3.simd_eq(u8x64::splat(b'S')).to_bitmask();

    let line = &input[141..][..140];
    let p1 = u8x64::from_slice(&line[R1]);
    let mut pm1 = p1.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut ps1 = p1.simd_eq(u8x64::splat(b'S')).to_bitmask();
    let mut pa1 = p1.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let p2 = u8x64::from_slice(&line[R2]);
    let mut pm2 = p2.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut ps2 = p2.simd_eq(u8x64::splat(b'S')).to_bitmask();
    let mut pa2 = p2.simd_eq(u8x64::splat(b'A')).to_bitmask();
    let p3 = u8x64::from_slice(&line[R3]);
    let mut pm3 = p3.simd_eq(u8x64::splat(b'M')).to_bitmask();
    let mut ps3 = p3.simd_eq(u8x64::splat(b'S')).to_bitmask();
    let mut pa3 = p3.simd_eq(u8x64::splat(b'A')).to_bitmask();

    #[inline(always)]
    fn check_chunk(ppm: u64, pps: u64, pa: u64, cm: u64, cs: u64) -> u32 {
        let mut tot = 0;
        tot |= (ppm >> 1) & (pps << 1) & pa & (cm >> 1) & (cs << 1);
        tot |= (ppm >> 1) & (ppm << 1) & pa & (cs >> 1) & (cs << 1);
        tot |= (pps >> 1) & (ppm << 1) & pa & (cs >> 1) & (cm << 1);
        tot |= (pps >> 1) & (pps << 1) & pa & (cm >> 1) & (cm << 1);
        tot.count_ones()
    }

    for line in input[141 * 2..].chunks_exact(141) {
        let c1 = u8x64::from_slice(&line[R1]);
        let cm1 = c1.simd_eq(u8x64::splat(b'M')).to_bitmask();
        let cs1 = c1.simd_eq(u8x64::splat(b'S')).to_bitmask();
        let ca1 = c1.simd_eq(u8x64::splat(b'A')).to_bitmask();
        count += check_chunk(ppm1, pps1, pa1, cm1, cs1);
        (ppm1, pps1, pm1, ps1, pa1) = (pm1, ps1, cm1, cs1, ca1);

        let c2 = u8x64::from_slice(&line[R2]);
        let cm2 = c2.simd_eq(u8x64::splat(b'M')).to_bitmask();
        let cs2 = c2.simd_eq(u8x64::splat(b'S')).to_bitmask();
        let ca2 = c2.simd_eq(u8x64::splat(b'A')).to_bitmask();
        count += check_chunk(ppm2, pps2, pa2, cm2, cs2);
        (ppm2, pps2, pm2, ps2, pa2) = (pm2, ps2, cm2, cs2, ca2);

        let c3 = u8x64::from_slice(&line[R3]);
        let cm3 = c3.simd_eq(u8x64::splat(b'M')).to_bitmask();
        let cs3 = c3.simd_eq(u8x64::splat(b'S')).to_bitmask();
        let ca3 = c3.simd_eq(u8x64::splat(b'A')).to_bitmask();
        count += check_chunk(ppm3, pps3, pa3 & MASK_A3, cm3, cs3);
        (ppm3, pps3, pm3, ps3, pa3) = (pm3, ps3, cm3, cs3, ca3);
    }

    count
}
