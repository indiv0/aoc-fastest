// Original by: alion02
#![allow(clippy::pointers_in_nomem_asm_block)]
#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
    clippy::erasing_op,
    static_mut_refs,
    internal_features,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

use std::{
    arch::{
        asm,
        x86_64::{
            __m256i, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_movemask_epi8,
            _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16,
            _mm_movemask_epi8, _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pext_u32,
        },
    },
    fmt::Display,
    mem::{offset_of, transmute, MaybeUninit},
    simd::prelude::*,
};

// perhaps for later use
macro_rules! black_box {
    ($thing:expr) => {{
        let mut thing = $thing;
        asm!(
            "/*{t}*/",
            t = inout(reg) thing,
            options(pure, nomem, preserves_flags, nostack)
        );
        thing
    }};
}

unsafe fn process<const P2: bool>(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();
    let mut ptr = r.start;
    let mut cy = 0usize;

    #[repr(C, align(32))]
    struct Tables {
        _padding1: [u8; 16],
        antinodes: [u64; 150],
        _padding2: [u8; 16],
        frequencies: [[[u8; 2]; 4]; 75],
    }

    static mut TABLES: Tables = Tables {
        _padding1: [0; 16],
        antinodes: [0; 150],
        _padding2: [0; 16],
        frequencies: [[[0; 2]; 4]; 75],
    };

    let Tables {
        antinodes,
        frequencies,
        ..
    } = &mut TABLES;

    antinodes[50..100].fill(0);
    frequencies.fill(Default::default());

    loop {
        let c1 = ptr.cast::<u8x32>().read_unaligned() + Simd::splat(127 - b'.');
        let c2 = ptr.add(18).cast::<u8x32>().read_unaligned() + Simd::splat(127 - b'.');
        let m1 = c1.simd_ge(Simd::splat(128)).to_bitmask();
        let m2 = c2.simd_ge(Simd::splat(128)).to_bitmask();
        let mut mask = m1 | m2 << 18;
        if P2 {
            *antinodes.get_unchecked_mut(50 + cy) |= mask;
        }
        while mask != 0 {
            let cx = mask.trailing_zeros() as usize;
            let bucket = frequencies
                .get_unchecked_mut((ptr.add(cx).read() as usize).unchecked_sub(b'0' as usize));
            let count_bucket = bucket.get_unchecked_mut(3).get_unchecked_mut(0);
            let count = *count_bucket as usize;
            *count_bucket += 1;
            let [nx, ny] = bucket.get_unchecked_mut(count);
            *nx = cx as u8;
            *ny = cy as u8;
            for i in 0..count {
                let [sx, sy] = *bucket.get_unchecked(i);
                let sx = sx as usize;
                let sy = sy as usize;
                let dx = cx as isize - sx as isize;
                let dy = cy - sy;
                let sbit = 1 << sx;
                let cbit = 1 << cx;
                if dx > 0 {
                    let dx = dx as usize;
                    if P2 {
                        let mut bit = cbit << dx;
                        let mut idx = cy + dy;
                        while bit < 1 << 50 && idx < 50 {
                            *antinodes.get_unchecked_mut(50 + idx) |= bit;
                            bit <<= dx;
                            idx += dy;
                        }
                        let mut bit = sbit >> dx;
                        let mut idx = sy as isize - dy as isize;
                        while bit > 0 && idx >= 0 {
                            *antinodes.get_unchecked_mut(50 + idx as usize) |= bit;
                            bit >>= dx;
                            idx -= dy as isize;
                        }
                    } else {
                        *antinodes.get_unchecked_mut(50 + cy + dy) |= cbit << dx;
                        *antinodes.get_unchecked_mut(50 + sy - dy) |= sbit >> dx;
                    }
                } else {
                    let dx = -dx as usize;
                    if P2 {
                        let mut bit = cbit >> dx;
                        let mut idx = cy + dy;
                        while bit > 0 && idx < 50 {
                            *antinodes.get_unchecked_mut(50 + idx) |= bit;
                            bit >>= dx;
                            idx += dy;
                        }
                        let mut bit = sbit << dx;
                        let mut idx = sy as isize - dy as isize;
                        while bit < 1 << 50 && idx >= 0 {
                            *antinodes.get_unchecked_mut(50 + idx as usize) |= bit;
                            bit <<= dx;
                            idx -= dy as isize;
                        }
                    } else {
                        *antinodes.get_unchecked_mut(50 + cy + dy) |= cbit >> dx;
                        *antinodes.get_unchecked_mut(50 + sy - dy) |= sbit << dx;
                    }
                }
            }

            mask &= mask - 1;
        }

        ptr = ptr.add(51);
        cy += 1;
        if ptr == r.end {
            break;
        }
    }

    antinodes
        .get_unchecked(50..100)
        .iter()
        .map(|&row| if P2 { row } else { row & 0x3FFFFFFFFFFFF }.count_ones())
        .sum()
}

