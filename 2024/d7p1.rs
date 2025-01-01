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

pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[inline(always)]
pub unsafe fn parse1(mut ptr: *const u8, buf: &mut [u32; 16], buf_len: &mut usize, goal: &mut u64) {
    let mut acc = *ptr as u64 - b'0' as u64;
    loop {
        ptr = ptr.add(1);
        let b = (*ptr as u64).wrapping_sub(b'0' as u64);
        if b >= 10 {
            break;
        }
        acc = 10 * acc + b;
    }
    *goal = acc;

    ptr = ptr.add(1);
    *buf_len = 0;

    while *ptr != b'\n' {
        ptr = ptr.add(1);
        let mut acc = *ptr as u32 - b'0' as u32;
        loop {
            ptr = ptr.add(1);
            let b = (*ptr as u32).wrapping_sub(b'0' as u32);
            if b >= 10 {
                break;
            }
            acc = 10 * acc + b;
        }
        *buf.get_unchecked_mut(*buf_len) = acc;
        *buf_len += 1;
    }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut lines = [0; 850];
    let mut lines_len = 1;
    let mut offset = 0;
    while offset + 32 < input.len() {
        let b = u8x32::from_slice(input.get_unchecked(offset..offset + 32));
        let mut m = b.simd_eq(u8x32::splat(b'\n')).to_bitmask();
        while m != 0 {
            let pos = m.trailing_zeros();
            m &= !(1 << pos);
            *lines.get_unchecked_mut(lines_len) = offset + pos as usize + 1;
            lines_len += 1;
        }
        offset += 32;
    }
    while offset + 1 < input.len() {
        if *input.get_unchecked(offset) == b'\n' {
            *lines.get_unchecked_mut(lines_len) = offset + 1;
            lines_len += 1;
        }
        offset += 1;
    }

    let sum = std::sync::atomic::AtomicU64::new(0);
    let chunk_len = lines.len().div_ceil(par::NUM_THREADS);
    par::par(|idx| {
        let chunk = lines.get_unchecked(chunk_len * idx..);
        let chunk = chunk.get_unchecked(..std::cmp::min(chunk.len(), chunk_len));

        let mut tot = 0;

        let mut max = [0; 16];
        let mut buf = [0; 16];
        let mut buf_len = 0;
        let mut goal = 0;

        let mut stack = [(0, 0); 32];
        let mut stack_len;

        for &i in chunk {
            parse1(input.as_ptr().add(i), &mut buf, &mut buf_len, &mut goal);

            let (add, mul) = (buf[0] + buf[1], buf[0] * buf[1]);
            max[0] = buf[0] as u64;
            max[1] = std::cmp::max(add, mul) as u64;
            let min2 = std::cmp::min(add, mul) as u64;
            for i in 2..buf_len {
                *max.get_unchecked_mut(i) = std::cmp::max(
                    *max.get_unchecked(i - 1) + 1,
                    *max.get_unchecked(i - 1) * *buf.get_unchecked(i) as u64,
                );
            }

            stack[0] = (goal, buf_len - 1);
            stack_len = 1;

            'outer: while stack_len != 0 {
                let (mut local_goal, mut i) = *stack.get_unchecked(stack_len - 1);
                stack_len -= 1;

                loop {
                    if local_goal >= *max.get_unchecked(i) {
                        if local_goal == *max.get_unchecked(i) {
                            tot += goal;
                            break 'outer;
                        }
                        continue 'outer;
                    }

                    if i == 1 {
                        if local_goal == min2 {
                            tot += goal;
                            break 'outer;
                        }
                        continue 'outer;
                    }

                    let n = *buf.get_unchecked(i) as u64;
                    std::hint::assert_unchecked(n != 0);

                    if local_goal.wrapping_sub(n) <= *max.get_unchecked(i - 1) {
                        *stack.get_unchecked_mut(stack_len) = (local_goal - n, i - 1);
                        stack_len += 1;
                    }

                    if local_goal % n == 0 {
                        local_goal /= n;
                        i -= 1;
                    } else {
                        break;
                    }
                }
            }
        }

        sum.fetch_add(tot, std::sync::atomic::Ordering::Relaxed);
    });

    sum.into_inner()
}

