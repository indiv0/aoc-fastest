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

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    #[cfg(debug_assertions)]
    let real_input = input;

    let mut edges = const {
        let mut edges = [0; 141 + 141 * 141 + 32];

        let mut i = 0;
        while i < 141 {
            edges[141 + 0 * 141 + i] = 1;
            i += 1;
        }

        edges[141 + 0] += 1;

        edges
    };

    let mut array = [MaybeUninit::<u8>::uninit(); 141 + 141 * 141 + 32];
    array.get_unchecked_mut(..141).fill(MaybeUninit::new(b'\n'));
    std::ptr::copy(
        input.as_ptr(),
        array.as_mut_ptr().add(141).cast(),
        140 * 141,
    );
    array
        .get_unchecked_mut(141 + 140 * 141..)
        .fill(MaybeUninit::new(b'\n'));
    let input = &mut *((&raw mut array) as *mut [u8; 141 + 141 * 141 + 32]);

    // TODO: bitmap to which neighbours are equal -> switch table

    let mut off = 141;
    while off < 141 + 140 * 141 {
        let o1 = off;
        let o2 = off + 1;
        let o3 = off + 141;

        let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
        let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));
        let b3 = u8x32::from_slice(input.get_unchecked(o3..o3 + 32));

        let t = b1.simd_ne(b2);
        let l = b1.simd_ne(b3);

        let mut s1 = i8x32::from_slice(edges.get_unchecked(o1..o1 + 32));
        s1 += t.to_int() & i8x32::splat(1);
        s1 += l.to_int() & i8x32::splat(1);
        *edges.get_unchecked_mut(o1) = s1[0];

        let mut s2 = s1.rotate_elements_left::<1>();
        s2[31] = *edges.get_unchecked(o2 + 32 - 1);
        s2 += t.to_int() & i8x32::splat(1);
        s2.copy_to_slice(edges.get_unchecked_mut(o2..o2 + 32));

        let mut s3 = i8x32::from_slice(edges.get_unchecked(o3..o3 + 32));
        s3 += l.to_int() & i8x32::splat(1);
        s3.copy_to_slice(edges.get_unchecked_mut(o3..o3 + 32));

        off += 32;
    }

    #[cfg(debug_assertions)]
    {
        let mut expected = [0; 140 * 141];
        for y in 0..140 {
            for x in 0..140 {
                let c = real_input[141 * y + x];
                let mut n = 0;
                if x == 0 || real_input[141 * y + (x - 1)] != c {
                    n += 1;
                }
                if x == 140 || real_input[141 * y + (x + 1)] != c {
                    n += 1;
                }
                if y == 0 || real_input[141 * (y - 1) + x] != c {
                    n += 1;
                }
                if y == 139 || real_input[141 * (y + 1) + x] != c {
                    n += 1;
                }
                expected[141 * y + x] = n;
            }
        }

        for y in 0..140 {
            for x in 0..140 {
                assert_eq!(
                    edges[141 + 141 * y + x],
                    expected[141 * y + x],
                    "x={x} y={y}"
                );
            }
        }
    }

    collect(&input, &edges)
}

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    #[cfg(debug_assertions)]
    let real_input = input;

    let mut corners = [0; 141 + 141 * 141 + 32];
    corners[141 + 0] += 1;

    let mut array = [MaybeUninit::<u8>::uninit(); 141 + 141 * 141 + 32];
    array.get_unchecked_mut(..141).fill(MaybeUninit::new(b'\n'));
    std::ptr::copy(
        input.as_ptr(),
        array.as_mut_ptr().add(141).cast(),
        140 * 141,
    );
    array
        .get_unchecked_mut(141 + 140 * 141..)
        .fill(MaybeUninit::new(b'\n'));
    let input = &mut *((&raw mut array) as *mut [u8; 141 + 141 * 141 + 32]);

    // TODO: bitmap to which neighbours are equal -> switch table

    let mut off = 0;
    while off < 141 + 140 * 141 {
        let o1 = off;
        let o2 = off + 1;
        let o3 = off + 141;
        let o4 = off + 1 + 141;

        let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
        let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));
        let b3 = u8x32::from_slice(input.get_unchecked(o3..o3 + 32));
        let b4 = u8x32::from_slice(input.get_unchecked(o4..o4 + 32));

        let t = b1.simd_ne(b2);
        let b = b3.simd_ne(b4);
        let l = b1.simd_ne(b3);
        let r = b2.simd_ne(b4);
        let d1 = b1.simd_ne(b4);
        let d2 = b2.simd_ne(b3);

        let mut s1 = i8x32::from_slice(corners.get_unchecked(o1..o1 + 32));
        s1 += (t.simd_eq(l) & (d1 | t)).to_int() & i8x32::splat(1);
        *corners.get_unchecked_mut(o1) = s1[0];

        let mut s2 = s1.rotate_elements_left::<1>();
        s2[31] = *corners.get_unchecked(o2 + 32 - 1);
        s2 += (t.simd_eq(r) & (d2 | t)).to_int() & i8x32::splat(1);
        s2.copy_to_slice(corners.get_unchecked_mut(o2..o2 + 32));

        let mut s3 = i8x32::from_slice(corners.get_unchecked(o3..o3 + 32));
        s3 += (b.simd_eq(l) & (d2 | b)).to_int() & i8x32::splat(1);
        *corners.get_unchecked_mut(o3) = s3[0];

        let mut s4 = s3.rotate_elements_left::<1>();
        s4[31] = *corners.get_unchecked(o4 + 32 - 1);
        s4 += (b.simd_eq(r) & (d1 | b)).to_int() & i8x32::splat(1);
        s4.copy_to_slice(corners.get_unchecked_mut(o4..o4 + 32));

        off += 32;
    }

    #[cfg(debug_assertions)]
    {
        let mut expected = [0; 140 * 141];
        for y in 0..140 {
            for x in 0..140 {
                let c = real_input[141 * y + x];
                let mut n = 0;

                let t = (y != 0).then(|| real_input[141 * (y - 1) + x]);
                let b = (y != 139).then(|| real_input[141 * (y + 1) + x]);
                let l = (x != 0).then(|| real_input[141 * y + (x - 1)]);
                let r = (x != 139).then(|| real_input[141 * y + (x + 1)]);

                let tl = (y != 0 && x != 0).then(|| real_input[141 * (y - 1) + (x - 1)]);
                let tr = (y != 0 && x != 139).then(|| real_input[141 * (y - 1) + (x + 1)]);
                let bl = (y != 139 && x != 0).then(|| real_input[141 * (y + 1) + (x - 1)]);
                let br = (y != 139 && x != 139).then(|| real_input[141 * (y + 1) + (x + 1)]);

                n += (t != Some(c) && l != Some(c)) as i8;
                n += (t != Some(c) && r != Some(c)) as i8;
                n += (b != Some(c) && l != Some(c)) as i8;
                n += (b != Some(c) && r != Some(c)) as i8;

                n += (t == Some(c) && l == Some(c) && tl != Some(c)) as i8;
                n += (t == Some(c) && r == Some(c) && tr != Some(c)) as i8;
                n += (b == Some(c) && l == Some(c) && bl != Some(c)) as i8;
                n += (b == Some(c) && r == Some(c) && br != Some(c)) as i8;

                expected[141 * y + x] = n;
            }
        }

        for y in 0..140 {
            for x in 0..140 {
                assert_eq!(
                    corners[141 + 141 * y + x],
                    expected[141 * y + x],
                    "got={}, expected={}, x={x}, y={y}",
                    corners[141 + 141 * y + x],
                    expected[141 * y + x],
                );
            }
        }
    }

    collect(&input, &corners)
}

