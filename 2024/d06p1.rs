// Original by: doge
const WIDTH: usize = 131;
const HEIGHT: usize = 130;
const SIZE: usize = WIDTH * HEIGHT;

#[inline(always)]
unsafe fn inner(s: &[u8]) -> u32 {
    let loc = memchr::memchr(b'^', s).unwrap_unchecked();
    let mut new = [1_u8; SIZE];
    let s_ptr = s.as_ptr();
    let new_ptr = new.as_mut_ptr();
    let slen = s.len();
    let mut total = 0;
    let mut loc = loc;

    macro_rules! process_cell {
        () => {{
            let cell_ptr = new_ptr.add(loc);
            total += *cell_ptr as u32;
            *cell_ptr = 0;
        }};
    }

    macro_rules! check_bounds {
        ($next_expr:expr, $check_expr:expr) => {{
            process_cell!();
            let next = $next_expr;
            if $check_expr(next) {
                return total;
            }
            let c = *s_ptr.add(next);
            if c == b'#' {
                break;
            }
            loc = next;
        }};
    }

    'outer: loop {
        // Up
        loop {
            check_bounds!(loc.wrapping_sub(WIDTH), |n| n >= slen);
        }
        // Right
        loop {
            check_bounds!(loc + 1, |n| *s_ptr.add(n) == b'\n');
        }
        // Down
        loop {
            check_bounds!(loc.wrapping_add(WIDTH), |n| n >= slen);
        }
        // Left
        loop {
            check_bounds!(loc.wrapping_sub(1), |n| *s_ptr.add(n) == b'\n');
        }
    }
}

#[inline(always)]
pub fn run(input: &[u8]) -> u32 {
    unsafe { inner(input) }
}

#[test]
fn d6p1() {
    assert_eq!(run(include_bytes!("./../input/day6.txt")), 5269);
}
