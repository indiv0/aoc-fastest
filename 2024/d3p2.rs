// Original by: ameo
#![feature(array_chunks, array_windows, duration_constructors, portable_simd)]

use std::{
    fmt::Display,
    simd::{cmp::SimdPartialEq, u8x16, u8x64},
};

#[cfg(feature = "local")]
pub const INPUT: &'static [u8] = include_bytes!("../inputs/day3.txt");

fn parse_digit(c: u8) -> usize {
    (c - 48) as usize
}

#[inline(always)]
fn add_num(digits: &[usize]) -> usize {
    match digits.len() {
        0 => unreachable!(),
        1 => unsafe { *digits.get_unchecked(0) },
        2 => 10 * unsafe { *digits.get_unchecked(0) } + unsafe { *digits.get_unchecked(1) },
        3 => {
            100 * unsafe { *digits.get_unchecked(0) }
                + 10 * unsafe { *digits.get_unchecked(1) }
                + unsafe { *digits.get_unchecked(2) }
        }
        _ => unreachable!(),
    }
}

const MUL: [u8; 4] = ['m' as u8, 'u' as u8, 'l' as u8, '(' as u8];
const DONT: [u8; 7] = [
    'd' as u8, 'o' as u8, 'n' as u8, '\'' as u8, 't' as u8, '(' as u8, ')' as u8,
];
const DO: [u8; 4] = ['d' as u8, 'o' as u8, '(' as u8, ')' as u8];
// shortest valid mul is `mul(1,1)` so 8 chars
const MIN_VALID_MUL_LEN: usize = 8;

