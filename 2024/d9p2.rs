// Original by: ameo
#![feature(array_chunks, array_windows, portable_simd, pointer_is_aligned_to)]

// TRICKS:
//
// - Span structure to represent regions of the memory
//   - Takes advantage of the fact that free space is always to the right of data in each slot.
//   - Uses that trick of adding free space to previous span in the case that the first element is
//     removed
//   - Uses the fact that data will always be removed from the first element of the vector to make
//     this possible
// - SoA for the spans
// - The usual assumptions from input
//   - hard-code input size
//   - hard-code the max number of elements that could appear in one span together
//     - For my input 4 was enough, but technically it would have to be 9 to cover all possible
//       inputs (9 1-size moves into a 9-free-space span)
//   - assume that we'll never move an element into the last [SIMD vector size] spans to avoid
//     having to do remainder checking in the inner SIMD loop
//     - probably also could have just added some padding elements to the free space array to work
//       around this
// - aligned input vector as well as data vectors for counts, free lists, and minivecs which
//   facilitates:
// - SIMD parsing
//   - de-interleaves sizes + free spaces into separate arrays and writes out with SIMD
//   - compiles into a beautiful 10x unrolled loop of basically pure SIMD
// - MiniVec; specialized capped-sized-vec that stores stuff inline.
//   - all methods custom-built and unsafe with no checks
//   - TRIED the fancy const impl that pre-computes the vector here with lens and IDs pre-set, but
//     memcpy overhead was greater than savings
//   - clever `pop_front` impl to avoid having to shift elements down
// - `start_span_ix_by_needed_size` to keep track of the earliest possible location of a big enough
//   free space for every size
//   - TRIED the fancy impl. that max's the val of all larger buckets as well, but turned out to be
//     way slower (especially when SIMD was enabled)
// - SIMD for finding the first free slot which is large enough
//   - as mentioned later, loading + checking 8 bytes at a time seems to be the fastest (even though
//     the checks themselves are done with SIMD instructions on a 128-bit register, the top half is
//     zeroed).
// - target-cpu=znver3
// - constant-time checksumming
// - `max_unmoved_src_id` accounting
//   - allows fully empty chunks at the end to be skipped during checksum computation
// - `finished_digit_count` bookkeeping
//   - allows for early exit of the main loop after we've found a stopping place for every char
//   - includes code that marks all bigger digits finished as well as the current digit since if
//     there's no space left to fit 5, no way you can fit 6+ either.
//     - This emits a `memset` call I can't get rid of, but since this path is only going to get hit
//       a few times it's cold enough to not matter + be worth it
// - using smaller int sizes for data which allows more items to be considered by SIMD as well as
//   reducing memory footprint and potentially reducing cache pressure
// - avoid storing counts inside the minivec at all.  instead, reference back to main counts vec
//   - this allows size of minivec to go from 32-16 bytes
//   - this necessitates a change in a previous opt. instead of setting count to 0 to pop_front, we
//     need allocate one extra slot in the counts vec and then set the id to that.
// - SIMD initialization for slot vectors.  Amazing stuff. (this put me back in the lead from
//   giooschi on the rust discord speed leaderboard)
//   - manual padding to force rust to keep all of the u16s 16bit aligned and keep the minivecs
//     32bit-aligned
// - made `start_span_ix_by_needed_size` `[u16; 10]` instead of `[usize; 10]`
// - switch `arr.get_unchecked_mut(x..).fill(u16::MAX)` to `for i in x.. { arr[i] = u16::MAX }`
//   which... caused a 20% improvement on the bot??
//   - (perhaps in combination with the previous optimization, but I don't think so)
// - micro-optim of the inner free space checking loop, shifting one pointer addition to happen
//   lazily only in the case that the first check fails to find any hits in the first SIMD scan.
//     - Essentially split the loop into a check start ptr -> (inc + check inc'd) loop instead of
//       just an (inc + check) loop
//     - Good 7% perf improvement over previous best on the bot.  Interestingly, no perf gain
//       locally.
// - removed every last bounds check with `get_unchecked/mut()`
//   - Actually made things slightly slower on the benchbot, probably because the code seemed to get
//     re-structured to be a bit less local, probably because the size of the code after the main
//     loop went down after the bounds checks were removed.
// - tuning the vector size for the inner loop
//   - turns out that u8x8 faster than u8x16 faster than u8x32
//   - u8x8 is pretty slightly - but significantly - faster than u8x16 on both local and benchmark
//     machine
//   - the same SIMD instruction, operating on 128-bit XMM register, is used for u8x8 case, just
//     with top bits zeroed
//   - the overhead of fetching just 64 extra bytes seems to outweigh the cost of having less chance
//     of finding the needle in the first vector
// - TRIED to initialize all spans to contain the empty elem ID and then just compute checksums on
//   all elements.
//   - turns out that most spans seem to have very few items (which makes sense tbh) so the work of
//     computing checksums for those empty slots greatly outweighed any benefit of avoiding the
//     dynamic checksum count etc.

