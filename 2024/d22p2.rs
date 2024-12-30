// Original by: giooschi
#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]
#![feature(array_chunks)]

use std::arch::x86_64::*;
use std::simd::prelude::*;

use rayon::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[inline(always)]
fn parse8(n: u64) -> u32 {
    use std::num::Wrapping as W;

    let mut n = W(n);
    let mask = W(0xFF | (0xFF << 32));
    let mul1 = W(100 + (1000000 << 32));
    let mul2 = W(1 + (10000 << 32));

    n = (n * W(10)) + (n >> 8);
    n = (((n & mask) * mul1) + (((n >> 16) & mask) * mul2)) >> 32;

    n.0 as u32
}

macro_rules! parse {
    ($ptr:ident) => {{
        let n = $ptr.cast::<u64>().read_unaligned();
        let len = _pext_u64(n, 0x1010101010101010).trailing_ones();
        let n = (n & 0x0F0F0F0F0F0F0F0F) << (8 * (8 - len));
        $ptr = $ptr.add(len as usize + 1);
        parse8(n)
    }};
}

const NUM_THREADS: usize = 16;
const NUM_SEQUENCES: usize = 19 * 19 * 19 * 19;
const NUM_COUNTS: usize = NUM_SEQUENCES * NUM_THREADS + (16 - NUM_SEQUENCES % 16) % 16;

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
"avx512vl""))]
unsafe fn inner_part2(input: &str) -> u64 {
    static mut NUMS: [u32; 4096] = [0; 4096];
    let nums = &mut NUMS;
    let mut nums_len = 0;

    let mut ptr = input.as_ptr();
    while ptr <= input.as_ptr().add(input.len() - 8) {
        let n = parse!(ptr);

        *nums.get_unchecked_mut(nums_len) = n;
        nums_len += 1;
    }

    if ptr != input.as_ptr().add(input.len()) {
        let len = input.as_ptr().add(input.len()).offset_from(ptr) - 1;
        let n = input
            .as_ptr()
            .add(input.len() - 1 - 8)
            .cast::<u64>()
            .read_unaligned();
        let n = (n & 0x0F0F0F0F0F0F0F0F) & (u64::MAX << (8 * (8 - len)));
        let n = parse8(n);

        *nums.get_unchecked_mut(nums_len) = n;
        nums_len += 1;
    };

    static mut COUNTS: [u8; NUM_COUNTS] = [0; NUM_COUNTS];
    COUNTS.fill(0);

    let nums = nums.get_unchecked_mut(..nums_len);

    let chunk_len = nums.len().div_ceil(NUM_THREADS).next_multiple_of(8);

    nums.par_chunks(chunk_len)
        .zip(COUNTS.par_chunks_mut(NUM_SEQUENCES))
        .with_max_len(1)
        .for_each(|(chunk, counts)| {
            let mut seen_sequences_bitset = vec![0; NUM_SEQUENCES];
            let mut chunks = chunk.array_chunks::<8>();

            for (i, c) in chunks.by_ref().enumerate() {
                if i != 0 {
                    seen_sequences_bitset.fill(0);
                }
                process_part2_totals(&c, counts, &mut seen_sequences_bitset);
            }

            let rem = chunks.remainder();
            if !rem.is_empty() {
                seen_sequences_bitset.fill(0);

                let mut remainder = [0; 8];
                remainder[..rem.len()].copy_from_slice(rem);
                process_part2_totals(&remainder, counts, &mut seen_sequences_bitset);
            }
        });

    let mut max = u16x16::splat(0);

    for i in 0..NUM_SEQUENCES.div_ceil(16) {
        let mut sum = u16x16::splat(0);
        for j in 0..NUM_THREADS {
            let b = u8x16::from_slice(
                COUNTS
                    .get_unchecked(NUM_SEQUENCES * j + 16 * i..)
                    .get_unchecked(..16),
            );
            sum += b.cast::<u16>();
        }
        max = max.simd_max(sum);
    }

    max.reduce_max() as u64
}

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
unsafe fn process_part2_totals(
    secrets: &[u32; 8],
    sequence_totals: &mut [u8],
    seen_sequences_bitset: &mut [u8],
) {
    let mut v = u32x8::from_array(*secrets);
    let mut prev = v % u32x8::splat(10);
    let mut history = u32x8::splat(0);

    // First 3 iterations
    for _ in 0..3 {
        v = perform_operation(v);
        let curr = v % u32x8::splat(10);
        history = (history << 8) | (u32x8::splat(9) + curr - prev);
        prev = curr;
    }

    for _ in 0..1997 {
        v = perform_operation(v);
        let curr = v % u32x8::splat(10);
        history = (history << 8) | (u32x8::splat(9) + curr - prev);
        let diff = history_to_diff_sequence(history);

        for k in 0..8 {
            let index = diff[k] as usize;
            let bit_offset = k;
            let bitset_ptr = seen_sequences_bitset.get_unchecked_mut(index);
            let bitset = *bitset_ptr;
            let curr_mask = -((bitset & (1 << bit_offset) == 0) as i32) as u32;
            *bitset_ptr = bitset | (1u8 << bit_offset);
            *sequence_totals.get_unchecked_mut(index) += (curr_mask & curr[k]) as u8;
        }

        prev = curr;
    }
}

#[inline(always)]
fn perform_operation(mut v: Simd<u32, 8>) -> Simd<u32, 8> {
    let mask = u32x8::splat((1 << 24) - 1);
    v ^= v << 6;
    v &= mask;
    v ^= v >> 5;
    v ^= v << 11;
    v &= mask;
    v
}

#[inline(always)]
unsafe fn history_to_diff_sequence(history: Simd<u32, 8>) -> i32x8 {
    let diff_intermediate =
        _mm256_maddubs_epi16(history.into(), i16x16::splat(19 * 256 + 1).into());
    let diff = _mm256_madd_epi16(
        diff_intermediate,
        i32x8::splat(19 * 19 * 256 * 256 + 1).into(),
    );
    diff.into()
}
