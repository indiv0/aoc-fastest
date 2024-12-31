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
            __m128i, __m256i, _bextr2_u32, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_movemask_epi8,
            _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16, _mm_minpos_epu16,
            _mm_movemask_epi8, _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pext_u32,
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

macro_rules! row_len {
    () => {
        142
    };
}

macro_rules! side_len {
    () => {
        row_len!() - 1
    };
}

macro_rules! far_edge {
    () => {
        row_len!() - 3
    };
}

#[inline]
unsafe fn inner1(s: &[u8]) -> u32 {
    static mut CURR: [Node; 65536] = [unsafe { transmute(0) }; 65536];
    static mut NEXT: [Node; 65536] = [unsafe { transmute(0) }; 65536];
    static OFFSET: [i16; 4] = [1, row_len!(), -1, -row_len!()];

    let mut visited = [0u8; row_len!() * side_len!()];
    let mut curr = &mut CURR;
    let mut next = &mut NEXT;
    let offset = &OFFSET;

    #[derive(Clone, Copy)]
    #[repr(align(4))]
    struct Node {
        pos: u16,
        dir: u8,
        cost: u8,
    }

    curr[0] = Node {
        pos: far_edge!() * row_len!() + 1,
        dir: 0,
        cost: 0,
    };
    curr[1].cost = !0;

    let mut turn_cost = 0;
    loop {
        let mut i = 0;
        let mut k = 0;
        loop {
            let mut j = i;
            let cost = curr.get_unchecked_mut(j).cost;
            let next_cost = cost + 1;
            loop {
                let node = curr.get_unchecked_mut(j);
                let mut pos = node.pos;
                assert!(*s.get_unchecked(pos as usize) != b'#');
                if pos == row_len!() + far_edge!() {
                    return turn_cost + cost as u32 * 2;
                }
                let mut dir = node.dir;
                let visit_mask = dir & 1;
                'delete: {
                    if *visited.get_unchecked(pos as usize) & visit_mask == 0 {
                        *visited.get_unchecked_mut(pos as usize) |= visit_mask;
                        dir ^= 1;
                        {
                            let pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            if *s.get_unchecked(pos as usize) != b'#' {
                                *next.get_unchecked_mut(k) = Node {
                                    pos: pos.wrapping_add_signed(*offset.get_unchecked(dir as usize)),
                                    dir,
                                    cost: next_cost,
                                };
                                k += 1;
                            }
                        }
                        dir ^= 2;
                        {
                            let pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            if *s.get_unchecked(pos as usize) != b'#' {
                                *next.get_unchecked_mut(k) = Node {
                                    pos: pos.wrapping_add_signed(*offset.get_unchecked(dir as usize)),
                                    dir,
                                    cost: next_cost,
                                };
                                k += 1;
                            }
                        }
                        dir ^= 3;
                        pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                        if *s.get_unchecked(pos as usize) != b'#' {
                            node.pos = pos.wrapping_add_signed(*offset.get_unchecked(dir as usize));
                            node.cost = next_cost;
                            break 'delete;
                        }
                    }
                    *curr.get_unchecked_mut(j) = *curr.get_unchecked(i);
                    i += 1;
                }

                j += 1;
                if curr.get_unchecked(j).cost > cost {
                    break;
                }
            }

            if curr.get_unchecked(i).cost == !0 {
                break;
            }
        }

        turn_cost += 1000;
        (curr, next) = (next, curr);
        curr.get_unchecked_mut(k).cost = !0;
    }

    // let mut curr = Vec::<Node>::from_iter([Node {
    //     pos: far_edge!() * row_len!() + 1,
    //     dir: 0,
    //     cost: 0,
    // }]);
    // let mut next = Vec::<Node>::from_iter([Node {
    //     pos: !0,
    //     dir: !0,
    //     cost: !0,
    // }]);

    // let mut i = 0;
    // let mut turn_cost = 0;

    // let mut visited = [0u8; row_len!() * side_len!()];

    // loop {
    //     // let mut map = s.to_vec();
    //     // for node in &front {
    //     //     map[node.pos as usize] = b">v<^"[node.dir as usize];
    //     // }
    //     // for node in &curr {
    //     //     map[node.pos as usize] = b'*';
    //     // }
    //     // for node in &next {
    //     //     map[node.pos as usize] = b'+';
    //     // }
    //     // for y in 0..side_len!() {
    //     //     println!(
    //     //         "{}",
    //     //         std::str::from_utf8(&map[y * row_len!()..y * row_len!() + side_len!()]).unwrap()
    //     //     );
    //     // }

    //     let mut node = curr.get_unchecked_mut(i);
    //     let curr_cost = node.cost;

    //     let mut j = i;
    //     loop {
    //         let node = curr.get_unchecked_mut(j);

    //         if *visited.get_unchecked(node.pos as usize) & 1 << node.dir != 0 {
    //             *curr.get_unchecked_mut(j) = *curr.get_unchecked(i);
    //             i += 1;
    //         } else {
    //             if node.pos == row_len!() + far_edge!() {
    //                 return turn_cost + node.cost as u32 * 2 + 2;
    //             }

    //             macro_rules! offset {
    //                 ($dir:expr) => {
    //                     *[1i16, row_len!(), -1, -row_len!()].get_unchecked($dir as usize)
    //                 };
    //             }
    //         }

    //         if j == curr.len() {
    //             break;
    //         }
    //     }

    //     if i == curr.len() {
    //         (curr, next) = (next, curr);
    //         i = 0;
    //         turn_cost += 1000;
    //     }

    //     // front.retain_mut(
    //     //     |&mut Node {
    //     //          pos: ref mut pos_ref,
    //     //          dir,
    //     //          cost: ref mut cost_ref,
    //     //      }| {
    //     //         if found_end {
    //     //             return false;
    //     //         }

    //     //         let pos = *pos_ref;
    //     //         let cost = *cost_ref + 1;

    //     //         if pos == row_len!() + far_edge!() {
    //     //             found_end = true;
    //     //             return true;
    //     //         }

    //     //         macro_rules! offset {
    //     //             ($dir:expr) => {
    //     //                 *[1i16, row_len!(), -1, -row_len!()].get_unchecked($dir as usize)
    //     //             };
    //     //         }

    //     //         let off = offset!(dir);
    //     //         let mut npos = pos.wrapping_add_signed(off);
    //     //         let retain = if *s.get_unchecked(npos as usize) != b'#' && {
    //     //             npos = npos.wrapping_add_signed(off);
    //     //             *visited.get_unchecked(npos as usize) & 5 << (dir & 1) == 0
    //     //         } {
    //     //             *visited.get_unchecked_mut(npos as usize) |= 1 << dir;
    //     //             *cost_ref = cost;
    //     //             *pos_ref = npos;
    //     //             true
    //     //         } else {
    //     //             false
    //     //         };

    //     //         let dir = dir ^ 1;
    //     //         let off = offset!(dir);
    //     //         let mut npos = pos.wrapping_add_signed(off);
    //     //         if *s.get_unchecked(npos as usize) != b'#' && {
    //     //             npos = npos.wrapping_add_signed(off);
    //     //             *visited.get_unchecked(npos as usize) & 5 << (dir & 1) == 0
    //     //         } {
    //     //             *visited.get_unchecked_mut(npos as usize) |= 1 << dir;
    //     //             next.push_back(Node { pos: npos, dir, cost });
    //     //         }

    //     //         let dir = dir ^ 2;
    //     //         let off = offset!(dir);
    //     //         let mut npos = pos.wrapping_add_signed(off);
    //     //         if *s.get_unchecked(npos as usize) != b'#' && {
    //     //             npos = npos.wrapping_add_signed(off);
    //     //             *visited.get_unchecked(npos as usize) & 5 << (dir & 1) == 0
    //     //         } {
    //     //             *visited.get_unchecked_mut(npos as usize) |= 1 << dir;
    //     //             next.push_back(Node { pos: npos, dir, cost });
    //     //         }

    //     //         retain
    //     //     },
    //     // );

    //     // if found_end {
    //     //     return turn_cost + front.back().unwrap_unchecked().cost as u32 * 2;
    //     // }
    // }
}

#[inline]
unsafe fn inner2(s: &[u8]) -> u32 {
    0
}

#[inline]
pub fn run(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s.as_bytes()) }
}