use std::{
  fmt::Display,
  simd::{cmp::SimdPartialOrd, u16x16, u16x2, u8x32, u8x64, u8x8},
};

#[cfg(feature = "local")]
pub const INPUT: &'static [u8; 20_000] = include_bytes!("../inputs/day9.txt");

fn parse_digit(c: u8) -> u8 { c - 48 }

fn parse_input(input: &[u8]) -> Vec<(u32, u32)> {
  let mut it = input[..input.len() - if input.len() % 2 == 0 { 1 } else { 0 }].array_chunks::<2>();

  let mut out = Vec::with_capacity(20_002 / 2);
  while let Some(&[size, free]) = it.next() {
    out.push((parse_digit(size) as _, parse_digit(free) as _));
  }

  if let Some(remainder) = it.remainder().get(0) {
    out.push((parse_digit(*remainder) as _, 0));
  }

  out
}

#[repr(C, align(64))]
struct AlignToSixtyFour([u8; 64]);

/// This creates a vector with its data aligned to 64 bytes.  This allows us to use faster aligned
/// loads for SIMD operations.
///
/// adapted from: https://stackoverflow.com/a/60180226/3833068
fn aligned_vec<T>(n_bytes: usize) -> Vec<T> {
  assert_eq!(
    std::mem::size_of::<AlignToSixtyFour>() % std::mem::size_of::<T>(),
    0,
    "64 must be divisible by the size of `T` in bytes"
  );

  // Lazy math to ensure we always have enough.
  let n_units = (n_bytes / std::mem::size_of::<AlignToSixtyFour>()) + 1;

  let mut aligned: Vec<AlignToSixtyFour> = Vec::with_capacity(n_units);

  let ptr = aligned.as_mut_ptr();
  let len_units = aligned.len();
  let cap_units = aligned.capacity();

  std::mem::forget(aligned);

  unsafe {
    Vec::from_raw_parts(
      ptr as *mut T,
      (len_units * std::mem::size_of::<AlignToSixtyFour>()) / std::mem::size_of::<T>(),
      (cap_units * std::mem::size_of::<AlignToSixtyFour>()) / std::mem::size_of::<T>(),
    )
  }
}

// sadly, the cost of copying all of the uninitialized bytes that we don't care about is higher than
// being able to set the lengths and indices up front.
// const fn build_empty_slots() -> [MiniVec; 10_000] {
//   let mut arr: [MiniVec; 10_000] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//   let mut i = 0usize;
//   loop {
//     arr[i].len = 1;
//     arr[i].elements[0].id = i as u32;

//     i += 1;
//     if i == arr.len() {
//       break;
//     }
//   }
//   arr
// }

const MAX_ID: usize = 9_999;

