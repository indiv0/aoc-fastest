// Original by: alion02
#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
    clippy::precedence,
    clippy::missing_transmute_annotations,
    clippy::pointers_in_nomem_asm_block,
    clippy::erasing_op,
    static_mut_refs,
    internal_features,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

#[allow(unused)]
use std::{
    arch::{
        asm,
        x86_64::{
            __m128i, __m256i, _bextr2_u32, _mm256_madd_epi16, _mm256_maddubs_epi16,
            _mm256_movemask_epi8, _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16,
            _mm_maddubs_epi16, _mm_minpos_epu16, _mm_movemask_epi8, _mm_packus_epi32,
            _mm_shuffle_epi8, _mm_testc_si128, _pext_u32,
        },
    },
    array,
    fmt::Display,
    hint::assert_unchecked,
    intrinsics::{likely, unlikely},
    mem::{offset_of, transmute, MaybeUninit},
    ptr,
    simd::prelude::*,
    slice,
};

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    static DIR_TABLE: [i16; 256] = {
        let mut dir_table = [0; 256];
        dir_table[b'>' as usize] = 1;
        dir_table[b'v' as usize] = 51;
        dir_table[b'<' as usize] = -1;
        dir_table[b'<' as usize + 1] = -1;
        dir_table[b'^' as usize] = -51;
        dir_table[b'^' as usize + 1] = -1;
        dir_table
    };
    static mut MAP: [u8; 2560] = [0; 2560];
    let map = &mut MAP;
    map.copy_from_slice(s.get_unchecked(..2560));
    let pos = 24usize * 51 + 24;
    *map.get_unchecked_mut(pos) = b'.';

    asm!(
        "jmp 24f",
    // #
    "21:",
        "sub {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
    // .
    "20:",
        "inc {ip}",
        "je 99f",
    "24:",
        "movzx {inst:e}, byte ptr[{instrs} + {ip}]",
        "add {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
        "cmp byte ptr[{map} + {pos}], 46",
        "je 20b",
        "jb 21b",
    // O
        "mov {block_pos:e}, {pos:e}",
    "22:",
    // O repeats
        "add {block_pos:e}, dword ptr[{dir_table} + {inst} * 2]",
        "cmp byte ptr[{map} + {block_pos}], 46",
        "ja 22b",
        "jb 21b",
    // O then .
    "23:",
        "mov byte ptr[{map} + {pos}], 46",
        "mov byte ptr[{map} + {block_pos}], 79",
        "inc {ip}",
        "jne 24b",
    "99:",
        instrs = in(reg) s.as_ptr_range().end,
        ip = inout(reg) -20020isize => _,
        map = in(reg) map,
        pos = inout(reg) pos => _,
        inst = out(reg) _,
        block_pos = out(reg) _,
        dir_table = inout(reg) &DIR_TABLE => _,
        options(nostack),
    );

    let mut map = map.as_ptr().add(52).cast::<u8x16>();
    let mut vec_counts = u32x4::splat(0);
    let mut y_mult = i16x8::splat(-100);
    for _y in 1..49 {
        macro_rules! process {
            ($i:expr) => {{
                let c = map.byte_add($i).read_unaligned();
                let c = c.simd_eq(Simd::splat(b'O'));
                let x = c.select(
                    u8x16::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
                        + Simd::splat($i),
                    Simd::splat(0),
                );
                (c.to_int(), x)
            }};
        }
        let (c1, x1) = process!(0);
        let (c2, x2) = process!(16);
        let (c3, x3) = process!(32);
        let c = c1 + c2 + c3;
        let c = _mm_maddubs_epi16(i8x16::splat(1).into(), c.into());
        let c = _mm_madd_epi16(c, y_mult.into());
        vec_counts += u32x4::from(c);
        let x = x1 + x2 + x3;
        let x = _mm_maddubs_epi16(x.into(), i8x16::splat(1).into());
        let x = _mm_madd_epi16(x, i16x8::splat(1).into());
        vec_counts += u32x4::from(x);
        y_mult -= i16x8::splat(100);
        map = map.byte_add(51);
    }

    vec_counts.reduce_sum()
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u32 {
    static DIR_TABLE: [i16; 256] = {
        let mut dir_table = [0; 256];
        dir_table[b'>' as usize] = 1;
        dir_table[b'v' as usize] = 128;
        dir_table[b'<' as usize] = -1;
        dir_table[b'<' as usize + 1] = -1;
        dir_table[b'^' as usize] = -128;
        dir_table[b'^' as usize + 1] = -1;
        dir_table
    };
    static mut MAP: [i8; 6400] = [-2; 6400];
    let map = &mut MAP;

    for y in 1..49 {
        for x in 0..3 {
            let chunk = s
                .as_ptr()
                .add(y * 51 + x * 16 + 1)
                .cast::<u8x16>()
                .read_unaligned();
            let chunk = simd_swizzle!(
                chunk,
                [
                    0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12,
                    12, 13, 13, 14, 14, 15, 15
                ]
            );
            let a = chunk
                .simd_eq(Simd::splat(b'#'))
                .select(i8x32::splat(-2), i8x32::splat(-1));
            let b = chunk.simd_eq(Simd::splat(b'O')).select(
                Simd::from_array([
                    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
                    0, 1, 0, 1, 0, 1,
                ]),
                a,
            );

            map.as_mut_ptr()
                .add(y * 128 + x * 32 + 2)
                .cast::<i8x32>()
                .write_unaligned(b);
        }
    }

    let pos = 24usize * 128 + 48;

    asm!(
        "jmp 24f",
    "21:",
        "sub {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
    "20:",
        "inc {ip}",
        "je 99f",
    "24:",
        "movzx {inst:e}, byte ptr[{instrs} + {ip}]",
        "add {pos:e}, dword ptr[{dir_table} + {inst} * 2]",
        "cmp byte ptr[{map} + {pos}], -1",
        "je 20b", // .
        "jl 21b", // #
        // []
        "mov {bpos:e}, {pos:e}",
        "mov {step:e}, dword ptr[{dir_table} + {inst} * 2]",
        "cmp {step:l}, -128",
        "je 25f", // vertical
        // horizontal
        "add {step:e}, {step:e}",
    "26:",
        "add {bpos:e}, {step:e}",
        "cmp byte ptr[{map} + {bpos}], -1",
        "jg 26b", // [] repeats
        "jl 21b", // [] then # // TODO optimize
        // [] then .
        "cmp byte ptr[{map} + {pos}], 0",
        "je 27f", // right
        // left
    "28:",
        "mov word ptr[{map} + {bpos}], 256",
        "sub {bpos:e}, {step:e}",
        "cmp {bpos:e}, {pos:e}",
        "jne 28b",
        "mov byte ptr[{map} + {bpos}], -1",
        "inc {ip}",
        "jne 24b",
        "jmp 99f",
    "27:",
        "mov word ptr[{map} + {bpos} - 1], 256",
        "sub {bpos:e}, {step:e}",
        "cmp {bpos:e}, {pos:e}",
        "jne 27b",
        "mov byte ptr[{map} + {bpos}], -1",
        "inc {ip}",
        "jne 24b",
        "jmp 99f",
    "31:",
        "sub {pos:e}, {step:e}",
        "mov rsp, {saved_rsp}",
        "inc {ip}",
        "jne 24b",
        "jmp 99f",
    "33:",
        "inc {bpos:e}",
    "30:",
        "sub {bpos:l}, byte ptr[{map} + {bpos}]", // align block position to left
        "add {bpos:e}, {step:e}",
        "cmp byte ptr[{map} + {bpos}], -1",
        "jl 31b", // #
        "je 32f", // .
        "cmp byte ptr[{map} + {bpos}], 0",
        "je 30b",
        "push {bpos}",
        "call 30b",
        "pop {bpos}",
    "32:",
        "cmp byte ptr[{map} + {bpos} + 1], -1",
        "jl 31b", // #
        "jg 33b", // []
        // .
        "ret",
    "35:",
        "sub {bpos2:l}, byte ptr[{map} + {bpos2}]", // align block position to left
        "mov word ptr[{map} + {bpos2}], -1",
        "add {bpos2:e}, {step:e}",
        "cmp byte ptr[{map} + {bpos2}], 0",
        "push {bpos2}",
        "jl 36f", // done
        "call 35b",
        "mov {bpos2}, qword ptr[rsp]",
    "36:",
        "inc {bpos2:e}",
        "cmp byte ptr[{map} + {bpos2}], 0",
        "jl 37f", // done
        "call 35b",
    "37:",
        "pop {bpos2}",
        "mov word ptr[{map} + {bpos2}], 256",
        "ret",
    "25:",
        "mov {saved_rsp}, rsp",
        "mov {bpos2:e}, {bpos:e}",
        "call 30b", // check pushability
        "call 35b", // returned normally, so we can push
        "inc {ip}",
        "jne 24b",
    "99:",
        instrs = in(reg) s.as_ptr_range().end,
        ip = inout(reg) -20020isize => _,
        map = in(reg) map,
        pos = inout(reg) pos => _,
        inst = out(reg) _,
        bpos = out(reg) _,
        bpos2 = out(reg) _,
        step = out(reg) _,
        saved_rsp = out(reg) _,
        dir_table = inout(reg) &DIR_TABLE => _,
    );

    let mut map = map.as_ptr().add(130).cast::<i8x32>();
    let mut vec_counts = u32x8::splat(0);
    let mut y_mult = i16x16::splat(-100);
    for _y in 1..49 {
        macro_rules! process {
            ($i:expr) => {{
                let c = map.byte_add($i).read_unaligned();
                let c = c.simd_eq(Simd::splat(0));
                let x = c.select(
                    u8x32::from_array([
                        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                        23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                    ]) + Simd::splat($i),
                    Simd::splat(0),
                );
                (c.to_int(), x)
            }};
        }
        let (c1, x1) = process!(0);
        let (c2, x2) = process!(32);
        let (c3, x3) = process!(64);
        let c = c1 + c2 + c3;
        let c = _mm256_maddubs_epi16(i8x32::splat(1).into(), c.into());
        let c = _mm256_madd_epi16(c, y_mult.into());
        vec_counts += u32x8::from(c);
        let x = x1 + x2 + x3;
        let x = _mm256_maddubs_epi16(x.into(), i8x32::splat(1).into());
        let x = _mm256_madd_epi16(x, i16x16::splat(1).into());
        vec_counts += u32x8::from(x);
        y_mult -= i16x16::splat(100);
        map = map.byte_add(128);
    }

    vec_counts.reduce_sum()
}

#[inline]
pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn run(s: &str) -> impl Display {
    unsafe { inner2(s.as_bytes()) }
}
