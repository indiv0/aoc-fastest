// Original by: caavik + giooschi
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

const WSIZE: usize = 16;
const LEN: usize = (1 << 24) - 1 + 2000 + WSIZE;
static mut NUM_TO_INDEX: [u32; 1 << 24] = [0; 1 << 24];
static mut DIGITS: [u8; LEN] = [0; LEN];
static mut DIFFS: [u16; LEN] = [0; LEN];
static mut LAST_SEEN: [u32; LEN] = [0; LEN];

unsafe fn build_tables() {
    let mut i = 0;
    let mut n = 1;

    let mut next_diff_id = 0;
    static mut DIFF_IDS: [u16; 19 * 19 * 19 * 19] = [u16::MAX; 19 * 19 * 19 * 19];
    static mut DIFF_TO_LAST_SEEN: [u32; 40951] = [0; 40951];

    while i < LEN {
        if i < (1 << 24) - 1 {
            NUM_TO_INDEX[n] = i as u32;
        }

        DIGITS[i] = (n % 10) as u8;

        if i >= 4 {
            let d1 = DIGITS[i - 4] as usize;
            let d2 = DIGITS[i - 3] as usize;
            let d3 = DIGITS[i - 2] as usize;
            let d4 = DIGITS[i - 1] as usize;
            let d5 = DIGITS[i - 0] as usize;

            let diff = [9 + d2 - d1, 9 + d3 - d2, 9 + d4 - d3, 9 + d5 - d4]
                .into_iter()
                .fold(0, |a, d| 19 * a + d);
            let mut diff_id = DIFF_IDS[diff];
            if diff_id == u16::MAX {
                diff_id = next_diff_id;
                DIFF_IDS[diff] = next_diff_id;
                next_diff_id += 1;
            }
            DIFFS[i] = diff_id;
            LAST_SEEN[i] = DIFF_TO_LAST_SEEN[diff_id as usize];
            DIFF_TO_LAST_SEEN[diff_id as usize] = i as u32;
        }

        n ^= (n << 6) & ((1 << 24) - 1);
        n ^= n >> 5;
        n ^= (n << 11) & ((1 << 24) - 1);

        i += 1;
    }
}

#[cfg_attr(any(target_os = "linux"), link_section = ".text.startup")]
unsafe extern "C" fn __ctor() {
    build_tables();
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(windows, link_section = ".CRT$XCU")]
static __CTOR: unsafe extern "C" fn() = __ctor;

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

const NUM_SEQUENCES: usize = 19 * 19 * 19 * 19;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
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

    const NUM_COUNTS: usize = NUM_SEQUENCES * par::NUM_THREADS + (16 - NUM_SEQUENCES % 16) % 16;
    static mut COUNTS: [u8; NUM_COUNTS] = [0; NUM_COUNTS];
    COUNTS.fill(0);

    let nums = nums.get_unchecked_mut(..nums_len);

    let mut chunk_lens = [nums.len() / 8 / par::NUM_THREADS * 8; par::NUM_THREADS];
    for i in 0..nums.len() / 8 % par::NUM_THREADS {
        chunk_lens[i] += 8;
    }
    chunk_lens[par::NUM_THREADS - 1] += nums.len() % 8;

    let mut chunk_pos = [0; par::NUM_THREADS + 1];
    for i in 0..par::NUM_THREADS {
        chunk_pos[i + 1] = chunk_pos[i] + chunk_lens[i];
    }

    par::par(|idx| {
        let chunk = nums.get_unchecked(chunk_pos[idx]..chunk_pos[idx + 1]);
        let counts = &mut *(&raw mut COUNTS).cast::<[u8; NUM_SEQUENCES]>().add(idx);

        for &c in chunk {
            let idx = *NUM_TO_INDEX.get_unchecked(c as usize) as usize;
            let mut curr = idx + 1;

            macro_rules! handle {
                ($min:expr) => {{
                    let digits = DIGITS.get_unchecked(curr..curr + WSIZE);
                    let digits = Simd::<u8, WSIZE>::from_slice(digits);

                    let diff = DIFFS.get_unchecked(curr..curr + WSIZE);

                    let last = LAST_SEEN.get_unchecked(curr..curr + WSIZE);
                    let last = Simd::<u32, WSIZE>::from_slice(last).cast::<i32>();
                    let mask = last.simd_lt(Simd::splat(idx as i32 + 4));
                    let to_sum = digits & mask.to_int().cast();

                    for i in $min..WSIZE {
                        *counts.get_unchecked_mut(diff[i] as usize) += to_sum[i];
                    }

                    curr += WSIZE;
                }};
            }

            handle!(3);
            for _ in 1..2000 / WSIZE {
                handle!(0);
            }
        }
    });

    let mut max = u16x16::splat(0);

    for i in 0..NUM_SEQUENCES.div_ceil(16) {
        let mut sum = u16x16::splat(0);
        for j in 0..par::NUM_THREADS {
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

    pub fn par<F: Fn(usize)>(f: F) {
        submit(&f);
        f(0);
        wait();
    }
}
