// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::mem::MaybeUninit;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u32 {
    unsafe { inner_part2(input) }
}

const START: u32 = 142 * 139 + 1;
const END: u32 = 142 * 1 + 139;

const UP: usize = -142isize as _;
const DOWN: usize = 142;
const LEFT: usize = -1isize as _;
const RIGHT: usize = 1;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut seen_h = [0u64; (2 * 141 * 142 + 63) / 64];
    let mut seen_v = [0u64; (2 * 141 * 142 + 63) / 64];

    let mut queue_h = [MaybeUninit::uninit(); 256];
    let mut queue_v = [MaybeUninit::uninit(); 256];

    let mut queue_h_len = 1;
    let mut queue_v_len = 1;

    *queue_h.get_unchecked_mut(0).as_mut_ptr() = (START, 0);
    *queue_v.get_unchecked_mut(0).as_mut_ptr() = (START, 1000);

    let mut end_cost = u32::MAX;

    macro_rules! advance {
        ($pos:ident, $cost:ident, $dir:ident, $pdir1:ident, $pdir2:ident, $queue:ident, $queue_len:ident, $seen:ident) => {{
            let mut next_pos = $pos as usize;
            let mut next_cost = $cost;
            'advance: loop {
                next_pos = next_pos.wrapping_add($dir);
                next_cost += 1;

                if *input.get_unchecked(next_pos) != b'.' {
                    if next_pos as u32 == END {
                        end_cost = end_cost.min(next_cost);
                    }
                    break;
                }

                let up = next_pos.wrapping_add($pdir1);
                let down = next_pos.wrapping_add($pdir2);

                if *input.get_unchecked(up) != b'#' || *input.get_unchecked(down) != b'#' {
                    if *$seen.get_unchecked(next_pos / 64) & (1 << (next_pos % 64)) == 0 {
                        let idx = 2 * next_pos;
                        if $seen.get_unchecked(idx / 64) & (1 << (idx % 64)) != 0 {
                            for i in 0..$queue_len {
                                let (old_pos, old_cost) =
                                    &mut *$queue.get_unchecked_mut(i).as_mut_ptr();
                                if *old_pos == next_pos as u32 {
                                    *old_cost = (*old_cost).min(next_cost + 1000);
                                    continue 'advance;
                                }
                            }
                        }
                        *$seen.get_unchecked_mut(idx / 64) |= 1 << (idx % 64);
                        let ptr = $queue.get_unchecked_mut($queue_len).as_mut_ptr();
                        *ptr = (next_pos as u32, next_cost + 1000);
                        $queue_len += 1;
                    }
                }
            }
        }};
    }

    loop {
        for i in 0..std::mem::take(&mut queue_h_len) {
            let (pos, cost) = *queue_h.get_unchecked(i).as_ptr();
            *seen_h.get_unchecked_mut(pos as usize / 64) |= 1 << (pos % 64);
            advance!(pos, cost, LEFT, UP, DOWN, queue_v, queue_v_len, seen_v);
            advance!(pos, cost, RIGHT, UP, DOWN, queue_v, queue_v_len, seen_v);
        }

        if end_cost != u32::MAX {
            return end_cost;
        }

        for i in 0..std::mem::take(&mut queue_v_len) {
            let (pos, cost) = *queue_v.get_unchecked(i).as_ptr();
            *seen_v.get_unchecked_mut(pos as usize / 64) |= 1 << (pos % 64);
            advance!(pos, cost, UP, LEFT, RIGHT, queue_h, queue_h_len, seen_h);
            advance!(pos, cost, DOWN, LEFT, RIGHT, queue_h, queue_h_len, seen_h);
        }

        if end_cost != u32::MAX {
            return end_cost;
        }
    }
}

const LEFT_ID: DirId = DirId(0b00);
const RIGHT_ID: DirId = DirId(0b11);
const UP_ID: DirId = DirId(0b10);
const DOWN_ID: DirId = DirId(0b01);

static DIR_MAP: [usize; 4] = {
    let mut dirs = [0; 4];
    dirs[LEFT_ID.idx()] = LEFT;
    dirs[RIGHT_ID.idx()] = RIGHT;
    dirs[UP_ID.idx()] = UP;
    dirs[DOWN_ID.idx()] = DOWN;
    dirs
};

