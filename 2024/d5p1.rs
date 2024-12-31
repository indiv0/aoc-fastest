// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]

use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdInt;
use std::simd::ptr::SimdMutPtr;
use std::simd::{mask8x32, simd_swizzle, u64x4, u8x32, u8x64, usizex4, Simd};

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { inner_part2(input) }
}

static NLMASK: [u8; 32] = {
    let mut mask = [0; 32];
    mask[0 * 6] = b'\n';
    mask[1 * 6] = b'\n';
    mask[2 * 6] = b'\n';
    mask[3 * 6] = b'\n';
    mask
};

static RULE_MUL_BLOCK: [u8; 32] = {
    let mut block = [0; 32];
    let mut i = 0;
    while i < 4 {
        block[6 * i] = 10;
        block[6 * i + 1] = 1;
        block[6 * i + 3] = 10;
        block[6 * i + 4] = 1;
        i += 1;
    }
    block
};

static SWIZZLE_BLOCK: [usize; 32] = {
    let mut block = [31; 32];
    let mut i = 0;
    while i < 4 {
        block[6 * i] = 6 * i + 1;
        block[6 * i + 3] = 6 * i + 4;
        i += 1;
    }
    block
};

static SWIZZLE_LEFT: [usize; 32] = {
    let mut block = [31; 32];
    let mut i = 0;
    while i < 4 {
        block[8 * i] = 6 * i;
        i += 1;
    }
    block
};

static SWIZZLE_RIGHT: [usize; 32] = {
    let mut block = [31; 32];
    let mut i = 0;
    while i < 4 {
        block[8 * i] = 6 * i + 3;
        i += 1;
    }
    block
};

#[inline(always)]
unsafe fn read_rules(rules: &mut [u128; 100], iter: &mut std::slice::Iter<u8>) {
    loop {
        let block = u8x32::from_slice(iter.as_slice().get_unchecked(..32));
        if block.simd_eq(u8x32::from_slice(&NLMASK)).any() {
            break;
        }

        let digits = (block - u8x32::splat(b'0')) * u8x32::from_slice(&RULE_MUL_BLOCK);
        let nums = digits + simd_swizzle!(digits, SWIZZLE_BLOCK);
        let left = std::mem::transmute::<_, usizex4>(simd_swizzle!(nums, SWIZZLE_LEFT));
        let right = std::mem::transmute::<_, u64x4>(simd_swizzle!(nums, SWIZZLE_RIGHT));
        let right_big = right.simd_ge(u64x4::splat(64));
        let idx = left * usizex4::splat(2) + (right_big.to_int().cast() & usizex4::splat(1));
        let ptr = Simd::splat(rules.as_mut_ptr().cast::<u64>()).wrapping_add(idx);
        let shift_amt = right & u64x4::splat(0b111111);
        let to_store = u64x4::splat(1) << shift_amt;

        for (&ptr, &to_store) in std::iter::zip(ptr.as_array(), to_store.as_array()) {
            *ptr |= to_store;
        }

        *iter = iter.as_slice().get_unchecked(24..).iter();
    }

    while *iter.as_slice().get_unchecked(0) != b'\n' {
        let c = iter.as_slice().get_unchecked(..5);
        let d1 = ((c[0] - b'0') as usize * 10) + ((c[1] - b'0') as usize);
        let d2 = ((c[3] - b'0') as usize * 10) + ((c[4] - b'0') as usize);
        *rules.get_unchecked_mut(d1) |= 1 << d2;
        *iter = iter.as_slice().get_unchecked(6..).iter();
    }
    *iter = iter.as_slice().get_unchecked(1..).iter();
}

static UPDATE_MUL_BLOCK: [u8; 32] = {
    let mut block = [0; 32];
    let mut i = 0;
    while i < 11 {
        block[3 * i] = 10;
        block[3 * i + 1] = 1;
        i += 1;
    }
    block
};

const SWIZZLE_HI: [usize; 32] = {
    let mut block = [2; 32];
    let mut i = 0;
    while i < 11 {
        block[i] = 3 * i;
        i += 1;
    }
    block
};
const SWIZZLE_LO: [usize; 32] = {
    let mut block = [2; 32];
    let mut i = 0;
    while i < 11 {
        block[i] = 3 * i + 1;
        i += 1;
    }
    block
};

static MASKS: [u64; 23] = {
    let mut masks = [0; 23];
    let mut i = 0;
    while i < 23 {
        masks[i] = (1 << i) - 1;
        i += 1;
    }
    masks
};

