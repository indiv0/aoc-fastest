// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]

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

macro_rules! parse2 {
    ($ptr:ident as $ty:ident) => {{
        let n = (*$ptr as $ty - b'0' as $ty);
        let d = (*$ptr.add(1) as $ty).wrapping_sub(b'0' as $ty);
        $ptr = $ptr.add(2);
        10 * n + d
    }};
}
macro_rules! parse {
    ($ptr:ident as $ty:ident) => {{
        // TODO: SWAR?
        let mut n = 100 * (*$ptr as $ty - b'0' as $ty);
        n += 10 * (*$ptr.add(1) as $ty - b'0' as $ty);
        n += (*$ptr.add(2) as $ty - b'0' as $ty);
        let d = *$ptr.add(3) as $ty;
        $ptr = $ptr.add(4);
        if d >= b'0' as $ty {
            n = 10 * n + (d - b'0' as $ty);
            let d = *$ptr as $ty;
            $ptr = $ptr.add(1);
            if d >= b'0' as $ty {
                n = 10 * n + (d - b'0' as $ty);
                $ptr = $ptr.add(1);
            }
        }
        n
    }};
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut ptr = input.as_ptr().wrapping_sub(1);
    let end = input.as_ptr().add(input.len());
    let mut tot = 0;

    loop {
        ptr = ptr.add(13);
        let dxa = parse2!(ptr as u32) as i32;
        ptr = ptr.add(4);
        let dya = parse2!(ptr as u32) as i32;
        ptr = ptr.add(13);
        let dxb = parse2!(ptr as u32) as i32;
        ptr = ptr.add(4);
        let dyb = parse2!(ptr as u32) as i32;

        ptr = ptr.add(10);
        let x = parse!(ptr as u32) as i32;
        ptr = ptr.add(3);
        let y = parse!(ptr as u32) as i32;

        let det = dxa * dyb - dxb * dya;

        let d1 = x * dyb - dxb * y;
        let q1 = std::intrinsics::unchecked_div(d1, det);
        let r1 = std::intrinsics::unchecked_rem(d1, det);

        if r1 == 0 {
            let d2 = dxa * y - x * dya;
            let q2 = std::intrinsics::unchecked_div(d2, det);
            let r2 = std::intrinsics::unchecked_rem(d2, det);
            if r2 == 0 {
                tot += (3 * q1 + q2) as u64;
            }
        }

        if ptr == end {
            break;
        }
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let mut ptr = input.as_ptr().wrapping_sub(1);
    let end = input.as_ptr().add(input.len());
    let mut tot = 0;

    loop {
        ptr = ptr.add(13);
        let dxa = parse2!(ptr as u64) as i64;
        ptr = ptr.add(4);
        let dya = parse2!(ptr as u64) as i64;
        ptr = ptr.add(13);
        let dxb = parse2!(ptr as u64) as i64;
        ptr = ptr.add(4);
        let dyb = parse2!(ptr as u64) as i64;

        ptr = ptr.add(10);
        let x = parse!(ptr as u64) as i64 + 10000000000000;
        ptr = ptr.add(3);
        let y = parse!(ptr as u64) as i64 + 10000000000000;

        let det = dxa * dyb - dxb * dya;

        let d1 = x * dyb - dxb * y;
        let q1 = std::intrinsics::unchecked_div(d1, det);
        let r1 = std::intrinsics::unchecked_rem(d1, det);

        if r1 == 0 {
            let d2 = dxa * y - x * dya;
            let q2 = std::intrinsics::unchecked_div(d2, det);
            let r2 = std::intrinsics::unchecked_rem(d2, det);
            if r2 == 0 {
                tot += (3 * q1 + q2) as u64;
            }
        }

        if ptr == end {
            break;
        }
    }

    tot
}

