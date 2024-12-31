// Original by: giooschi
#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

// pub fn run(input: &str) -> &'static str {
//     part2(input)
// }

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> &'static str {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut xids = [u8::MAX; 45];
    let mut yids = [u8::MAX; 45];

    let mut ptr = input.as_ptr();
    for i in 0..45 {
        xids[i] = (*ptr.add(5) - b'0') + 46;
        ptr = ptr.add(7);
    }
    for i in 0..45 {
        yids[i] = (*ptr.add(5) - b'0') + 46;
        ptr = ptr.add(7);
    }
    ptr = ptr.add(1);

    let mut node_to_id = [u8::MAX; 23 * 26 * 26];
    let mut next_id = 48;

    macro_rules! get_id {
        ($a:ident, $b:ident, $c:ident) => {{
            let node =
                26 * 26 * ($a - b'a' as usize) + 26 * ($b - b'a' as usize) + ($c - b'a' as usize);
            let mut id = *node_to_id.get_unchecked(node);
            if id == u8::MAX {
                id = next_id;
                *node_to_id.get_unchecked_mut(node) = id;
                next_id += 1;
            }
            id
        }};
    }

    static mut OPS: [[u8; 3]; 224] = [[u8::MAX; 3]; 224];
    let ops = &mut OPS;

    let end = input.as_ptr().add(input.len());
    loop {
        let a = *ptr as usize;
        let b = *ptr.add(1) as usize;
        let c = *ptr.add(2) as usize;
        ptr = ptr.add(4);

        let (n, op, m) = if a == b'x' as usize {
            let idx = 10 * (b - b'0' as usize) + (c - b'0' as usize);
            let n = *xids.get_unchecked(idx);
            let m = *yids.get_unchecked(idx);
            let op = (*ptr == b'X') as u8;
            ptr = ptr.add(11);
            (n, op, m)
        } else if a == b'y' as usize {
            let idx = 10 * (b - b'0' as usize) + (c - b'0' as usize);
            let n = *yids.get_unchecked(idx);
            let m = *xids.get_unchecked(idx);
            let op = (*ptr == b'X') as u8;
            ptr = ptr.add(11);
            (n, op, m)
        } else {
            let n = get_id!(a, b, c);

            let op = *ptr;
            let op = if op == b'O' {
                ptr = ptr.add(3);
                2
            } else {
                ptr = ptr.add(4);
                (op == b'X') as u8
            };

            let a = *ptr as usize;
            let b = *ptr.add(1) as usize;
            let c = *ptr.add(2) as usize;
            ptr = ptr.add(7);

            let m = get_id!(a, b, c);

            (n, op, m)
        };

        let a = *ptr as usize;
        let b = *ptr.add(1) as usize;
        let c = *ptr.add(2) as usize;
        ptr = ptr.add(4);

        let out = if a == b'z' as usize {
            (10 * (b - b'0' as usize) + (c - b'0' as usize)) as u8
        } else {
            get_id!(a, b, c)
        };

        *ops.get_unchecked_mut(out as usize) = [n, op, m];

        if ptr == end {
            break;
        }
    }

    let mut values = [u8::MAX; 224];
    values[46] = 0;
    values[47] = 1;

    let mut out = 0;

    for z in 0..46 {
        macro_rules! calc_rec {
            (force [fuel: $($fuel:tt)*] $n:expr) => {{
                let n = $n as usize;
                let [l, op, r] = *ops.get_unchecked(n);
                let l = calc_rec!([fuel: $($fuel)*] l);
                let r = calc_rec!([fuel: $($fuel)*] r);
                match op {
                    0 => l & r,
                    1 => l ^ r,
                    2 => l | r,
                    _ => std::hint::unreachable_unchecked(),
                }
            }};
            ([fuel:] $n:expr) => {
                *values.get_unchecked($n as usize)
            };
            ([fuel: f $($rest:tt)*] $n:expr) => {{
                let n = $n  as usize;
                let mut v = *values.get_unchecked(n);
                if v == u8::MAX {
                    v = calc_rec!(force [fuel: $($rest)*] n);
                    *values.get_unchecked_mut(n) = v;
                }
                v
            }};
        }

        out |= (calc_rec!(force [fuel: f f f f f] z) as u64) << z;
    }

    out
}

