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
            _mm_maddubs_epi16, _mm_minpos_epu16, _mm_movemask_epi8, _mm_packus_epi16,
            _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pdep_u32, _pext_u32, _pext_u64,
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
    static mut TRIE: [[u16; 6]; 4096] = [[0; 6]; 4096];

    let mut ptr = s.as_ptr();
    let trie = TRIE.as_mut_ptr();
    *trie = [0; 6];
    let mut len = 0;

    macro_rules! hash {
        ($byte:expr) => {
            _pext_u32($byte as u32, 83).wrapping_sub(10)
        };
    }

    macro_rules! is_lf {
        ($hash:expr) => {
            $hash == 2u32.wrapping_sub(10)
        };
    }

    loop {
        let mut hash = hash!(*ptr);
        let mut curr = 0;
        loop {
            ptr = ptr.add(1);
            let next = trie.byte_add(curr).cast::<u16>().add(hash as usize);
            if *next == 0 {
                len += 3;
                *next = len;
                *trie.byte_add(len as usize * 4) = [0; 6];
            }

            hash = hash!(*ptr);
            curr = *next as usize * 4;
            if (hash as i32) < 0 {
                break;
            }
        }

        ptr = ptr.add(2);
        assert!(*ptr > 64);
        *trie.byte_add(curr).cast::<u16>().add(2) = 1;
        if is_lf!(hash) {
            break;
        }
    }

    let mut total = 0;

    asm!(
        "mov {saved_rsp}, rsp",
        "jmp 20f",

    "203:",
        "inc {i:e}",
        "lea {node}, [{trie} + {tmp} * 4]",
    "200:",
        "cmp byte ptr[{node} + 4], 0",
        "je 201f", // try continuing
        "bts {seen}, {i}",
        "jc 201f", // memoized: can't finish pattern here
        "cmp byte ptr[{ptr} + {i}], {lf}",
        "je 202f", // success
        // try finishing this pattern
        "push {i}",
        "push {node}",
        "mov {node}, {trie}",
        "call 201f",
        "pop {node}",
        "pop {i}",
    "201:",
        "movzx {hash:e}, byte ptr[{ptr} + {i}]",
        "pext {hash:e}, {hash:e}, {hash_mask:e}",
        "sub {hash:e}, {hash_offset}",
        "js 204f", // in the middle of a pattern but towel is done
        "movzx {tmp:e}, word ptr[{node} + {hash} * 2]",
        "test {tmp:e}, {tmp:e}",
        "jne 203b", // continue
    "204:",
        "ret", // dead end

    "202:",
        "mov rsp, {saved_rsp}",
        "inc {total:e}",
        "lea {ptr}, [{ptr} + {i} + 1]",
        "cmp {ptr}, {end}",
        "je 22f",
    "20:",
        "mov {node}, {trie}",
        "xor {seen:e}, {seen:e}",
        "xor {i:e}, {i:e}",
        "call 201b",
    "21:",
        "inc {ptr}",
        "cmp byte ptr[{ptr}], {lf}",
        "jne 21b",
        "inc {ptr}",
        "cmp {ptr}, {end}",
        "jne 20b",
    "22:",

        saved_rsp = out(reg) _,
        seen = out(reg) _,
        i = out(reg) _,
        ptr = inout(reg) ptr => _,
        end = in(reg) s.as_ptr_range().end,
        hash = out(reg) _,
        node = out(reg) _,
        tmp = out(reg) _,
        trie = in(reg) trie,
        hash_mask = in(reg) 83,
        hash_offset = const 10,
        total = inout(reg) total,
        lf = const b'\n',
    );

    total
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u64 {
    0
}

#[inline]
pub fn run(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> u64 {
    unsafe { inner2(s.as_bytes()) }
}
