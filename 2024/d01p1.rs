// Original by: doge
#![feature(portable_simd)]
use std::simd::Simd;

const N: usize = 1000;
const LANES: usize = 8; // SIMD width for `Simd<u32, 8>`

#[inline(always)]
unsafe fn radix_sort_u17<const N: usize>(arr: &mut [u32; N]) {
    let mut cnt_lo: [u16; 256] = [0; 256];
    let mut cnt_hi: [u16; 512] = [0; 512];

    let len = arr.len();
    let chunks = len / LANES * LANES;

    // Counting frequencies using SIMD
    for i in (0..chunks).step_by(LANES) {
        // Load 8 u32 elements into a SIMD vector
        let data = Simd::<u32, LANES>::from_slice(&arr[i..i + LANES]);

        // Extract lower 8 bits
        let lo = data & Simd::splat(0xFFu32);
        // Extract higher 9 bits
        let hi = data >> Simd::splat(8u32);

        // Update counts arrays
        for j in 0..LANES {
            *cnt_lo.get_unchecked_mut(lo[j] as usize) += 1;
            *cnt_hi.get_unchecked_mut(hi[j] as usize) += 1;
        }
    }

    // Process any remaining elements
    for &x in &arr[chunks..] {
        *cnt_lo.get_unchecked_mut((x & 0xFF) as usize) += 1;
        *cnt_hi.get_unchecked_mut((x >> 8) as usize) += 1;
    }

    // Compute exclusive prefix sums
    {
        let mut sum = 0u16;
        for count in cnt_lo.iter_mut() {
            let temp = *count;
            *count = sum;
            sum += temp;
        }
    }
    {
        let mut sum = 0u16;
        for count in cnt_hi.iter_mut() {
            let temp = *count;
            *count = sum;
            sum += temp;
        }
    }

    // First redistribution pass (lower 8 bits)
    let mut buf = [0u32; N];
    {
        for &x in arr.iter() {
            let idx = (x & 0xFF) as usize;
            let dest = cnt_lo.get_unchecked_mut(idx);
            *buf.get_unchecked_mut(*dest as usize) = x;
            *dest += 1;
        }
    }

    // Second redistribution pass (higher 9 bits)
    {
        for &x in buf.iter() {
            let idx = (x >> 8) as usize;
            let dest = cnt_hi.get_unchecked_mut(idx);
            *arr.get_unchecked_mut(*dest as usize) = x;
            *dest += 1;
        }
    }
}

#[inline(always)]
fn parse_5b(s: &[u8]) -> u32 {
    // Optimize by unrolling the loop and using direct subtraction to convert ASCII digits to numbers
    unsafe {
        let s0 = *s.get_unchecked(0) as u32;
        let s1 = *s.get_unchecked(1) as u32;
        let s2 = *s.get_unchecked(2) as u32;
        let s3 = *s.get_unchecked(3) as u32;
        let s4 = *s.get_unchecked(4) as u32;

        (s0 * 10000 + s1 * 1000 + s2 * 100 + s3 * 10 + s4) - 533328
    }
}

pub fn run(s: &[u8]) -> i64 {
    let mut left = [0; N];
    let mut right = [0; N];

    for i in 0..N {
        left[i] = parse_5b(&s[i * 14..]);
        right[i] = parse_5b(&s[i * 14 + 8..]);
    }

    unsafe {
        radix_sort_u17(&mut left);
        radix_sort_u17(&mut right);
    }

    left.iter()
        .zip(&right)
        .map(|(a, &b)| a.abs_diff(b))
        .sum::<u32>() as i64
}

#[test]
fn d1p1() {
    assert_eq!(run(include_bytes!("./../input/day1.txt")), 2192892);
}
