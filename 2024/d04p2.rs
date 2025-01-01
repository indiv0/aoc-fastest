// Original by: bendn
#![feature(portable_simd)]
// type bitset = bitvec::BitArr!(for 140, in u64);
// pub mod set {
//     uint::construct_uint! {pub struct bitset(3); }
// }
// use set::bitset;
use cmp::SimdPartialEq;
use std::simd::*;
const SIZE: usize = 140;

fn bits(input: &[u8], i: usize) -> u64 {
    let w = u8x64::from_slice(&input[64 * i..]);
    let x = u8x64::from_slice(&input[64 * i + 2..]);
    let a = u8x64::from_slice(&input[141 + 64 * i + 1..]);
    let y = u8x64::from_slice(&input[141 * 2 + 64 * i..]);
    let z = u8x64::from_slice(&input[141 * 2 + 64 * i + 2..]);

    let wz = ((w ^ z) + (x ^ y)).simd_eq(u8x64::splat((b'M' ^ b'S') + (b'M' ^ b'S')));
    // let xy = (x ^ y).simd_eq(u8x64::splat(b'M' ^ b'S'));
    let a = a.simd_eq(u8x64::splat(b'A'));
    (wz & a).to_bitmask()
}

pub fn run(input: &[u8]) -> u32 {
    assert_eq!(input.len(), 141 * 140);
    let mut sum = 0;
    for i in 0..304 {
        sum += bits(input, i).count_ones();
    }
    sum
}
