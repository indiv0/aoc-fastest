// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[inline(always)]
pub unsafe fn parse1(
    input: &mut std::slice::Iter<u8>,
    buf: &mut [u32; 16],
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

        *input = input.as_slice().get_unchecked(1..).iter();
        let d = input.as_slice().get_unchecked(0).wrapping_sub(b'0');
        if d < 10 {
            n = 10 * n + d as u32;
            *input = input.as_slice().get_unchecked(1..).iter();
            let d = input.as_slice().get_unchecked(0).wrapping_sub(b'0');
            if d < 10 {
                n = 10 * n + d as u32;
                *input = input.as_slice().get_unchecked(1..).iter();
            }
        }
        *buf.get_unchecked_mut(*buf_len) = n;
        *buf_len += 1;
    }
    *input = input.as_slice().get_unchecked(1..).iter();

    true
}

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
"avx512vl""))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut tot = 0;

    let mut max = [0; 16];
    let mut buf = [0; 16];
    let mut buf_len = 0;
    let mut goal = 0;

    let mut stack = [(0, 0); 32];
    let mut stack_len;

    let mut input = input.as_bytes().iter();
    loop {
        if !parse1(&mut input, &mut buf, &mut buf_len, &mut goal) {
            break;
        }

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

    tot
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

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
"avx512vl""))]
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