unsafe fn inner1(s: &[u8]) -> u32 {
    let r = s.as_ptr_range();

    #[repr(C, align(32))]
    struct Tables {
        _padding1: [u8; 16],
        antinodes: [u64; 150],
        _padding2: [u8; 16],
        frequencies: [[[u8; 2]; 4]; 75],
    }

    static mut TABLES: Tables = Tables {
        _padding1: [0; 16],
        antinodes: [0; 150],
        _padding2: [0; 16],
        frequencies: [[[0; 2]; 4]; 75],
    };

    let tables = &mut TABLES;

    tables.antinodes[50..100].fill(0);
    tables.frequencies.fill([[255; 2]; 4]);

    asm!(
    "21:",
        "vpaddb {y1}, {offset}, ymmword ptr[{ptr}]",
        "vpaddb {y2}, {offset}, ymmword ptr[{ptr} + 18]",
        "vpmovmskb {r1:e}, {y1}",
        "vpmovmskb {r2:e}, {y2}",
        "shl {r2}, 18",
        "or {r1}, {r2}",
        "jz 20f",
    "23:",
        "tzcnt {cx}, {r1}",
        "movzx {r2:e}, byte ptr[{ptr} + {cx}]",
        "lea {r2}, [{table} + {r2} * 8 + 432]",
        "movsx {count:e}, byte ptr[{r2} + 7]",
        "inc {count:e}",
        "mov byte ptr[{r2} + 7], {count:l}",
        "mov byte ptr[{r2} + {count} * 2], {cx:l}",
        "mov byte ptr[{r2} + {count} * 2 + 1], {cy:l}",
        "jz 22f",
        "shlx {cbit}, {one:r}, {cx}",
    "26:",
        "movzx {sx:e}, byte ptr[{r2} + {count} * 2 - 2]",
        "movzx {sy:e}, byte ptr[{r2} + {count} * 2 - 1]",
        "shlx {sbit}, {one:r}, {sx}",
        "mov {dy:e}, {cy:e}",
        "sub {dy}, {sy}",
        "mov {dx:e}, {cx:e}",
        "sub {sy:e}, {dy:e}",
        "sub {dx}, {sx}",
        "lea {sx}, [{cy} + {dy}]",
        "jbe 24f",
        "shlx {dy}, {cbit}, {dx}",
        "shrx {sbit}, {sbit}, {dx}",
        "jmp 25f",
    "24:",
        "neg {dx}",
        "shrx {dy}, {cbit}, {dx}",
        "shlx {sbit}, {sbit}, {dx}",
    "25:",
        "or qword ptr[{table} + {sx} * 8], {dy}",
        "or qword ptr[{table} + {sy} * 8], {sbit}",
        "dec {count:e}",
        "jnz 26b",
    "22:",
        "blsr {r1}, {r1}",
        "jnz 23b",
    "20:",
        "add {ptr}, -51",
        "dec {cy:e}",
        "jns 21b",
        y1 = out(ymm_reg) _,
        y2 = out(ymm_reg) _,
        offset = in(ymm_reg) u8x32::splat(127 - b'.'),
        ptr = inout(reg) r.end.sub(51) => _,
        r1 = out(reg) _,
        r2 = out(reg) _,
        count = out(reg) _,
        cx = out(reg) _,
        cy = inout(reg) 49usize => _,
        sx = out(reg) _,
        sy = out(reg) _,
        dx = out(reg) _,
        dy = out(reg) _,
        cbit = out(reg) _,
        sbit = out(reg) _,
        table = in(reg) (tables as *mut Tables).byte_add(offset_of!(Tables, antinodes) + size_of::<u64>() * 50),
        one = in(reg) 1,
        options(nostack),
    );

    tables
        .antinodes
        .get_unchecked(50..100)
        .iter()
        .map(|&row| (row & 0x3FFFFFFFFFFFF).count_ones())
        .sum()
}

pub fn run(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

unsafe fn inner2(s: &[u8]) -> u32 {
    process::<true>(s)
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s.as_bytes()) }
}