#[derive(Copy, Clone)]
#[repr(transparent)]
struct DirId(u16);

impl DirId {
    const fn parity(self) -> usize {
        (self.0 & 1) as usize
    }
    const fn kind(self) -> u16 {
        (self.0 ^ (self.0 >> 1)) & 1
    }
    const fn invert(self) -> DirId {
        Self(self.0 ^ 0b11)
    }
    const fn perp1(self) -> DirId {
        Self(self.0 ^ 0b01)
    }
    const fn perp2(self) -> DirId {
        Self(self.0 ^ 0b10)
    }
    const fn idx(self) -> usize {
        self.0 as usize
    }
}

const START_H_ID: u16 = 0;
const START_V_ID: u16 = START_H_ID + 1;
const END_H_ID: u16 = 2;
const END_V_ID: u16 = END_H_ID + 1;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut ids = [u16::MAX; 141 * 142];
    ids[START as usize] = START_H_ID;
    ids[END as usize] = END_H_ID;
    let mut next_id = 4;

    let mut moves = [MaybeUninit::<[(u16, u8, u8); 2]>::uninit(); 3000];
    moves[START_H_ID as usize] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);
    moves[START_V_ID as usize] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);
    moves[END_H_ID as usize] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);
    moves[END_V_ID as usize] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);

    let mut queue = [MaybeUninit::uninit(); 512];
    let mut queue_len = 0;
    if *input.get_unchecked((START as usize).wrapping_add(RIGHT)) != b'#' {
        *queue.get_unchecked_mut(queue_len).as_mut_ptr() = (START, RIGHT_ID, START_H_ID);
        queue_len += 1;
    }
    if *input.get_unchecked((START as usize).wrapping_add(UP)) != b'#' {
        *queue.get_unchecked_mut(queue_len).as_mut_ptr() = (START, UP_ID, START_V_ID);
        queue_len += 1;
    }

    'queue: while queue_len != 0 {
        queue_len -= 1;
        let (pos, start_dir_id, start_id) = *queue.get_unchecked(queue_len).as_ptr();

        let m = &*moves.get_unchecked(start_id as usize).as_ptr();
        if m.get_unchecked(start_dir_id.parity() as usize).0 != u16::MAX {
            continue;
        }

        let mut pos = pos as usize;
        let mut dir_id = start_dir_id;
        let mut turns = 0;
        let mut tiles = 0;

        let mut dir = *DIR_MAP.get_unchecked(dir_id.idx());
        let mut dir1 = *DIR_MAP.get_unchecked(dir_id.perp1().idx());
        let mut dir2 = *DIR_MAP.get_unchecked(dir_id.perp2().idx());

        debug_assert_ne!(input[pos] as char, '#');
        debug_assert_ne!(
            input[pos.wrapping_add(dir)] as char,
            '#',
            "{} {}, {} {}, {}",
            pos % 142,
            pos / 142,
            pos.wrapping_add(dir) % 142,
            pos.wrapping_add(dir) / 142,
            dir as isize
        );

        'inner: loop {
            pos = pos.wrapping_add(dir);
            tiles += 1;

            let cont = *input.get_unchecked(pos.wrapping_add(dir)) != b'#';
            let cont1 = *input.get_unchecked(pos.wrapping_add(dir1)) != b'#';
            let cont2 = *input.get_unchecked(pos.wrapping_add(dir2)) != b'#';

            debug_assert_ne!(input[pos] as char, '#', "{} {}", pos % 142, pos / 142);

            if !cont1 && !cont2 {
                if cont {
                    // go straight
                    continue 'inner;
                } else if pos != START as usize && pos != END as usize {
                    // deadend
                    continue 'queue;
                }
            }

            if cont || (cont1 && cont2) || pos == START as usize || pos == END as usize {
                // new node

                let mut dest_id = *ids.get_unchecked(pos) | dir_id.kind();
                if dest_id == u16::MAX {
                    dest_id = next_id | dir_id.kind();

                    *ids.get_unchecked_mut(pos) = next_id;
                    *moves.get_unchecked_mut(next_id as usize).as_mut_ptr() = [(u16::MAX, 0, 0); 2];
                    *moves.get_unchecked_mut(next_id as usize + 1).as_mut_ptr() =
                        [(u16::MAX, 0, 0); 2];

                    debug_assert!(dest_id == next_id || dest_id == next_id + 1);

                    next_id += 2;

                    let m = &*moves.get_unchecked(dest_id as usize).as_ptr();
                    if cont {
                        debug_assert_eq!(m.get_unchecked(dir_id.invert().parity()).0, u16::MAX);
                        *queue.get_unchecked_mut(queue_len).as_mut_ptr() =
                            (pos as u32, dir_id, dest_id);
                        queue_len += 1;
                    }

                    let m = &*moves.get_unchecked(dest_id as usize ^ 1).as_ptr();

                    let dir1_id = dir_id.perp1();
                    if cont1 && m.get_unchecked(dir1_id.parity()).0 == u16::MAX {
                        *queue.get_unchecked_mut(queue_len).as_mut_ptr() =
                            (pos as u32, dir1_id, dest_id ^ 1);
                        queue_len += 1;
                    }

                    let dir2_id = dir_id.perp2();
                    if cont2 && m.get_unchecked(dir2_id.parity()).0 == u16::MAX {
                        *queue.get_unchecked_mut(queue_len).as_mut_ptr() =
                            (pos as u32, dir2_id, dest_id ^ 1);
                        queue_len += 1;
                    }
                }

                // if start_id & !1 == 0 && dest_id & !1 == 4 {
                //     println!(
                //         "setting! {start_id} -> {dest_id} [{} {}] {turns} {tiles}",
                //         dir_id.invert().parity(),
                //         dir_id.kind(),
                //     );
                // }

                // if start_id & !1 == 4 && dest_id & !1 == 10 {
                //     println!(
                //         "setting! {start_id} -> {dest_id} [{} {}] {turns} {tiles}",
                //         dir_id.invert().parity(),
                //         dir_id.kind(),
                //     );
                // }

                // if start_id & !1 == 10 && dest_id & !1 == 4 {
                //     println!(
                //         "setting! {start_id} -> {dest_id} [{} {}] {turns} {tiles}",
                //         dir_id.invert().parity(),
                //         dir_id.kind(),
                //     );
                // }

                *(*moves.get_unchecked_mut(start_id as usize).as_mut_ptr())
                    .get_unchecked_mut(start_dir_id.parity()) = (dest_id, turns, tiles);
                *(*moves.get_unchecked_mut(dest_id as usize).as_mut_ptr())
                    .get_unchecked_mut(dir_id.invert().parity()) = (start_id, turns, tiles);

                continue 'queue;
            } else {
                // turn

                dir_id = if cont1 {
                    dir_id.perp1()
                } else {
                    dir_id.perp2()
                };
                dir = *DIR_MAP.get_unchecked(dir_id.idx());
                dir1 = *DIR_MAP.get_unchecked(dir_id.perp1().idx());
                dir2 = *DIR_MAP.get_unchecked(dir_id.perp2().idx());
                turns += 1;

                continue 'inner;
            }
        }
    }

    // TODO reuse previous queue
    let mut queue = MaybeUninit::<[u64; 128]>::uninit();
    *queue.as_mut_ptr().cast::<[u32; 2]>().add(0) = [START_H_ID as u32, 0];
    let mut queue_len = 1;

    macro_rules! mk_queue_slice {
        () => {{
            debug_assert!(queue_len <= 128);
            std::slice::from_raw_parts_mut(queue.as_mut_ptr().cast::<u64>(), queue_len)
        }};
    }

    let (_, costs, _) = ids.align_to_mut::<u32>();
    let costs = &mut costs[..next_id as usize];
    costs.fill(u32::MAX / 2);

    let end_cost = loop {
        // TODO: Binary queue
        let [id, cost] = *queue.as_ptr().cast::<[u32; 2]>();
        bheap::pop(mk_queue_slice!());
        queue_len -= 1;

        // let pos = positions[id as usize / 2];
        // let (x, y) = (pos % 142, pos / 142);
        // println!("{id:>4}: {x:>3} {y:>3} | {cost}");
        // println!("{:?}", mk_queue_slice!());

        // println!("{id:>4} | {cost}");

        // if cost > 10000 {
        //     panic!();
        // }

        let cost_ref = costs.get_unchecked_mut(id as usize);
        if *cost_ref <= cost {
            continue;
        }
        *cost_ref = cost;

        let cost_swap = costs.get_unchecked_mut(id as usize ^ 1);
        *cost_swap = (*cost_swap).min(cost + 1000);

        if id as u16 & !1 == END_H_ID {
            break cost;
        }

        let m = *moves.get_unchecked(id as usize).as_ptr();
        let (next_id, turns, tiles) = m[0];
        let next_cost = cost + turns as u32 * 1000 + tiles as u32;
        if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) > next_cost {
            // TODO: insert with extra_cost
            *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
            queue_len += 1;
            bheap::push(mk_queue_slice!());
        }
        let (next_id, turns, tiles) = m[1];
        let next_cost = cost + turns as u32 * 1000 + tiles as u32;
        if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) > next_cost {
            // TODO: insert with extra_cost
            *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
            queue_len += 1;
            bheap::push(mk_queue_slice!());
        }

        // if *costs.get_unchecked(id as usize ^ 1) > cost + 1000 {
        //     *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [id as u32 ^ 1, cost + 1000];
        //     queue_len += 1;
        //     bheap::push(mk_queue_slice!());
        // }

        let m = *moves.get_unchecked(id as usize ^ 1).as_ptr();
        let (next_id, turns, tiles) = m[0];
        let next_cost = cost + turns as u32 * 1000 + 1000 + tiles as u32;
        if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) > next_cost {
            // TODO: insert with extra_cost + 1000
            *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
            queue_len += 1;
            bheap::push(mk_queue_slice!());
        }
        let (next_id, turns, tiles) = m[1];
        let next_cost = cost + turns as u32 * 1000 + 1000 + tiles as u32;
        if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) > next_cost {
            // TODO: insert with extra_cost + 1000
            *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
            queue_len += 1;
            bheap::push(mk_queue_slice!());
        }
    };

    while queue_len != 0 {
        let [id, cost] = *queue.as_ptr().cast::<[u32; 2]>();

        if cost > end_cost {
            break;
        }

        let cost_ref = costs.get_unchecked_mut(id as usize);
        if *cost_ref > cost {
            *cost_ref = cost;
            let cost_swap = costs.get_unchecked_mut(id as usize ^ 1);
            *cost_swap = (*cost_swap).min(cost + 1000);
        }

        bheap::pop(mk_queue_slice!());
        queue_len -= 1;
    }

    queue_len = 0;
    if *costs.get_unchecked(END_H_ID as usize) == end_cost {
        *queue.as_mut_ptr().cast::<[u32; 2]>().add(0) = [END_H_ID as u32, end_cost];
        queue_len += 1;
    }
    if *costs.get_unchecked(END_V_ID as usize) == end_cost {
        *queue.as_mut_ptr().cast::<[u32; 2]>().add(0) = [END_V_ID as u32, end_cost];
        queue_len += 1;
    }

    // println!("{:?} {:?}", costs[4], costs[5]);
    // println!("{:?} {:?}", costs[0], costs[1]);

    // println!("{:?} {:?}", moves[4].assume_init(), moves[5].assume_init());
    // println!("{:?} {:?}", moves[0].assume_init(), moves[1].assume_init());

    #[cfg(debug_assertions)]
    let mut seen = std::collections::HashSet::new();

    let mut count = 0;
    while queue_len != 0 {
        queue_len -= 1;
        let [id, ex_cost] = *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len);

        // if id == START_H_ID as _ {
        //     println!("start h! 1");
        // }
        // if id == START_V_ID as _ {
        //     println!("start v! 1");
        // }

        const SMASK: u32 = 1 << 31;

        let cost_ref = costs.get_unchecked_mut(id as usize);
        if *cost_ref != ex_cost {
            debug_assert!((*cost_ref > ex_cost && *cost_ref < end_cost) || *cost_ref & SMASK != 0);
            continue;
        }
        *cost_ref |= SMASK;

        // println!("{id}, {ex_cost}");

        #[cfg(debug_assertions)]
        debug_assert!(seen.insert(id));

        if *costs.get_unchecked(id as usize ^ 1) & SMASK == 0 {
            #[cfg(debug_assertions)]
            debug_assert!(!seen.contains(&(id ^ 1)));
            count += 1;
        } else {
            #[cfg(debug_assertions)]
            debug_assert!(seen.contains(&(id ^ 1)));
        }

        // if id == START_H_ID as _ {
        //     println!("start h!");
        // }
        // if id == START_V_ID as _ {
        //     println!("start v!");
        // }

        let m = *moves.get_unchecked(id as usize).as_ptr();
        let (next_id, turns, tiles) = m[0];
        let next_cost = ex_cost.wrapping_sub(turns as u32 * 1000 + tiles as u32);
        if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) & !SMASK == next_cost {
            // TODO: insert with extra_cost
            *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
            queue_len += 1;
            count += tiles as u32 - 1;
        }
        let (next_id, turns, tiles) = m[1];
        let next_cost = ex_cost.wrapping_sub(turns as u32 * 1000 + tiles as u32);
        if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) & !SMASK == next_cost {
            // TODO: insert with extra_cost
            *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
            queue_len += 1;
            count += tiles as u32 - 1;
        }

        let next_cost = ex_cost.wrapping_sub(1000);
        if *costs.get_unchecked(id as usize ^ 1) & !SMASK == next_cost {
            // TODO: insert with extra_cost + 1000
            *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [id as u32 ^ 1, next_cost];
            queue_len += 1;
        }

        // let m = *moves.get_unchecked(id as usize ^ 1).as_ptr();
        // let (next_id, turns, tiles) = m[0];
        // let next_cost = ex_cost.wrapping_sub(turns as u32 * 1000 + 1000 + tiles as u32);
        // if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) & !SMASK == next_cost {
        //     // TODO: insert with extra_cost + 1000
        //     *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
        //     queue_len += 1;
        //     count += tiles as u32 - 1;
        // }
        // let (next_id, turns, tiles) = m[1];
        // let next_cost = ex_cost.wrapping_sub(turns as u32 * 1000 + 1000 + tiles as u32);
        // if next_id != u16::MAX && *costs.get_unchecked(next_id as usize) & !SMASK == next_cost {
        //     // TODO: insert with extra_cost + 1000
        //     *queue.as_mut_ptr().cast::<[u32; 2]>().add(queue_len) = [next_id as u32, next_cost];
        //     queue_len += 1;
        //     count += tiles as u32 - 1;
        // }
    }
    // println!("{:?} {:?}", costs[4], costs[5]);
    // println!("{:?} {:?}", costs[0], costs[1]);

    count
}

