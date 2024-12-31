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

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    type Ty = u64;

    let mut tot = 0u64;

    let lut = LUT.as_ptr().cast::<Ty>();

    #[rustfmt::skip]
    core::arch::asm!(
    "2:",
        "movzx   {n}, byte ptr [{ptr}]",
        "movzx   {d}, byte ptr [{ptr} + 1]",
        "sub     {n}, {b0}",
        "sub     {d}, {b0}",
        "add     {ptr}, 2",
        "cmp     {d}, 9",
        "ja      4f",
        
        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",
        
        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
    "4:",
        "add     {tot}, qword ptr [{lut} + {s}*{n}]",
        "cmp     {ptr}, {end}",
        "jne     2b",
        ptr = in(reg) input.as_ptr(),
        end = in(reg) input.as_ptr().add(input.len()),
        lut = in(reg) lut,
        tot = inout(reg) tot,
        n = out(reg) _,
        d = out(reg) _,
        s = const std::mem::size_of::<Ty>(),
        b0 = const b'0' as u64
    );

    tot as u32
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    type Ty = u64;

    let mut tot = 0;

    let lut = LUT.as_ptr().cast::<Ty>();

    #[rustfmt::skip]
    core::arch::asm!(
    "2:",
        "movzx   {n}, byte ptr [{ptr}]",
        "movzx   {d}, byte ptr [{ptr} + 1]",
        "sub     {n}, {b0}",
        "sub     {d}, {b0}",
        "add     {ptr}, 2",
        "cmp     {d}, 9",
        "ja      4f",
        
        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",

        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 9",
        "ja      4f",
        
        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
    "4:",
        "add     {tot}, qword ptr [{lut} + {s}*{n}]",
        "cmp     {ptr}, {end}",
        "jne     2b",
        ptr = in(reg) input.as_ptr(),
        end = in(reg) input.as_ptr().add(input.len()),
        lut = in(reg) lut,
        tot = inout(reg) tot,
        n = out(reg) _,
        d = out(reg) _,
        s = const std::mem::size_of::<Ty>(),
        b0 = const b'0' as u64
    );

    tot
}