static mut PART2_OUT: [u8; 8 * 3 + 7] = [b','; 8 * 3 + 7];

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> &'static str {
    let input = input.as_bytes();

    let mut node_to_id = [u8::MAX; 23 * 26 * 26];
    let mut id_to_node = [u16::MAX; 222];
    let mut next_id = 46;

    static mut XYOPS: [[u8; 2]; 45] = [[u8::MAX; 2]; 45];
    let mut xyops = &mut XYOPS;

    static mut OPS: [[[u8; 2]; 2]; 222] = {
        let mut ops = [[[u8::MAX; 2]; 2]; 222];

        let mut i = 0;
        while i < 46 {
            ops[i] = [[u8::MAX - 1; 2]; 2];
            i += 1;
        }

        ops
    };
    let ops = &mut OPS;

    macro_rules! get_id {
        ($a:ident, $b:ident, $c:ident) => {{
            let node =
                26 * 26 * ($a - b'a' as usize) + 26 * ($b - b'a' as usize) + ($c - b'a' as usize);
            let mut id = *node_to_id.get_unchecked(node);
            if id == u8::MAX {
                id = next_id;
                *node_to_id.get_unchecked_mut(node) = id;
                *id_to_node.get_unchecked_mut(id as usize) = node as u16;
                next_id += 1;
            }
            id
        }};
    }

    let mut ptr = input.as_ptr().add(631);
    let end = input.as_ptr().add(input.len());
    loop {
        let a = *ptr as usize;
        let b = *ptr.add(1) as usize;
        let c = *ptr.add(2) as usize;
        ptr = ptr.add(4);

        if a == b'x' as usize || a == b'y' as usize {
            let n = 10 * (b - b'0' as usize) + (c - b'0' as usize);
            let off = (*ptr == b'X') as usize;

            ptr = ptr.add(11);

            let a = *ptr as usize;
            let b = *ptr.add(1) as usize;
            let c = *ptr.add(2) as usize;
            ptr = ptr.add(4);

            let out = if a == b'z' as usize {
                (10 * (b - b'0' as usize) + (c - b'0' as usize)) as u8
            } else {
                get_id!(a, b, c)
            };

            *xyops.get_unchecked_mut(n).get_unchecked_mut(off) = out;
        } else {
            let n = get_id!(a, b, c);

            let op = *ptr;
            ptr = ptr.add(3);
            if op != b'O' {
                ptr = ptr.add(1);
            }
            let off = (op == b'X') as usize;

            let a = *ptr as usize;
            let b = *ptr.add(1) as usize;
            let c = *ptr.add(2) as usize;
            ptr = ptr.add(7);
            let m = get_id!(a, b, c);

            let a = *ptr as usize;
            let b = *ptr.add(1) as usize;
            let c = *ptr.add(2) as usize;
            ptr = ptr.add(4);

            let out = if a == b'z' as usize {
                (10 * (b - b'0' as usize) + (c - b'0' as usize)) as u8
            } else {
                get_id!(a, b, c)
            };

            if op == b'O' {
                *ops.get_unchecked_mut(n as usize).get_unchecked_mut(1) = [u8::MAX; 2];
                *ops.get_unchecked_mut(m as usize).get_unchecked_mut(1) = [u8::MAX; 2];
            }

            *ops.get_unchecked_mut(n as usize).get_unchecked_mut(off) = [m, out];
            *ops.get_unchecked_mut(m as usize).get_unchecked_mut(off) = [n, out];
        }

        if ptr == end {
            break;
        }
    }

    let mut out = [u16::MAX; 8];
    let mut out_len = 0;

    let mut carry = xyops[0][0] as usize;

    for n in 1..45 {
        let act_carry_1 = xyops[n][0] as usize;
        let act_res = xyops[n][1] as usize;
        let exp_res = ops.get_unchecked(carry)[0][0] as usize;
        let act_carry_2 = ops.get_unchecked(carry)[0][1] as usize;
        let act_z = ops.get_unchecked(carry)[1][1] as usize;

        if act_z >= 46 {
            *out.get_unchecked_mut(out_len) = act_z as u16;
            *out.get_unchecked_mut(out_len + 1) = n as u16;
            out_len += 2;

            debug_assert!(act_z < 222);
            debug_assert!(n < 222);

            if ops.get_unchecked(act_carry_1)[1] == [u8::MAX; 2] {
                carry = ops.get_unchecked(act_carry_1)[0][1] as usize;
            } else {
                carry = ops.get_unchecked(act_carry_2)[0][1] as usize;
            }
            if carry == n {
                carry = act_z;
            }
        } else {
            if act_res != exp_res {
                *out.get_unchecked_mut(out_len) = act_res as u16;
                out_len += 1;
                debug_assert!(act_res < 222);
            }

            if ops.get_unchecked(act_carry_1)[1] != [u8::MAX; 2] {
                *out.get_unchecked_mut(out_len) = act_carry_1 as u16;
                out_len += 1;
                debug_assert!(act_carry_1 < 222);
            } else {
                carry = ops.get_unchecked(act_carry_1)[0][1] as usize;
            }

            if ops.get_unchecked(act_carry_2)[1] != [u8::MAX; 2] {
                *out.get_unchecked_mut(out_len) = act_carry_2 as u16;
                out_len += 1;
                debug_assert!(act_carry_2 < 222);
            } else {
                carry = ops.get_unchecked(act_carry_2)[0][1] as usize;
            }

            if out_len & 1 != 0 {
                *out.get_unchecked_mut(out_len) = carry as u16;
                out_len += 1;
                debug_assert!(carry < 222);

                carry = *out.get_unchecked(out_len - 2) as usize;
            }
        }

        if out_len == 8 {
            break;
        }
    }

    debug_assert_eq!(out_len, 8);

    let mut out_chr = [[u8::MAX; 3]; 8];
    for i in 0..8 {
        let n = out[i];
        if n < 46 {
            out_chr[i] = [b'z', b'0' + n as u8 / 10, b'0' + n as u8 % 10];
        } else {
            let n = id_to_node[n as usize];
            out_chr[i] = [
                b'a' + (n / (26 * 26)) as u8,
                b'a' + (n / 26 % 26) as u8,
                b'a' + (n % 26) as u8,
            ];
        }
    }

    out_chr.sort_unstable();

    for i in 0..8 {
        PART2_OUT[4 * i + 0] = out_chr[i][0];
        PART2_OUT[4 * i + 1] = out_chr[i][1];
        PART2_OUT[4 * i + 2] = out_chr[i][2];
    }

    std::str::from_utf8_unchecked(&PART2_OUT)
}
