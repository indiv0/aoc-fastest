// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let line_len = 1 + u8x64::from_slice(input.get_unchecked(..64))
        .simd_eq(u8x64::splat(b'\n'))
        .first_set()
        .unwrap_unchecked();
    let len = input.len() - 1;

    let mut chunk_lens = [input.len() / 64 / par::NUM_THREADS * 64; par::NUM_THREADS];
    for i in 0..input.len() / 64 % par::NUM_THREADS {
        chunk_lens[i] += 64;
    }
    chunk_lens[par::NUM_THREADS - 1] += input.len() % 64;

    let mut chunk_pos = [0; par::NUM_THREADS + 1];
    for i in 0..par::NUM_THREADS {
        chunk_pos[i + 1] = chunk_pos[i] + chunk_lens[i];
    }

    let acc = std::sync::atomic::AtomicU64::new(0);
    par::par(|idx| {
        let mut tot = 0;
        let mut offset = chunk_pos[idx];
        let end = chunk_pos[idx + 1];

        loop {
            let mut mask = if offset + 64 <= end {
                let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
                block.simd_eq(u8x64::splat(b'9')).to_bitmask()
            } else if offset < end {
                let block = u8x64::from_slice(input.get_unchecked(input.len() - 64..));
                block.simd_eq(u8x64::splat(b'9')).to_bitmask() >> (64 - (input.len() - offset))
            } else {
                break;
            };

            while mask != 0 {
                let o = mask.trailing_zeros();
                mask &= !(1 << o);

                let mut seen = u16x16::from_slice(&[u16::MAX; 16]);
                let mut seen_len = 0;

                let mut stack = [MaybeUninit::uninit(); 16];
                let mut stack_len = 0;

                let mut curr_o = offset + o as usize;
                let mut c = b'9';

                loop {
                    if c == b'0' {
                        if seen.simd_ne(u16x16::splat(curr_o as u16)).all() {
                            *seen.as_mut_array().get_unchecked_mut(seen_len) = curr_o as u16;
                            seen_len += 1;
                        }

                        if stack_len == 0 {
                            break;
                        }
                        stack_len -= 1;
                        (curr_o, c) = stack.get_unchecked(stack_len).assume_init();

                        continue;
                    }

                    let l = curr_o.wrapping_sub(1);
                    let r = curr_o.wrapping_add(1);
                    let t = curr_o.wrapping_sub(line_len);
                    let b = curr_o.wrapping_add(line_len);

                    macro_rules! handle {
                        ($new_o:expr) => {{
                            let new_o = $new_o;
                            if *input.get_unchecked(new_o) == c - 1 {
                                *stack.get_unchecked_mut(stack_len).as_mut_ptr() = (new_o, c - 1);
                                stack_len += 1;
                            }
                        }};
                    }

                    if t < len - 2 * line_len {
                        handle!(t);
                        handle!(b);
                        handle!(l);
                    } else {
                        if t < len {
                            handle!(t);
                            handle!(l);
                            if b < len {
                                handle!(b);
                            }
                        } else {
                            handle!(b);
                            if l < len {
                                handle!(l);
                            }
                        }
                    }
                    if *input.get_unchecked(r) == c - 1 {
                        (curr_o, c) = (r, c - 1);
                    } else if stack_len > 0 {
                        stack_len -= 1;
                        (curr_o, c) = stack.get_unchecked(stack_len).assume_init();
                    } else {
                        break;
                    }
                }

                tot += seen_len;
            }

            offset += 64;
        }

        acc.fetch_add(tot as u64, std::sync::atomic::Ordering::Relaxed);
    });
    acc.into_inner()
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
    pub fn wait(i: usize) {
        loop {
            let ptr = WORK[i].0.load(Ordering::Acquire);
            if ptr.is_null() {
                break;
            }
            std::hint::spin_loop();
        }
    }

    #[inline(always)]
    fn wait_all() {
        for i in 1..NUM_THREADS {
            wait(i);
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
        wait_all();
    }
}