fn parse_input_p2(input: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<MiniVec>) {
  let id_count = if input.len() % 2 == 1 {
    input.len() / 2 + 1
  } else {
    input.len() / 2
  };

  let mut orig_counts: Vec<u8> = aligned_vec(id_count + 1);
  unsafe { orig_counts.set_len(id_count + 1) };
  // this sets a special element at `orig_counts[MAX_ID + 1]` is used to facilitate efficient
  // `pop_front()` of the minivecs that happens when one of the files is moved down to a different
  // span.
  //
  // The ID of the removed slot is set to `MAX_ID + 1` which we hard-code to zero here.  This has a
  // result of causing the computed checksum for that slot to be zero while avoiding the need to do
  // complicated stuff like shift elements down, add state to track whether the first element has
  // been removed, etc.
  unsafe { *orig_counts.get_unchecked_mut(MAX_ID + 1) = 0 };
  let mut empty_spaces: Vec<u8> = aligned_vec(id_count);
  unsafe { empty_spaces.set_len(id_count) };
  let mut slots: Vec<MiniVec> = aligned_vec(id_count * std::mem::size_of::<MiniVec>());
  unsafe { slots.set_len(id_count) };
  // initialize the memory layout for the minivecs manually using SIMD.
  //
  // this sets them all up to have a length of one with a single element corresponding to file index
  // `i` as the first and only element.
  unsafe {
    let data: [u16; 2] = [1u16, 0u16];
    /// how many minivecs are set per SIMD store
    const CHUNK_SIZE: usize = 2;
    let mut data = u16x2::from_array(data);
    let to_add: [u16; 2] = [0u16, 1u16];
    debug_assert_eq!(slots.len() % CHUNK_SIZE, 0);
    let to_add = u16x2::from_array(to_add);

    let start = slots.as_mut_ptr();
    for i in 0..slots.len() {
      let ptr = start.add(i) as *mut u16x2;
      std::ptr::write(ptr, data);
      data += to_add;

      debug_assert_eq!(slots[i].len(), 1);
      debug_assert_eq!(slots[i].elements()[0], i as _);
    }
    // for chunk_ix in 0..(slots.len() / CHUNK_SIZE) {
    //   let out_ptr = start.add(chunk_ix);
    //   out_ptr.write(data);
    //   data += to_add;

    //   debug_assert_eq!(
    //     &slots[chunk_ix * CHUNK_SIZE..chunk_ix * CHUNK_SIZE + CHUNK_SIZE],
    //     &[
    //       MiniVec {
    //         len: 1,
    //         elements: [
    //           Slot {
    //             id: (chunk_ix * CHUNK_SIZE) as u16
    //           },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //         ],
    //         padding: 0,
    //       },
    //       MiniVec {
    //         len: 1,
    //         elements: [
    //           Slot {
    //             id: (chunk_ix * CHUNK_SIZE + 1) as u16
    //           },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //           Slot { id: 0 },
    //         ],
    //         padding: 0,
    //       }
    //     ]
    //   )
    // }

    // for i in 0..id_count {
    //   slots.get_unchecked_mut(i).len = 1;
    //   slots.get_unchecked_mut(i).elements[0].id = i as _;
    // }
  }

  debug_assert!(input.as_ptr().is_aligned_to(std::mem::align_of::<u8x64>()));

  const VECTOR_LEN: usize = 32;
  const STORE_VECTOR_LEN: usize = VECTOR_LEN / 2;
  let batch_count = input.len() / VECTOR_LEN;

  for batch_ix in 0..batch_count {
    let vec: u8x32 =
      unsafe { std::ptr::read(input.as_ptr().add(batch_ix * VECTOR_LEN) as *const _) };
    // convert from ascii digits to bytes representing the digit ('0' -> 0)
    let converted = vec - u8x32::splat(48);
    // split out from size,free,size,free to ([size,size], [free,free])
    let (sizes, frees) = converted.deinterleave(converted);
    // the de-interleave duplicates the results, so keeping only the first half is correct
    let sizes = sizes.resize::<STORE_VECTOR_LEN>(STORE_VECTOR_LEN as u8);
    let frees = frees.resize::<STORE_VECTOR_LEN>(STORE_VECTOR_LEN as u8);

    unsafe {
      let frees_ptr = empty_spaces.as_mut_ptr().add(batch_ix * STORE_VECTOR_LEN) as *mut _;
      *frees_ptr = frees;

      let orig_counts_ptr = orig_counts.as_mut_ptr().add(batch_ix * STORE_VECTOR_LEN) as *mut _;
      *orig_counts_ptr = sizes;
    }
  }

  /*
  if cfg!(feature = "local") && input.len() % 2 != 0 {
    let batch_handled_count = batch_count * VECTOR_LEN;
    let mut it = input[batch_handled_count..input.len() - if input.len() % 2 == 0 { 1 } else { 0 }]
      .array_chunks::<2>();

    let mut id = STORE_VECTOR_LEN * batch_count;
    while let Some(&[size, free]) = it.next() {
      let size = parse_digit(size);
      let free = parse_digit(free);

      unsafe {
        *empty_spaces.get_unchecked_mut(id) = free;
        *orig_counts.get_unchecked_mut(id) = size;
      }
      id += 1;
    }

    if let Some(remainder) = it.remainder().get(0) {
      let size = parse_digit(*remainder);

      unsafe {
        *empty_spaces.get_unchecked_mut(id) = 0;
        *orig_counts.get_unchecked_mut(id) = size;
      }
    }
  } else if cfg!(feature = "local") {
    assert!(input.len() % VECTOR_LEN == 0);
    // we'd don't need to handle converting the newline that we parsed as the last character into a
    // 0 to indicate that there are zero empty slots at the end.
    //
    // however, there is no situation where we'd need to move anything into the last slot, so who
    // cares how big the empty space is.

    // let last_id = empty_spaces.len() - 1;
    // unsafe {
    //   *empty_spaces.get_unchecked_mut(last_id) = 0;
    // }
  }
   */

  (orig_counts, empty_spaces, slots)
}