pub fn parse_and_compute<const ENABLE_DO_STATE: bool>(input: &[u8]) -> usize {
    let mut sum = 0usize;
    let mut do_state = true;
    let mut char_ix = 0usize;

    'outer: loop {
        if char_ix >= input.len() - MIN_VALID_MUL_LEN {
            return sum;
        }

        // For part 2, when the "do" mode is set to "don't", we only care about finding `d` characters.
        //
        // Since d's are so much sparser in the inputs than m's, there's a decent chance it will be
        // closer to 64 chars ahead than 16, and the overhead of reading further tends to be worth it.
        if ENABLE_DO_STATE && !do_state && char_ix < input.len() - (64 + 1) {
            let vector = u8x64::from_slice(&input[char_ix..char_ix + 64]);

            let mask = vector.simd_eq(u8x64::splat('d' as u8));
            let hit_ix = match mask.first_set() {
                Some(hit_ix) => hit_ix,
                None => {
                    // no hit in the entire window; skip it completely and move on to the next
                    char_ix += 64;
                    continue;
                }
            };

            char_ix += hit_ix;
        }
        // Try to find the first relavant start character in the input by checking 16 at a time and then
        // selecting the index of the first match
        else if char_ix < input.len() - (16 + 1) {
            let vector = u8x16::from_slice(&input[char_ix..char_ix + 16]);

            let combined_mask = if ENABLE_DO_STATE {
                let d_mask = vector.simd_eq(u8x16::splat('d' as u8));
                // If we're keeping track of do/don't state and the do flag is not set, we can avoid
                // checking for `m` characters entirely and just scan for the next `d`.
                if !do_state {
                    d_mask
                } else {
                    let m_mask = vector.simd_eq(u8x16::splat('m' as u8));
                    m_mask | d_mask
                }
            } else {
                vector.simd_eq(u8x16::splat('m' as u8))
            };
            let hit_ix = match combined_mask.first_set() {
                Some(hit_ix) => hit_ix,
                None => {
                    // no hit in the entire window; skip it completely and move on to the next
                    char_ix += 16;
                    continue;
                }
            };

            char_ix += hit_ix;
        }
        // We use char-by-char checks for the remainder of the window
        else {
            let mut c = unsafe { *input.get_unchecked(char_ix) };
            while c != 'm' as u8 && (!ENABLE_DO_STATE || c != 'd' as u8) {
                char_ix += 1;
                if char_ix >= input.len() {
                    return sum;
                }
                c = unsafe { *input.get_unchecked(char_ix) };
            }
        }

        if char_ix >= input.len() - MIN_VALID_MUL_LEN {
            return sum;
        }

        // don't bother parsing out this mul if the do flag is not set
        if (!ENABLE_DO_STATE || do_state) && input.get(char_ix..char_ix + MUL.len()) == Some(&MUL) {
            char_ix += MUL.len();

            // at this point, `char_ix` is pointing to the next character after `mul(`.

            // parse out the rest of the `mul(___,___)` call.
            //
            // This code makes some assumptions which are supported by the structure of the input text:
            //  * exactly two arguments; any other arg count is invalid + skipped
            //  * arguments can have at between 1 and 3 digits
            //  * arguments are positive integers

            let first_num;
            let second_num;

            let mut d0;
            let mut d1;
            let mut d2;

            // first char after `mul(` must be a digit
            let mut c = unsafe { *input.get_unchecked(char_ix) };
            char_ix += 1;
            if c >= '0' as u8 && c <= '9' as u8 {
                d0 = parse_digit(c);
            } else {
                continue;
            }

            // next char `mul(1_` can be either digit or comma
            c = unsafe { *input.get_unchecked(char_ix) };
            char_ix += 1;

            if c >= '0' as u8 && c <= '9' as u8 {
                d1 = parse_digit(c);

                c = unsafe { *input.get_unchecked(char_ix) };
                char_ix += 1;

                // next char `mul(12_` can also be either digit or comma
                if c >= '0' as u8 && c <= '9' as u8 {
                    d2 = parse_digit(c);

                    c = unsafe { *input.get_unchecked(char_ix) };
                    char_ix += 1;

                    // next char `mul(123_` MUST be a comma if this mul is valid
                    if c != ',' as u8 {
                        continue 'outer;
                    }

                    first_num = add_num(&[d0, d1, d2]);
                } else if c == ',' as u8 {
                    first_num = add_num(&[d0, d1]);
                } else {
                    continue 'outer;
                }
            } else if c == ',' as u8 {
                first_num = d0;
            } else {
                continue 'outer;
            }

            c = unsafe { *input.get_unchecked(char_ix) };
            char_ix += 1;

            // at this point, we've successfully parsed a valid first argument number followed by a comma.
            //
            // we now have to parse out a valid second argument followed by a closing parenthesis.

            // next character `mul(123,_` must be a digit
            if c >= '0' as u8 && c <= '9' as u8 {
                d0 = parse_digit(c);
            } else {
                continue;
            }

            // finish parsing second arg.  Assuming that args have at most 3 chars, so take at most two
            // more digits followed by a `)`

            c = unsafe { *input.get_unchecked(char_ix) };
            char_ix += 1;

            // next character `mul(123,1_` can be either digit or `)`
            if c >= '0' as u8 && c <= '9' as u8 {
                d1 = parse_digit(c);

                c = unsafe { *input.get_unchecked(char_ix) };
                char_ix += 1;

                // next char `mul(123,12_` can also be either digit or `)`
                if c >= '0' as u8 && c <= '9' as u8 {
                    d2 = parse_digit(c);

                    c = unsafe { *input.get_unchecked(char_ix) };
                    char_ix += 1;

                    // next char `mul(123,123_` MUST be a `)` if this mul is valid
                    if c != ')' as u8 {
                        continue 'outer;
                    }

                    second_num = add_num(&[d0, d1, d2]);
                } else if c == ')' as u8 {
                    second_num = add_num(&[d0, d1]);
                } else {
                    continue 'outer;
                }
            } else if c == ')' as u8 {
                second_num = d0;
            } else {
                continue 'outer;
            }

            sum += first_num * second_num;
        } else if ENABLE_DO_STATE
            && do_state
            && input.get(char_ix..char_ix + DONT.len()) == Some(&DONT)
        {
            do_state = false;
            char_ix += DONT.len();
        } else if ENABLE_DO_STATE
            && !do_state
            && input.get(char_ix..char_ix + DO.len()) == Some(&DO)
        {
            do_state = true;
            char_ix += DO.len();
        } else {
            char_ix += 1;
        }
    }
}

#[cfg(feature = "local")]
pub fn solve() {
    let out = parse_and_compute::<false>(INPUT);
    println!("Part 1: {out}");

    let out = parse_and_compute::<true>(INPUT);
    println!("Part 2: {out}");
}

pub fn run(input: &[u8]) -> impl Display {
    parse_and_compute::<true>(input)
}