#[inline(always)]
pub unsafe fn parse2(
    input: &mut std::slice::Iter<u8>,
    buf: &mut [(u32, u32); 16],
    buf_len: &mut usize,
    goal: &mut u64,
) -> bool {
    if input.as_slice().len() > 0 {
        let mut acc = 0;
        while *input.as_slice().get_unchecked(0) != b':' {
            acc = 10 * acc + (input.as_slice().get_unchecked(0) - b'0') as u64;
            *input = input.as_slice().get_unchecked(1..).iter();
        }
        *input = input.as_slice().get_unchecked(1..).iter();
        *goal = acc;
    } else {
        return false;
    }

    *buf_len = 0;
    while *input.as_slice().get_unchecked(0) == b' ' {
        *input = input.as_slice().get_unchecked(1..).iter();
        let mut n = input.as_slice().get_unchecked(0).wrapping_sub(b'0') as u32;
        let mut pow10idx = 0;

        *input = input.as_slice().get_unchecked(1..).iter();
        let d = input.as_slice().get_unchecked(0).wrapping_sub(b'0');
        if d < 10 {
            n = 10 * n + d as u32;
            pow10idx = 1;
            *input = input.as_slice().get_unchecked(1..).iter();
            let d = input.as_slice().get_unchecked(0).wrapping_sub(b'0');
            if d < 10 {
                n = 10 * n + d as u32;
                pow10idx = 2;
                *input = input.as_slice().get_unchecked(1..).iter();
            }
        }
        *buf.get_unchecked_mut(*buf_len) = (n, pow10idx);
        *buf_len += 1;
    }
    *input = input.as_slice().get_unchecked(1..).iter();

    true
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let mut tot = 0;

    let mut max = [0; 16];
    let mut buf = [(0, 0); 16];
    let mut buf_len = 0;
    let mut goal = 0;

    let mut stack = [(0, 0); 32];
    let mut stack_len;

    let mut input = input.as_bytes().iter();
    loop {
        if !parse2(&mut input, &mut buf, &mut buf_len, &mut goal) {
            break;
        }

        max[0] = buf[0].0 as u64;
        for i in 1..buf_len {
            static LUT: [u64; 3] = [10, 100, 1000];
            let (n, l) = *buf.get_unchecked(i);
            let pow10 = *LUT.get_unchecked(l as usize);
            *max.get_unchecked_mut(i) = *max.get_unchecked(i - 1) * pow10 + n as u64;
        }
        let a2 = (buf[0].0 + buf[1].0) as u64;
        let m2 = (buf[0].0 * buf[1].0) as u64;

        stack[0] = (goal, buf_len - 1);
        stack_len = 1;

        'outer: while stack_len > 0 {
            let (mut local_goal, mut i) = *stack.get_unchecked(stack_len - 1);
            stack_len -= 1;

            loop {
                if local_goal >= *max.get_unchecked(i) {
                    if local_goal == *max.get_unchecked(i) {
                        tot += goal;
                        break 'outer;
                    }
                    continue 'outer;
                }

                if i == 1 {
                    if local_goal == a2 || local_goal == m2 {
                        tot += goal;
                        break 'outer;
                    }
                    continue 'outer;
                }

                let (n, l) = *buf.get_unchecked(i);
                let (n, l) = (n as u64, l as usize);
                std::hint::assert_unchecked(n != 0);

                if local_goal.wrapping_sub(n) <= *max.get_unchecked(i - 1) {
                    *stack.get_unchecked_mut(stack_len) = (local_goal - n, i - 1);
                    stack_len += 1;
                }

                use fastdiv::PrecomputedDivU64;
                static LUT: [PrecomputedDivU64; 3] = [
                    PrecomputedDivU64::new(10),
                    PrecomputedDivU64::new(100),
                    PrecomputedDivU64::new(1000),
                ];
                let pow10 = *LUT.get_unchecked(l);
                if local_goal > n && PrecomputedDivU64::is_multiple_of(local_goal - n, pow10) {
                    *stack.get_unchecked_mut(stack_len) =
                        (PrecomputedDivU64::fast_div(local_goal - n, pow10), i - 1);
                    stack_len += 1;
                }

                if local_goal % n == 0 {
                    local_goal /= n;
                    i -= 1;
                } else {
                    break;
                }
            }
        }
    }

    tot
}