#[inline(always)]
unsafe fn parse11(iter: &mut std::slice::Iter<u8>, update: &mut [u8], rem: &mut usize) {
    let len = std::cmp::min(*rem, 11);
    *rem -= len;

    let block = u8x32::from_slice(iter.as_slice().get_unchecked(..32));

    let digits = block - u8x32::splat(b'0');
    let nums = digits * u8x32::from_slice(&UPDATE_MUL_BLOCK);
    let nums = simd_swizzle!(nums, SWIZZLE_HI) + simd_swizzle!(nums, SWIZZLE_LO);

    let mask = mask8x32::from_bitmask(*MASKS.get_unchecked(len));

    nums.store_select_unchecked(update, mask);

    *iter = iter.as_slice().get_unchecked(len * 3..).iter();
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let mut iter = input.as_bytes().iter();

    let mut rules = [0u128; 100];
    read_rules(&mut rules, &mut iter);

    let mut tot = 0;
    let mut buf = [0; 24];
    let mut buf_len;
    let mut mask;

    'outer: while iter.len() >= 72 {
        buf_len = 0;
        mask = 0;

        let len = 8 + u8x64::from_slice(iter.as_slice().get_unchecked(8..8 + 64))
            .simd_eq(u8x64::splat(b'\n'))
            .first_set()
            .unwrap_unchecked();

        let mut c_iter = iter.as_slice().get_unchecked(..len + 1).iter();

        iter = iter.as_slice().get_unchecked(len + 1..).iter();

        while !c_iter.as_slice().is_empty() {
            let c = c_iter.as_slice();

            let n = (*c.get_unchecked(0) - b'0') * 10 + (*c.get_unchecked(1) - b'0');
            *buf.get_unchecked_mut(buf_len) = n;
            buf_len += 1;

            if *rules.get_unchecked(n as usize) & mask != 0 {
                continue 'outer;
            }

            mask |= 1 << n;
            c_iter = c_iter.as_slice().get_unchecked(3..).iter();
        }

        tot += *buf.get_unchecked(buf_len / 2) as u32;
    }

    buf_len = 0;
    mask = 0;
    'outer: while !iter.as_slice().is_empty() {
        let c = iter.as_slice();

        let n = (*c.get_unchecked(0) - b'0') * 10 + (*c.get_unchecked(1) - b'0');
        *buf.get_unchecked_mut(buf_len) = n;
        buf_len += 1;

        if *rules.get_unchecked(n as usize) & mask != 0 {
            while *iter.as_slice().get_unchecked(2) != b'\n' {
                iter = iter.as_slice().get_unchecked(3..).iter();
            }
            iter = iter.as_slice().get_unchecked(3..).iter();
            buf_len = 0;
            mask = 0;
            continue 'outer;
        }

        mask |= 1 << n;

        if *c.get_unchecked(2) == b'\n' {
            tot += *buf.get_unchecked(buf_len / 2) as u32;
            buf_len = 0;
            mask = 0;
        }

        iter = iter.as_slice().get_unchecked(3..).iter();
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    let mut iter = input.as_bytes().iter();

    let mut rules = [0u128; 100];
    read_rules(&mut rules, &mut iter);

    let mut tot = 0;
    let mut buf = [0; 24];
    let mut buf_len;
    let mut mask;
    let mut valid;

    while iter.len() >= 72 {
        let bytes_len = 9 + u8x64::from_slice(iter.as_slice().get_unchecked(8..8 + 64))
            .simd_eq(u8x64::splat(b'\n'))
            .first_set()
            .unwrap_unchecked();
        buf_len = bytes_len / 3;
        let mut rem = buf_len;

        while rem != 0 {
            let last_idx = buf_len - rem;
            parse11(&mut iter, buf.get_unchecked_mut(last_idx..), &mut rem);
        }

        valid = true;
        mask = 1 << *buf.get_unchecked(0);
        for i in 1..buf_len {
            let n = *buf.get_unchecked(i);
            valid &= *rules.get_unchecked(n as usize) & mask == 0;
            mask |= 1 << n;
        }

        if !valid {
            for i in 0..buf_len {
                let succs = *rules.get_unchecked(*buf.get_unchecked(i) as usize) & mask;
                if succs.count_ones() == buf_len as u32 / 2 {
                    tot += *buf.get_unchecked(i) as u32;
                    break;
                }
            }
        }
    }

    buf_len = 0;
    mask = 0;
    valid = true;

    for c in iter.as_slice().chunks_exact(3) {
        let n = (c[0] - b'0') * 10 + (c[1] - b'0');
        *buf.get_unchecked_mut(buf_len) = n;
        buf_len += 1;
        valid &= *rules.get_unchecked(n as usize) & mask == 0;
        mask |= 1 << n;

        if c[2] == b'\n' {
            if !valid {
                for i in 0..buf_len {
                    let succs = *rules.get_unchecked(*buf.get_unchecked(i) as usize) & mask;
                    if succs.count_ones() == buf_len as u32 / 2 {
                        tot += *buf.get_unchecked(i) as u32;
                        break;
                    }
                }
            }
            buf_len = 0;
            mask = 0;
            valid = true;
        }
    }

    tot
}