fn compute_fs(input: &[(u32, u32)]) -> Vec<Option<u32>> {
  let mut fs = Vec::new();
  for (id, (size, free)) in input.iter().enumerate() {
    for _ in 0..*size {
      fs.push(Some(id as u32));
    }
    for _ in 0..*free {
      fs.push(None);
    }
  }

  fs
}

pub fn part1(input: &[u8]) -> usize {
  let input = parse_input(input);
  let mut fs = compute_fs(&input);

  let mut dst_ix = 0usize;
  for src_ix in (0..fs.len()).rev() {
    let Some(id) = fs[src_ix] else { continue };

    if dst_ix >= src_ix {
      break;
    }

    while fs[dst_ix].is_some() {
      dst_ix += 1;
      if dst_ix >= src_ix {
        break;
      }
    }

    fs[dst_ix] = Some(id);
    fs[src_ix] = None;
    dst_ix += 1;
    if dst_ix >= src_ix {
      break;
    }
  }

  let mut out = 0usize;
  for i in 0..fs.len() {
    let Some(id) = fs[i] else {
      continue;
    };
    out += i * id as usize;
  }

  out
}

#[repr(align(2))]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Slot {
  pub id: u16,
  // count is elided here because it can be referred back to in the original counts array, saving
  // space and work.
}

impl Slot {
  pub fn count(&self, orig_counts: &[u8]) -> u8 {
    unsafe { *orig_counts.get_unchecked(self.id as usize) }
  }
}

#[repr(align(16))]
#[derive(Clone, Debug, PartialEq)]
struct MiniVec([u16; 8]);

impl MiniVec {
  fn len(&self) -> u16 { self.0[0] }

  fn elements<'a>(&'a self) -> &'a [u16; 6] {
    unsafe { std::mem::transmute((self.0.as_ptr() as *const u16).add(1)) }
  }

  fn elements_mut<'a>(&'a mut self) -> &'a mut [u16; 6] {
    unsafe { std::mem::transmute((self.0.as_mut_ptr() as *mut u16).add(1)) }
  }

  fn push(&mut self, item: Slot) {
    unsafe {
      let len = self.len();
      *self.elements_mut().get_unchecked_mut(len as usize) = item.id;
    }
    self.0[0] += 1;
    debug_assert!(self.len() as usize <= self.elements().len());
  }

  fn pop_front(&mut self) {
    // let out = self.elements[0];
    // for i in 1..self.len {
    //   unsafe {
    //     *self.elements.get_unchecked_mut(i as usize - 1) = self.elements[i as usize];
    //   }
    // }
    // self.len -= 1;

    // \/ does not help
    // if self.len == 1 {
    //   self.len = 0;
    //   return;
    // }

    // we should only ever mutate the vector once
    debug_assert!(self.elements()[0] != MAX_ID as u16 + 1);
    // this is a nice trick I came up with to accomplish the equivalent
    self.elements_mut()[0] = MAX_ID as u16 + 1;
  }

  fn as_slice(&self) -> &[Slot] {
    unsafe { std::mem::transmute(self.elements().get_unchecked(..self.len() as usize)) }
  }
}

