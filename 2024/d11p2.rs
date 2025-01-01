// Original by: giooschi
#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

static mut LUT: [u64; 10_000_000] = [0; 10_000_000];

#[cfg_attr(any(target_os = "linux"), link_section = ".text.startup")]
unsafe extern "C" fn __ctor() {
    make_d11_lut();
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(windows, link_section = ".CRT$XCU")]
static __CTOR: unsafe extern "C" fn() = __ctor;

fn make_d11_lut() {
    use std::collections::HashMap;
    let iters = 75;

    let mut levels = vec![HashMap::new(); iters];

    fn solve_rec(i: usize, j: usize, levels: &mut [HashMap<usize, usize>]) -> usize {
        if i == 0 {
            return 1;
        }

        if let Some(&res) = levels[i - 1].get(&j) {
            return res;
        }

        let res = if j == 0 {
            solve_rec(i - 1, 1, levels)
        } else if j.ilog10() % 2 == 1 {
            let pow10 = 10usize.pow((j.ilog10() + 1) / 2);
            solve_rec(i - 1, j / pow10, levels) + solve_rec(i - 1, j % pow10, levels)
        } else {
            solve_rec(i - 1, j * 2024, levels)
        };

        levels[i - 1].insert(j, res);

        res
    }

    for j in 0..10_000_000 {
        (unsafe { &mut LUT })[j] = solve_rec(iters, j, &mut levels) as _;
    }
}

macro_rules! solve {
    ($input:ident, $lut:literal, $ty:ident) => {{
        let lut = LUT.as_ptr();

        let input = $input.as_bytes();
        let mut ptr = input.as_ptr();
        let end = ptr.add(input.len());

        let mut tot = 0;

        loop {
            let mut n = (*ptr - b'0') as usize;
            ptr = ptr.add(1);

            loop {
                let d = (*ptr).wrapping_sub(b'0');
                ptr = ptr.add(1);
                if d >= 10 {
                    break;
                }
                n = 10 * n + d as usize;
            }

            tot += lut.add(n).read_unaligned();

            if ptr == end {
                break tot;
            }
        }
    }};
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    solve!(input, "/d11p1.lut", u32) as u32
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    solve!(input, "/d11p2.lut", u64)
}