mod bheap {
    #[inline(always)]
    pub unsafe fn pop<T: Copy + Ord>(heap: &mut [T]) {
        if heap.len() > 1 {
            // len = len - 1
            //
            // sift_down_to_bottom(0)

            let start = 0;
            let end = heap.len() - 1;

            let hole = *heap.get_unchecked(heap.len() - 1);
            let mut hole_pos = start;
            let mut child = 2 * hole_pos + 1;

            while child <= end.saturating_sub(2) {
                child += (*heap.get_unchecked(child) >= *heap.get_unchecked(child + 1)) as usize;

                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(child);
                hole_pos = child;

                child = 2 * hole_pos + 1;
            }

            if child == end - 1 {
                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(child);
                hole_pos = child;
            }

            // sift_up(start, hole_pos)
            while hole_pos > start {
                let parent = (hole_pos - 1) / 2;

                if hole >= *heap.get_unchecked(parent) {
                    break;
                }

                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(parent);
                hole_pos = parent;
            }

            *heap.get_unchecked_mut(hole_pos) = hole;
        }
    }

    #[inline(always)]
    pub unsafe fn push<T: Copy + Ord>(heap: &mut [T]) {
        // sift_up(0, heap.len() - 1)
        let start = 0;
        let pos = heap.len() - 1;

        let hole = *heap.get_unchecked(pos);
        let mut hole_pos = pos;

        while hole_pos > start {
            let parent = (hole_pos - 1) / 2;

            if hole >= *heap.get_unchecked(parent) {
                break;
            }

            *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(parent);
            hole_pos = parent;
        }

        *heap.get_unchecked_mut(hole_pos) = hole;
    }
}