const ADD_FACTORIAL_LUT: [usize; 11] = [
  0,
  0,
  1,
  2 + 1,
  3 + 2 + 1,
  4 + 3 + 2 + 1,
  5 + 4 + 3 + 2 + 1,
  6 + 5 + 4 + 3 + 2 + 1,
  7 + 6 + 5 + 4 + 3 + 2 + 1,
  8 + 7 + 6 + 5 + 4 + 3 + 2 + 1,
  9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1,
];

impl Slot {
  fn checksum(&self, total_prev: &mut usize, orig_counts: &[u8]) -> usize {
    // naive impl:
    // (0..self.count)
    //   .map(|i| (total_prev + i as usize) * self.id as usize)
    //   .sum::<usize>()

    // So, this condenses down to a sum of the following:
    //
    // (total_prev + 0) * id
    // (total_prev + 1) * id
    // (total_prev + 2) * id
    // ...
    // (total_prev + (count - 1)) * id
    //
    // the `total_prev` part can be split out:
    // total_prev * self.count * id
    //
    // leaving that base plus a sum of the following:
    //
    // 0 * id
    // 1 * id
    // 2 * id
    // ...
    // (count - 1) * id
    //
    // this reduces to (0 + 1 + 2 + ... + (count - 1)) * id
    //
    // and since count is always [0,9], we can use a tiny LUT for this which makes this whole
    // checksum essentially constant time

    let count = self.count(orig_counts) as usize;
    let checksum = *total_prev * count * self.id as usize
      + unsafe { *ADD_FACTORIAL_LUT.get_unchecked(count) } * self.id as usize;
    *total_prev += count;
    checksum
  }
}

