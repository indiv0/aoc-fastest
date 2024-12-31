// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) as u64 }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

macro_rules! parse_pos {
    ($ptr:ident as $ty:ty) => {{
        let mut n = *$ptr as $ty - b'0' as $ty;
        $ptr = $ptr.add(1);
        if *$ptr as $ty >= b'0' as $ty {
            n = 10 * n + *$ptr as $ty - b'0' as $ty;
            $ptr = $ptr.add(1);
            if *$ptr as $ty >= b'0' as $ty {
                n = 10 * n + *$ptr as $ty - b'0' as $ty;
                $ptr = $ptr.add(1);
            }
        }
        n
    }};
}

macro_rules! parse {
    ($ptr:ident as $ty:ident - $m:expr) => {{
        if *$ptr == b'-' {
            $ptr = $ptr.add(1);
            $m as $ty - parse_pos!($ptr as $ty)
        } else {
            parse_pos!($ptr as $ty)
        }
    }};
}

const W: i64 = 101;
const H: i64 = 103;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut counts = [[0; 2]; 2];
    let mut ptr = input.as_ptr().wrapping_sub(1);
    let end = ptr.add(input.len());

    type Ty = u32;

    loop {
        ptr = ptr.add(3);
        let px = parse_pos!(ptr as Ty);
        ptr = ptr.add(1);
        let py = parse_pos!(ptr as Ty);
        ptr = ptr.add(3);
        let vx = parse!(ptr as Ty - W);
        ptr = ptr.add(1);
        let vy = parse!(ptr as Ty - H);

        let fx = fastdiv::fastmod_w((px + 100 * vx) as _) as Ty;
        let fy = fastdiv::fastmod_h((py + 100 * vy) as _) as Ty;

        if fx != W as Ty / 2 && fy != H as Ty / 2 {
            counts[(fx < W as Ty / 2) as usize][(fy < H as Ty / 2) as usize] += 1;
        }

        if ptr == end {
            break;
        }
    }

    counts[0][0] * counts[0][1] * counts[1][0] * counts[1][1]
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    type Ty = u16;

    #[repr(C, align(32))]
    struct Aligned<T>(T);

    let mut robots_x = Aligned([MaybeUninit::<Ty>::uninit(); 128]);
    let mut robots_y = Aligned([MaybeUninit::<Ty>::uninit(); 128]);
    let mut robots_vx = Aligned([MaybeUninit::<Ty>::uninit(); 128]);
    let mut robots_vy = Aligned([MaybeUninit::<Ty>::uninit(); 128]);
    let mut offset = 0;

    let mut ptr = input.as_ptr().wrapping_sub(1);

    loop {
        ptr = ptr.add(3);
        let px = parse_pos!(ptr as Ty);
        *robots_x.0.get_unchecked_mut(offset).as_mut_ptr() = px;

        ptr = ptr.add(1);
        let py = parse_pos!(ptr as Ty);
        *robots_y.0.get_unchecked_mut(offset).as_mut_ptr() = py;

        ptr = ptr.add(3);
        let vx = parse!(ptr as Ty - W);
        *robots_vx.0.get_unchecked_mut(offset).as_mut_ptr() = vx;

        ptr = ptr.add(1);
        let vy = parse!(ptr as Ty - H);
        *robots_vy.0.get_unchecked_mut(offset).as_mut_ptr() = vy;

        offset += 1;

        if offset == 128 {
            break;
        }
    }

    macro_rules! run_loop {
        ($p:ident, $v:ident | $s:ident) => {{
            let mut i = 0;
            loop {
                i += 1;

                let mut sum = u16x16::splat(0);
                let mut sum2 = u32x16::splat(0);

                for offset in 0..128 / 16 {
                    let p = *$p.0.as_mut_ptr().cast::<u16x16>().add(offset);
                    let v = *$v.0.as_ptr().cast::<u16x16>().add(offset);

                    let np = p + v;
                    let np = np.simd_min(np - u16x16::splat($s as Ty));

                    sum += np;
                    sum2 += np.cast::<u32>() * np.cast::<u32>();

                    *$p.0.as_mut_ptr().cast::<u16x16>().add(offset) = np;
                }

                let sum = sum.reduce_sum() as u64;
                let sum2 = sum2.reduce_sum() as u64;

                let var = sum2 - (sum * sum / 128);

                if var < 540 * 128 {
                    break i;
                }
            }
        }};
    }

    let mut i = i64::MAX;
    let j;
    let mut p = &mut robots_x;
    let mut v = &robots_vx;
    let mut c = W;

    loop {
        let n = run_loop!(p, v | c);
        if i == i64::MAX {
            i = n;
            p = &mut robots_y;
            v = &robots_vy;
            c = H;
        } else {
            j = n;
            break;
        }
    }

    (51 * (i * H + j * W) % (W * H)) as u64
}

mod fastdiv {
    #[inline(always)]
    const fn compute_m_u16(d: u16) -> u32 {
        (u32::MAX / d as u32) + 1
    }

    #[inline(always)]
    const fn mul64_u16(lowbits: u32, d: u16) -> u32 {
        (lowbits as u64 * d as u64 >> 32) as u32
    }

    #[inline(always)]
    const fn fastmod_u16(a: u16, m: u32, d: u16) -> u16 {
        let lowbits = m.wrapping_mul(a as u32);
        mul64_u16(lowbits, d) as u16
    }

    #[inline(always)]
    pub fn fastmod_w(a: u16) -> u16 {
        use super::W as D;
        const M: u32 = compute_m_u16(D as _);
        fastmod_u16(a, M, D as _)
    }

    #[inline(always)]
    pub fn fastmod_h(a: u16) -> u16 {
        use super::H as D;
        const M: u32 = compute_m_u16(D as _);
        fastmod_u16(a, M, D as _)
    }
}
