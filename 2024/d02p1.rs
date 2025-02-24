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

static LT_VALID: [bool; 256] = {
    let mut out = [false; 256];
    out[1] = true;
    out[2] = true;
    out[3] = true;
    out
};

#[inline(always)]
fn lt_valid(diff: i8) -> bool {
    LT_VALID[diff as u8 as usize]
}

static GT_VALID: [bool; 256] = {
    let mut out = [false; 256];
    out[253] = true;
    out[254] = true;
    out[255] = true;
    out
};

#[inline(always)]
fn gt_valid(diff: i8) -> bool {
    GT_VALID[diff as u8 as usize]
}

pub unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut lines = [0; 1000];
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

    let mut lens = [1000 / par1::NUM_THREADS; par1::NUM_THREADS];
    for i in 0..1000 % par1::NUM_THREADS {
        lens[i] += 1;
    }
    let mut poss = [0; par1::NUM_THREADS + 1];
    for i in 0..par1::NUM_THREADS {
        poss[i + 1] = poss[i] + lens[i];
    }

    let acc = std::sync::atomic::AtomicU64::new(0);
    par1::par(|idx| {
        unsafe fn read(input: &mut std::slice::Iter<u8>) -> (i8, u8) {
            let d1 = *input.next().unwrap_unchecked();
            let mut d2 = *input.next().unwrap_unchecked();

            let mut n = d1 - b'0';

            if d2 >= b'0' {
                n = 10 * n + (d2 - b'0');
                d2 = *input.next().unwrap_unchecked();
            }

            (n as i8, d2)
        }

        let mut count = 0;

        for i in poss[idx]..poss[idx + 1] {
            let l = *lines.get_unchecked(i);
            let mut input = input.get_unchecked(l..).iter();

            let (n1, _) = read(&mut input);
            let (n2, c2) = read(&mut input);

            let diff = n2 - n1;

            static VALID: [bool; 256] = {
                let mut out = [false; 256];
                out[253] = true;
                out[254] = true;
                out[255] = true;
                out[1] = true;
                out[2] = true;
                out[3] = true;
                out
            };

            let mut prev = n2;
            let mut ctrl = c2;
            let mut valid = VALID[diff as u8 as usize];

            if valid {
                if diff > 0 {
                    while valid && ctrl != b'\n' {
                        let (n, c) = read(&mut input);
                        let new_diff = n - prev;
                        (prev, ctrl) = (n, c);

                        valid &= lt_valid(new_diff);
                    }
                } else {
                    while valid && ctrl != b'\n' {
                        let (n, c) = read(&mut input);
                        let new_diff = n - prev;
                        (prev, ctrl) = (n, c);

                        valid &= gt_valid(new_diff);
                    }
                }
            }

            if valid {
                count += 1;
            }
        }

        acc.fetch_add(count, std::sync::atomic::Ordering::Relaxed);
    });
    acc.into_inner()
}

pub unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut lines = [0; 1000];
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

    let mut lens = [1000 / par2::NUM_THREADS; par2::NUM_THREADS];
    for i in 0..1000 % par2::NUM_THREADS {
        lens[i] += 1;
    }
    let mut poss = [0; par2::NUM_THREADS + 1];
    for i in 0..par2::NUM_THREADS {
        poss[i + 1] = poss[i] + lens[i];
    }

    let acc = std::sync::atomic::AtomicU64::new(0);
    par2::par(|idx| {
        unsafe fn read(input: &mut std::slice::Iter<u8>) -> (i8, u8) {
            let d1 = *input.next().unwrap_unchecked();
            let mut d2 = *input.next().unwrap_unchecked();

            let mut n = d1 - b'0';

            if d2 >= b'0' {
                n = 10 * n + (d2 - b'0');
                d2 = *input.next().unwrap_unchecked();
            }

            (n as i8, d2)
        }

        let mut count = 0;

        for i in poss[idx]..poss[idx + 1] {
            let l = *lines.get_unchecked(i);
            let mut input = input.get_unchecked(l..).iter();

            let (n1, _) = read(&mut input);
            let (n2, c2) = read(&mut input);

            let diff = n2 - n1;

            let mut prevprev = n1;
            let mut prev = n2;
            let mut ctrl = c2;

            static STATE_MAP: [[u8; 4]; 4] =
                [[2, 1, 0, 0], [4, 3, 3, 3], [4, 3, 4, 3], [4, 4, 3, 3]];

            let mut lt_st = if lt_valid(diff) { 0 } else { 1 };
            let mut gt_st = if gt_valid(diff) { 0 } else { 1 };

            while lt_st != 4 && gt_st != 4 && ctrl != b'\n' {
                let (n, c) = read(&mut input);
                let p_diff = n - prev;
                let pp_diff = n - prevprev;

                let lt_idx = 2 * (lt_valid(p_diff) as usize) + lt_valid(pp_diff) as usize;
                let gt_idx = 2 * (gt_valid(p_diff) as usize) + gt_valid(pp_diff) as usize;

                lt_st = *STATE_MAP
                    .get_unchecked(lt_st as usize)
                    .get_unchecked(lt_idx);
                gt_st = *STATE_MAP
                    .get_unchecked(gt_st as usize)
                    .get_unchecked(gt_idx);

                (prevprev, prev, ctrl) = (prev, n, c);
            }

            if lt_st != 4 {
                while lt_st == 0 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !lt_valid(p_diff) {
                        let pp_diff = n - prevprev;
                        let lt_idx = 2 * (lt_valid(p_diff) as usize) + lt_valid(pp_diff) as usize;

                        lt_st = *STATE_MAP
                            .get_unchecked(lt_st as usize)
                            .get_unchecked(lt_idx);
                    }

                    (prevprev, prev, ctrl) = (prev, n, c);
                }

                if ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;
                    let pp_diff = n - prevprev;

                    let lt_idx = 2 * (lt_valid(p_diff) as usize) + lt_valid(pp_diff) as usize;

                    lt_st = *STATE_MAP
                        .get_unchecked(lt_st as usize)
                        .get_unchecked(lt_idx);

                    (prev, ctrl) = (n, c);
                }

                while lt_st == 3 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !lt_valid(p_diff) {
                        lt_st = 4;
                    }

                    (prev, ctrl) = (n, c);
                }
            } else if gt_st != 4 {
                while gt_st == 0 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !gt_valid(p_diff) {
                        let pp_diff = n - prevprev;
                        let gt_idx = 2 * (gt_valid(p_diff) as usize) + gt_valid(pp_diff) as usize;

                        gt_st = *STATE_MAP
                            .get_unchecked(gt_st as usize)
                            .get_unchecked(gt_idx);
                    }

                    (prevprev, prev, ctrl) = (prev, n, c);
                }

                if ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;
                    let pp_diff = n - prevprev;

                    let gt_idx = 2 * (gt_valid(p_diff) as usize) + gt_valid(pp_diff) as usize;

                    gt_st = *STATE_MAP
                        .get_unchecked(gt_st as usize)
                        .get_unchecked(gt_idx);

                    (prev, ctrl) = (n, c);
                }

                while gt_st == 3 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !gt_valid(p_diff) {
                        gt_st = 4;
                    }

                    (prev, ctrl) = (n, c);
                }
            }

            if lt_st != 4 || gt_st != 4 {
                count += 1;
            }
        }

        acc.fetch_add(count, std::sync::atomic::Ordering::Relaxed);
    });
    acc.into_inner()
}

mod par1 {
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

mod par2 {
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