pub fn part2(raw_input: &[u8]) -> usize {
  let (counts, mut empty_spaces, mut slots) = parse_input_p2(raw_input);

  fn checksum(
    slots: &[Slot],
    empty_space: u8,
    total_prev: &mut usize,
    orig_counts: &[u8],
  ) -> usize {
    let mut sum = 0usize;
    for slot in slots {
      sum += slot.checksum(total_prev, orig_counts);
    }
    *total_prev += empty_space as usize;
    sum
  }

  let mut start_span_ix_by_needed_size: [u16; 10] = [0; 10];
  let mut finished_digit_count = 0usize;
  // we keep track of the highest span that still has a value in it.
  //
  // this allows us to skip iterating over fully empty spans at the end when computing the checksum
  let mut max_unmoved_src_id = 0;
  'outer: for src_id in (0..=MAX_ID).rev() {
    let src_count = unsafe { *counts.get_unchecked(src_id) };

    let start_ix =
      unsafe { *start_span_ix_by_needed_size.get_unchecked(src_count as usize) } as usize;

    // we can only move elements to the left
    if start_ix >= src_id {
      if start_ix != u16::MAX as usize {
        max_unmoved_src_id = max_unmoved_src_id.max(src_id);
        debug_assert!(slots[max_unmoved_src_id + 1..]
          .iter()
          .all(|s| s.as_slice().is_empty()
            || s
              .as_slice()
              .iter()
              .all(|s| s.id == 0 || s.count(&counts) == 0)));

        finished_digit_count += 1;
        if finished_digit_count == 9 {
          debug_assert_eq!(
            start_span_ix_by_needed_size[0], 0,
            "there are never zero-size files in the inputs apparently"
          );
          break;
        }

        // mark this finished and all bigger digits too

        // unsafe {
        //   start_span_ix_by_needed_size
        //     .get_unchecked_mut(src_count as usize..10)
        //     .fill(u16::MAX);
        // }

        // this is faster than above and creates huge diff in the resulting assembly across the
        // whole binary, although the callsite looks almost identical with a `memset` still getting
        // generated.
        //
        // also, `get_unchecked_mut` generates idential code; bounds checks are elided automatically
        // somehow
        for i in src_count as usize..10 {
          start_span_ix_by_needed_size[i] = u16::MAX;
        }
      }

      continue;
    }

    let src_id = src_id as u16;

    let start_ptr = unsafe { empty_spaces.as_ptr().add(start_ix) };
    let mut cur_ptr = start_ptr;
    let mut cur_offset = 0usize;
    let mut dst_span_ix = loop {
      const VEC_SIZE: usize = 8usize;
      // let end_ix = start_ix + cur_offset + VEC_SIZE;
      // same caveat as before.  For a 100% correct implementation for all possible inputs, we'd
      // need to handle manually checking the tail here but I'm leaving that out
      //
      // I could leave this off if I wanted to and it wouldn't be an issue...
      // if end_ix > input.len() - VEC_SIZE {
      //   start_span_ix_by_needed_size[src_count as usize] = usize::MAX;
      //   finished_digit_count += 1;
      //   max_unmoved_src_id = max_unmoved_src_id.max(src_id as usize);
      //   continue 'outer;
      // }

      let empty_spaces_v: u8x8 = unsafe { std::ptr::read_unaligned(cur_ptr as *const _) };
      debug_assert_eq!(empty_spaces_v.len(), VEC_SIZE);
      let mask = empty_spaces_v.simd_ge(u8x8::splat(src_count));
      match mask.first_set() {
        Some(i) => {
          let dst_span_ix = start_ix + cur_offset + i;
          if dst_span_ix >= src_id as usize {
            unsafe {
              *start_span_ix_by_needed_size.get_unchecked_mut(src_count as usize) = u16::MAX
            };
            finished_digit_count += 1;
            max_unmoved_src_id = max_unmoved_src_id.max(src_id as usize);
            continue 'outer;
          }
          debug_assert!(empty_spaces[dst_span_ix] >= src_count);
          break dst_span_ix;
        },
        None => {
          cur_ptr = unsafe { cur_ptr.add(VEC_SIZE) };
          cur_offset += VEC_SIZE
        },
      }
    };

    let dst_slots: &mut MiniVec = unsafe { slots.get_unchecked_mut(dst_span_ix) };
    max_unmoved_src_id = max_unmoved_src_id.max(dst_span_ix);
    dst_slots.push(Slot { id: src_id });

    let new_empty_space = unsafe { *empty_spaces.get_unchecked(dst_span_ix) - src_count };
    unsafe { *empty_spaces.get_unchecked_mut(dst_span_ix) = new_empty_space };

    // no way we could fit more of this size into this span, so might as well move on to the next
    // one before continuing to loop
    if new_empty_space < src_count {
      dst_span_ix += 1;
    }

    unsafe {
      *start_span_ix_by_needed_size.get_unchecked_mut(src_count as usize) = dst_span_ix as u16
    };

    // \/ this code uses the fact that if a span of size `src_count` can't fit before `dst_span_ix`,
    // then no bigger span could either.
    //
    // However, it turns out to make things slower - especially when compiling with
    // `target-cpu=native`.  That causes some fancy SIMD that performs this operation using masks
    // and whatnot to be emitted, but that turns out to be way slower than the scalar version.
    //
    // Anyway, just skipping all this work here seems to be the fastest method of them all, probably
    // because our SIMD free slot search is fast enough to make up for the savings of doing the more
    // fancy accounting after the bookkeeping overhead.
    //
    // for i in src_count as usize..10 {
    //   start_span_ix_by_needed_size[i] = start_span_ix_by_needed_size[i].max(dst_span_ix);
    // }

    // the element we're removing is at the first index of the array since any others added to this
    // span will have been put after it
    let src_slots = unsafe { slots.get_unchecked_mut(src_id as usize) };
    // debug_assert_eq!(src_slots.elements[0].id, src_id);
    unsafe { *empty_spaces.get_unchecked_mut(src_id as usize - 1) += src_count };
    src_slots.pop_front();
  }

  let mut out = 0usize;
  let mut total_prev = 0usize;
  for (slot, &empty_count) in unsafe { slots.get_unchecked_mut(..=max_unmoved_src_id) }
    .iter()
    .zip(unsafe { empty_spaces.get_unchecked_mut(..=max_unmoved_src_id) }.iter())
  {
    out += checksum(slot.as_slice(), empty_count, &mut total_prev, &counts);
  }

  out
}

#[cfg(feature = "local")]
pub fn solve() {
  use crate::helpers::leak_to_page_aligned;

  let aligned_input = leak_to_page_aligned(INPUT);

  let out = part1(aligned_input);

  println!("Part 1: {}", out);

  let out = part2(aligned_input);

  println!("Part 2: {}", out);
}

pub fn run(input: &[u8]) -> impl Display { part2(input) }

