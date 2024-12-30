// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
"avx512vl""))]
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
"x={x} y={y}""
                );
            }
        }
    }

    collect(&input, &edges)
}

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
"avx512vl""))]
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
"got={}, expected={}, x={x}, y={y}"",
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
    let mut uf = [MaybeUninit::<(u32, u32)>::uninit(); 141 + 140 * 141];
    let uf = uf.as_mut_ptr().cast::<(u32, u32)>();
    let mut tot = 0;
    let mut off = 141;

    let mut goal = 141 + 140;

    while goal < 141 + 140 * 141 {
        while off < goal {
            let c = *input.get_unchecked(off);
            debug_assert_ne!(c, b'\n');
            let e = *extra.get_unchecked(off) as u8 as u64;

            if *input.get_unchecked(off - 1) == c {
                let mut root = off - 1;
                let mut t = &mut *uf.add(root);
                if t.0 == 0 {
                    root = t.1 as _;
                    t = &mut *uf.add(root);
                }
                debug_assert_ne!((*uf.add(root)).0, 0);
                t.0 += 1;
                tot += t.0 as u64 * e as u64 + t.1 as u64;
                t.1 += e as u32;
                *uf.add(off) = (0, root as _);

                if *input.get_unchecked(off - 141) == c {
                    'union: {
                        let mut root2 = off - 141;
                        if root != root2 {
                            let mut t2 = &mut *uf.add(root2);
                            while t2.0 == 0 {
                                root2 = t2.1 as _;
                                if root != root2 {
                                    t2 = &mut *uf.add(root2);
                                } else {
                                    break 'union;
                                }
                            }
                            debug_assert_ne!((*uf.add(root2)).0, 0);
                            tot += t.0 as u64 * t2.1 as u64 + t.1 as u64 * t2.0 as u64;
                            t.0 += t2.0;
                            t.1 += t2.1;
                            *t2 = (0, root as _);
                        }
                    }
                }
            } else if *input.get_unchecked(off - 141) == c {
                let mut root = off - 141;
                let mut t = &mut *uf.add(root);
                while t.0 == 0 {
                    root = t.1 as _;
                    t = &mut *uf.add(root);
                }
                debug_assert_ne!((*uf.add(root)).0, 0);
                t.0 += 1;
                tot += t.0 as u64 * e + t.1 as u64;
                t.1 += e as u32;
                *uf.add(off) = (0, root as _);
            } else {
                *uf.add(off) = (1, e as _);
                tot += e;
            }

            off += 1;
        }
        debug_assert_eq!(*input.get_unchecked(off), b'\n');
        off += 1;
        goal += 141;
    }

    tot
}
