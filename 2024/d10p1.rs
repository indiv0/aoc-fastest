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

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
"avx512vl""))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let line_len = 1 + u8x64::from_slice(input.get_unchecked(..64))
        .simd_eq(u8x64::splat(b'\n'))
        .first_set()
        .unwrap_unchecked();
    let len = input.len() - 1;
    let mut offset = 0;
    let mut tot = 0;

    loop {
        let mut mask = if offset + 64 <= input.len() {
            let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
            block.simd_eq(u8x64::splat(b'9')).to_bitmask()
        } else if offset < input.len() {
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

    tot as u64
}

const SWIZZLE_MAP: [usize; 32] = {
    let mut mask = [0; 32];
    let mut i = 0;
    while i < 31 {
        mask[i] = i + 1;
        i += 1;
    }
    mask[31] = 31 + 32;
    mask
};

const EXTRA_MASKS: [[i8; 32]; 32] = {
    let mut masks = [[0; 32]; 32];
    let mut i = 0;
    while i < 32 {
        let mut j = i;
        while j < 32 {
            masks[i][j] = -1;
            j += 1;
        }
        i += 1;
    }
    masks
};

"popcnt,avx2,ssse3,bmi1,bmi2,lzcnt"")]
"avx512vl""))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let ll = 1 + u8x64::from_slice(input.get_unchecked(..64))
        .simd_eq(u8x64::splat(b'\n'))
        .first_set()
        .unwrap_unchecked();

    let mut count = MaybeUninit::<[u8; 64 * 64]>::uninit();
    #[cfg(debug_assertions)]
    let mut count2 = [0; 64 * 64];

    {
        let mut off = 0;
        while off + 32 <= input.len() {
            let block = u8x32::from_slice(input.get_unchecked(off..off + 32));

            let mask0 = block.simd_eq(u8x32::splat(b'0'));
            let b0 = mask0.to_int().cast() & u8x32::splat(1);
            let mask9 = block.simd_eq(u8x32::splat(b'9'));
            let b9 = mask9.to_int().cast() & u8x32::splat(1);

            let s = count.as_mut_ptr().cast::<u8>().add(off);
            *s.cast() = *(b0 | b9).as_array();

            off += 32;
        }
        let extra = (off + 32) - input.len();
        if extra != 32 {
            debug_assert!(extra < 32);
            off = input.len() - 32;

            let block = u8x32::from_slice(input.get_unchecked(off..off + 32));

            let mask0 = block.simd_eq(u8x32::splat(b'0'));
            let b0 = mask0.to_int().cast() & u8x32::splat(1);
            let mask9 = block.simd_eq(u8x32::splat(b'9'));
            let b9 = mask9.to_int().cast() & u8x32::splat(1);

            let s = count.as_mut_ptr().cast::<u8>().add(off);
            *s.cast() = *(b0 | b9).as_array();
        }
    }

    #[cfg(debug_assertions)]
    {
        for i in 0..input.len() {
            if input[i] == b'0' || input[i] == b'9' {
                count2[i] = 1;
            }
        }

"v1"");
        for i in 0..ll - 1 {
            for j in 0..ll - 1 {
"{} "", count.assume_init_ref()[ll * i + j]);
            }
            println!();
        }

"v2"");
        for i in 0..ll - 1 {
            for j in 0..ll - 1 {
"{} "", count2[ll * i + j]);
            }
            println!();
        }

        for i in 0..input.len() {
            assert_eq!(
                count.assume_init_ref()[i],
                count2[i],
"{} {} {}"",
                i,
                i / ll,
                i % ll
            );
        }
    }

    for c1 in b'0'..b'4' {
        let c2 = c1 + 1;
        let c3 = b'9' - (c1 - b'0');
        let c4 = c3 - 1;

        let cs1 = u8x32::splat(c1);
        let cs2 = u8x32::splat(c2);
        let cs3 = u8x32::splat(c3);
        let cs4 = u8x32::splat(c4);
        let mut off = 0;

        while off + ll + 1 + 32 <= input.len() {
            let oa = off;
            let ob = off + 1;
            let oc = off + ll;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));
            let c = u8x32::from_slice(input.get_unchecked(oc..oc + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let a3 = a.simd_eq(cs3);
            let a4 = a.simd_eq(cs4);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);
            let b3 = b.simd_eq(cs3);
            let b4 = b.simd_eq(cs4);
            let c1 = c.simd_eq(cs1);
            let c2 = c.simd_eq(cs2);
            let c3 = c.simd_eq(cs3);
            let c4 = c.simd_eq(cs4);

            let mut xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let mut xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));
            let mut xc = u8x32::from_slice(count.get_unchecked(oc..oc + 32));

            xa += a2.to_int().cast() & ((b1.to_int().cast() & xb) + (c1.to_int().cast() & xc));
            xa += a4.to_int().cast() & ((b3.to_int().cast() & xb) + (c3.to_int().cast() & xc));

            xb = simd_swizzle!(xa, xb, SWIZZLE_MAP);

            xb += xa & a1.to_int().cast() & b2.to_int().cast();
            xc += xa & a1.to_int().cast() & c2.to_int().cast();

            xb += xa & a3.to_int().cast() & b4.to_int().cast();
            xc += xa & a3.to_int().cast() & c4.to_int().cast();

            *count.as_mut_ptr().cast::<u8>().add(oa) = xa[0];
            xb.copy_to_slice(count.get_unchecked_mut(ob..ob + 32));
            xc.copy_to_slice(count.get_unchecked_mut(oc..oc + 32));

            off += 32;
        }
        let extra = (off + ll + 1 + 32) - input.len();
        if extra != 32 {
            let mask =
                mask8x32::from_int_unchecked(i8x32::from_slice(EXTRA_MASKS.get_unchecked(extra)));
            off = input.len() - (ll + 1 + 32);

            let oa = off;
            let ob = off + 1;
            let oc = off + ll;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));
            let c = u8x32::from_slice(input.get_unchecked(oc..oc + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let a3 = a.simd_eq(cs3);
            let a4 = a.simd_eq(cs4);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);
            let b3 = b.simd_eq(cs3);
            let b4 = b.simd_eq(cs4);
            let c1 = c.simd_eq(cs1);
            let c2 = c.simd_eq(cs2);
            let c3 = c.simd_eq(cs3);
            let c4 = c.simd_eq(cs4);

            let mut xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let mut xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));
            let mut xc = u8x32::from_slice(count.get_unchecked(oc..oc + 32));

            xa += (mask & a2).to_int().cast()
                & ((b1.to_int().cast() & xb) + (c1.to_int().cast() & xc));
            xa += (mask & a4).to_int().cast()
                & ((b3.to_int().cast() & xb) + (c3.to_int().cast() & xc));

            xb = simd_swizzle!(xa, xb, SWIZZLE_MAP);

            xb += xa & (a1 & mask).to_int().cast() & b2.to_int().cast();
            xc += xa & (a1 & mask).to_int().cast() & c2.to_int().cast();

            xb += xa & (a3 & mask).to_int().cast() & b4.to_int().cast();
            xc += xa & (a3 & mask).to_int().cast() & c4.to_int().cast();

            *count.as_mut_ptr().cast::<u8>().add(oa) = xa[0];
            xb.copy_to_slice(count.get_unchecked_mut(ob..ob + 32));
            xc.copy_to_slice(count.get_unchecked_mut(oc..oc + 32));
        }

        off = input.len() - ll;
        while off + 1 + 32 <= input.len() {
            let oa = off;
            let ob = off + 1;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let a3 = a.simd_eq(cs3);
            let a4 = a.simd_eq(cs4);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);
            let b3 = b.simd_eq(cs3);
            let b4 = b.simd_eq(cs4);

            let mut xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let mut xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));

            xa += (a2 & b1).to_int().cast() & xb;
            xa += (a4 & b3).to_int().cast() & xb;

            xb = simd_swizzle!(xa, xb, SWIZZLE_MAP);

            xb += (b2 & a1).to_int().cast() & xa;
            xb += (b4 & a3).to_int().cast() & xa;

            *count.as_mut_ptr().cast::<u8>().add(oa) = xa[0];
            xb.copy_to_slice(count.get_unchecked_mut(ob..ob + 32));

            off += 32;
        }
        let extra = (off + 1 + 32) - input.len();
        if extra != 32 {
            debug_assert!(extra < 32);
            let mask =
                mask8x32::from_int_unchecked(i8x32::from_slice(EXTRA_MASKS.get_unchecked(extra)));
            off = input.len() - 32 - 1;

            let oa = off;
            let ob = off + 1;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let a3 = a.simd_eq(cs3);
            let a4 = a.simd_eq(cs4);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);
            let b3 = b.simd_eq(cs3);
            let b4 = b.simd_eq(cs4);

            let mut xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let mut xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));

            xa += (a2 & b1 & mask).to_int().cast() & xb;
            xa += (a4 & b3 & mask).to_int().cast() & xb;

            xb = simd_swizzle!(xa, xb, SWIZZLE_MAP);

            xb += (b2 & a1 & mask).to_int().cast() & xa;
            xb += (b4 & a3 & mask).to_int().cast() & xa;

            *count.as_mut_ptr().cast::<u8>().add(oa) = xa[0];
            xb.copy_to_slice(count.get_unchecked_mut(ob..ob + 32));
        }

        #[cfg(debug_assertions)]
        {
            println!(
"{}-{} + {}-{} v2"",
                c1 as char, c2 as char, c3 as char, c4 as char
            );
            for i in 0..input.len() {
                if input[i] == c2 {
                    let j = i.wrapping_sub(1);
                    if j < input.len() && input[j] == c1 {
                        count2[i] += count2[j];
                    }
                    let j = i.wrapping_add(1);
                    if j < input.len() && input[j] == c1 {
                        count2[i] += count2[j];
                    }
                    let j = i.wrapping_sub(ll);
                    if j < input.len() && input[j] == c1 {
                        count2[i] += count2[j];
                    }
                    let j = i.wrapping_add(ll);
                    if j < input.len() && input[j] == c1 {
                        count2[i] += count2[j];
                    }
                } else if input[i] == c4 {
                    let j = i.wrapping_sub(1);
                    if j < input.len() && input[j] == c3 {
                        count2[i] += count2[j];
                    }
                    let j = i.wrapping_add(1);
                    if j < input.len() && input[j] == c3 {
                        count2[i] += count2[j];
                    }
                    let j = i.wrapping_sub(ll);
                    if j < input.len() && input[j] == c3 {
                        count2[i] += count2[j];
                    }
                    let j = i.wrapping_add(ll);
                    if j < input.len() && input[j] == c3 {
                        count2[i] += count2[j];
                    }
                }
            }

"v1"");
            for i in 0..ll - 1 {
                for j in 0..ll - 1 {
"{} "", count.assume_init_ref()[ll * i + j]);
                }
                println!();
            }
"v2"");
            for i in 0..ll - 1 {
                for j in 0..ll - 1 {
"{} "", count2[ll * i + j]);
                }
                println!();
            }
            for i in 0..input.len() {
                assert_eq!(
                    count.assume_init_ref()[i],
                    count2[i],
"{} {} {}"",
                    i,
                    i / ll,
                    i % ll
                );
            }
        }
    }

    #[cfg(debug_assertions)]
    let mut tot_exp = 0;
    #[cfg(debug_assertions)]
    {
        for i in 0..input.len() {
            if input[i] == b'4' {
                let j = i.wrapping_sub(1);
                if j < input.len() && input[j] == b'5' {
                    tot_exp += (count2[i] * count2[j]) as u16;
                }
                let j = i.wrapping_add(1);
                if j < input.len() && input[j] == b'5' {
                    tot_exp += (count2[i] * count2[j]) as u16;
                }
                let j = i.wrapping_sub(ll);
                if j < input.len() && input[j] == b'5' {
                    tot_exp += (count2[i] * count2[j]) as u16;
                }
                let j = i.wrapping_add(ll);
                if j < input.len() && input[j] == b'5' {
                    tot_exp += (count2[i] * count2[j]) as u16;
                }
            }
        }
    }

    {
        let c1 = b'4';
        let c2 = c1 + 1;

        let cs1 = u8x32::splat(c1);
        let cs2 = u8x32::splat(c2);
        let mut off = 0;

        let mut tot = u8x32::splat(0);

        while off + ll + 1 + 32 <= input.len() {
            let oa = off;
            let ob = off + 1;
            let oc = off + ll;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));
            let c = u8x32::from_slice(input.get_unchecked(oc..oc + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);
            let c1 = c.simd_eq(cs1);
            let c2 = c.simd_eq(cs2);

            let xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));
            let xc = u8x32::from_slice(count.get_unchecked(oc..oc + 32));

            tot += (xa * xb) & b1.to_int().cast() & a2.to_int().cast();
            tot += (xa * xc) & c1.to_int().cast() & a2.to_int().cast();
            tot += (xa * xb) & a1.to_int().cast() & b2.to_int().cast();
            tot += (xa * xc) & a1.to_int().cast() & c2.to_int().cast();

            off += 32;
        }
        let extra = (off + ll + 1 + 32) - input.len();
        if extra != 32 {
            debug_assert!(extra < 32);
            let mask =
                mask8x32::from_int_unchecked(i8x32::from_slice(EXTRA_MASKS.get_unchecked(extra)));
            off = input.len() - (ll + 1 + 32);

            let oa = off;
            let ob = off + 1;
            let oc = off + ll;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));
            let c = u8x32::from_slice(input.get_unchecked(oc..oc + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);
            let c1 = c.simd_eq(cs1);
            let c2 = c.simd_eq(cs2);

            let xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));
            let xc = u8x32::from_slice(count.get_unchecked(oc..oc + 32));

            tot += (xa * xb) & mask.to_int().cast() & b1.to_int().cast() & a2.to_int().cast();
            tot += (xa * xc) & mask.to_int().cast() & c1.to_int().cast() & a2.to_int().cast();
            tot += (xa * xb) & mask.to_int().cast() & a1.to_int().cast() & b2.to_int().cast();
            tot += (xa * xc) & mask.to_int().cast() & a1.to_int().cast() & c2.to_int().cast();
        }

        off = input.len() - ll;
        while off + 1 + 32 <= input.len() {
            let oa = off;
            let ob = off + 1;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);

            let xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));

            tot += (a2 & b1).to_int().cast() & (xa * xb);
            tot += (b2 & a1).to_int().cast() & (xa * xb);

            off += 32;
        }
        let extra = (off + 1 + 32) - input.len();
        if extra != 32 {
            debug_assert!(extra < 32);
            let mask =
                mask8x32::from_int_unchecked(i8x32::from_slice(EXTRA_MASKS.get_unchecked(extra)));
            off = input.len() - 32 - 1;

            let oa = off;
            let ob = off + 1;

            let a = u8x32::from_slice(input.get_unchecked(oa..oa + 32));
            let b = u8x32::from_slice(input.get_unchecked(ob..ob + 32));

            let a1 = a.simd_eq(cs1);
            let a2 = a.simd_eq(cs2);
            let b1 = b.simd_eq(cs1);
            let b2 = b.simd_eq(cs2);

            let xa = u8x32::from_slice(count.get_unchecked(oa..oa + 32));
            let xb = u8x32::from_slice(count.get_unchecked(ob..ob + 32));

            tot += (a2 & b1 & mask).to_int().cast() & (xa * xb);
            tot += (b2 & a1 & mask).to_int().cast() & (xa * xb);
        }

        #[cfg(debug_assertions)]
        debug_assert_eq!(tot_exp, tot.cast::<u16>().reduce_sum());

        tot.cast::<u16>().reduce_sum() as u64
    }
}

use std::ops::Range;
trait MUHelper<T> {
    unsafe fn get_unchecked(&self, r: Range<usize>) -> &[T];
    unsafe fn get_unchecked_mut(&mut self, r: Range<usize>) -> &mut [T];
}
impl<T, const N: usize> MUHelper<T> for MaybeUninit<[T; N]> {
    unsafe fn get_unchecked(&self, r: Range<usize>) -> &[T] {
        &*self.as_ptr().as_slice().get_unchecked(r)
    }
    unsafe fn get_unchecked_mut(&mut self, r: Range<usize>) -> &mut [T] {
        &mut *self.as_mut_ptr().as_mut_slice().get_unchecked_mut(r)
    }
}
