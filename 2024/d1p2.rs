// Original by: doge
const N: usize = 1000;

#[inline(always)]
fn parse_5b(s: &[u8]) -> u32 {
    // Optimize by unrolling the loop and using direct subtraction to convert ASCII digits to numbers
    unsafe {
        let s0 = *s.get_unchecked(0) as u32;
        let s1 = *s.get_unchecked(1) as u32;
        let s2 = *s.get_unchecked(2) as u32;
        let s3 = *s.get_unchecked(3) as u32;
        let s4 = *s.get_unchecked(4) as u32;

        (s0 * 10000 + s1 * 1000 + s2 * 100 + s3 * 10 + s4) - 533328
    }
}

pub fn run(s: &[u8]) -> i64 {
    let mut assoc = [0; 2048];

    for i in 0..N {
        let right = parse_5b(&s[i * 14 + 8..]);
        let mut h = right & 2047;
        loop {
            let entry = &mut assoc[h as usize];
            if *entry == 0 {
                *entry = right | 1 << 20;
                break;
            }
            if (*entry & 0xfffff) == right {
                *entry += 1 << 20;
                break;
            }
            h = (h + 1) & 2047;
        }
    }

    let mut answer = 0;

    for i in 0..N {
        let left = parse_5b(&s[i * 14..]);
        let mut h = left & 2047;
        loop {
            let entry = assoc[h as usize];
            if entry == 0 {
                break;
            }
            if (entry & 0xfffff) == left {
                answer += left * (entry >> 20);
            }
            h = (h + 1) & 2047;
        }
    }
    answer as i64
}

#[test]
fn d1p2() {
    assert_eq!(run(include_bytes!("./../input/day1.txt")), 22962826);
}