#[inline(always)]
unsafe fn collect(input: &[u8; 141 + 141 * 141 + 32], extra: &[i8; 141 + 141 * 141 + 32]) -> u64 {
    let mut lens = [140 / par::NUM_THREADS; par::NUM_THREADS];
    for i in 0..140 % par::NUM_THREADS {
        lens[i] += 1;
    }
    let mut poss = [0; par::NUM_THREADS + 1];
    for i in 0..par::NUM_THREADS {
        poss[i + 1] = poss[i] + lens[i];
    }

    let mut uf = [MaybeUninit::<u64>::uninit(); 141 + 140 * 141];
    let uf = uf.as_mut_ptr().cast::<u64>();

    let acc = std::sync::atomic::AtomicU64::new(0);
    par::par(|idx| {
        let mut tot = 0;

        // First line: no union with top
        {
            let y = poss[idx];
            let mut prev_c = u8::MAX;
            let mut root = usize::MAX;
            for x in 0..140 {
                let off = 141 + 141 * y + x;

                let c = *input.get_unchecked(off);
                let e = *extra.get_unchecked(off) as u8 as u64;

                *uf.add(off) = root as u64;
                if c != prev_c {
                    prev_c = c;
                    root = off;
                    *uf.add(off) = 0;
                }

                let t = &mut *uf.add(root).cast::<[u32; 2]>();
                t[1] += 1;
                tot += t[1] as u64 * e as u64 + t[0] as u64;
                t[0] += e as u32;
            }
        }

        for y in poss[idx] + 1..poss[idx + 1] {
            let mut prev_c = u8::MAX;
            let mut root = usize::MAX;
            let mut top_prev = u32::MAX as u64;

            for x in 0..140 {
                let off = 141 + 141 * y + x;

                let c = *input.get_unchecked(off);
                let e = *extra.get_unchecked(off) as u8 as u64;

                *uf.add(off) = root as u64;
                if c != prev_c {
                    prev_c = c;
                    root = off;
                    *uf.add(off) = 0;
                }

                let t = &mut *uf.add(root).cast::<[u32; 2]>();
                t[1] += 1;
                tot += t[1] as u64 * e as u64 + t[0] as u64;
                t[0] += e as u32;

                let top = off - 141;
                let tt = *uf.add(top);
                if tt != top_prev {
                    top_prev = top as u64;
                }

                if (top_prev == top as u64 || root == off) && *input.get_unchecked(top) == c {
                    let mut tt_root = top;
                    while *uf.add(tt_root).cast::<u32>().add(1) == 0 {
                        tt_root = *uf.add(tt_root).cast::<u32>() as usize;
                    }

                    if tt_root != root {
                        let t2 = &mut *uf.add(tt_root).cast::<[u32; 2]>();

                        tot += t[1] as u64 * t2[0] as u64 + t[0] as u64 * t2[1] as u64;
                        t[1] += t2[1];
                        t[0] += t2[0];

                        *uf.add(tt_root) = root as u64;
                    }
                }
            }
        }

        const LEVELS: usize = par::NUM_THREADS.ilog2() as usize;
        const { assert!(1 << LEVELS == par::NUM_THREADS) };
        for i in 0..LEVELS {
            if idx & (1 << i) != 0 {
                break;
            }

            let next = idx + (1 << i);
            par::wait(next);
            let y = poss[next];

            let mut prev_c = u8::MAX;
            let mut root = usize::MAX;
            let mut top_prev = u32::MAX as u64;

            for x in 0..140 {
                let off = 141 + 141 * y + x;
                let top = off - 141;

                let c = *input.get_unchecked(off);

                let mut c_changed = false;
                if c != prev_c {
                    prev_c = c;
                    c_changed = true;

                    root = off;
                    while *uf.add(root).cast::<u32>().add(1) == 0 {
                        root = *uf.add(root).cast::<u32>() as usize;
                    }
                }

                let tt = *uf.add(top);
                if tt != top_prev {
                    top_prev = top as u64;
                }

                if (top_prev == top as u64 || c_changed) && *input.get_unchecked(top) == c {
                    let mut tt_root = top;
                    while *uf.add(tt_root).cast::<u32>().add(1) == 0 {
                        tt_root = *uf.add(tt_root).cast::<u32>() as usize;
                    }

                    if tt_root != root {
                        let t = &mut *uf.add(root).cast::<[u32; 2]>();
                        let t2 = &mut *uf.add(tt_root).cast::<[u32; 2]>();

                        tot += t[1] as u64 * t2[0] as u64 + t[0] as u64 * t2[1] as u64;
                        t[1] += t2[1];
                        t[0] += t2[0];

                        *uf.add(tt_root) = root as u64;
                    }
                }
            }
        }

        if idx & 1 == 0 {}

        acc.fetch_add(tot, std::sync::atomic::Ordering::Relaxed);
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