mod fastdiv {
    #[inline]
    const fn mul128_u64(lowbits: u128, d: u64) -> u64 {
        let mut bottom_half = (lowbits & 0xFFFFFFFFFFFFFFFF) * d as u128;
        bottom_half >>= 64;
        let top_half = (lowbits >> 64) * d as u128;
        let both_halves = bottom_half + top_half;
        (both_halves >> 64) as u64
    }

    #[inline]
    const fn divide_128_max_by_64(divisor: u16) -> u128 {
        let divisor = divisor as u64;
        let quotient_hi = core::u64::MAX / divisor as u64;
        let remainder_hi = core::u64::MAX - quotient_hi * divisor;
        let quotient_lo = {
            let numerator_mid = (remainder_hi << 32) | core::u32::MAX as u64;
            let quotient_mid = numerator_mid / divisor;
            let remainder_mid = numerator_mid - quotient_mid * divisor;

            let numerator_lo = (remainder_mid << 32) | core::u32::MAX as u64;
            let quotient_lo = numerator_lo / divisor;

            (quotient_mid << 32) | quotient_lo
        };
        ((quotient_hi as u128) << 64) | (quotient_lo as u128)
    }

    #[inline]
    const fn compute_m_u64(d: u64) -> u128 {
        divide_128_max_by_64(d as u16) + 1
    }
    // for d > 1
    #[inline]
    const fn fastdiv_u64(a: u64, m: u128) -> u64 {
        mul128_u64(m, a)
    }
    #[inline]
    const fn is_divisible_u64(n: u64, m: u128) -> bool {
        (n as u128).wrapping_mul(m) <= m - 1
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct PrecomputedDivU64(u128);

    impl PrecomputedDivU64 {
        #[inline]
        pub const fn new(n: u64) -> Self {
            Self(compute_m_u64(n))
        }

        #[inline]
        pub fn fast_div(n: u64, precomputed: Self) -> u64 {
            fastdiv_u64(n, precomputed.0)
        }

        #[inline]
        pub fn is_multiple_of(n: u64, precomputed: Self) -> bool {
            is_divisible_u64(n, precomputed.0)
        }
    }
}

mod par {
    use std::sync::atomic::{AtomicPtr, Ordering};

    pub const NUM_THREADS: usize = 16;

    #[repr(align(64))]
    struct CachePadded<T>(T);

    static mut INIT: bool = false;

    static WORK: [CachePadded<AtomicPtr<()>>; NUM_THREADS] =
        [const { CachePadded(AtomicPtr::new(std::ptr::null_mut())) }; NUM_THREADS];

    #[inline(always)]
    fn submit<F: Fn(usize)>(f: &F) {
        unsafe {
            if !INIT {
                INIT = true;
                for idx in 1..NUM_THREADS {
                    thread_run(idx, f);
                }
            }
        }

        for i in 1..NUM_THREADS {
            WORK[i].0.store(f as *const F as *mut (), Ordering::Release);
        }
    }

    #[inline(always)]
    fn wait() {
        for i in 1..NUM_THREADS {
            loop {
                let ptr = WORK[i].0.load(Ordering::Acquire);
                if ptr.is_null() {
                    break;
                }
                std::hint::spin_loop();
            }
        }
    }

    fn thread_run<F: Fn(usize)>(idx: usize, _f: &F) {
        _ = std::thread::Builder::new().spawn(move || unsafe {
            let work = WORK.get_unchecked(idx);

            loop {
                let data = work.0.load(Ordering::Acquire);
                if !data.is_null() {
                    (&*data.cast::<F>())(idx);
                    work.0.store(std::ptr::null_mut(), Ordering::Release);
                }
                std::hint::spin_loop();
            }
        });
    }

    pub unsafe fn par<F: Fn(usize)>(f: F) {
        submit(&f);
        f(0);
        wait();
    }
}
