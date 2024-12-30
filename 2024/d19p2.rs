// Originally by: __main_character__
use rayon::prelude::*;
use std::error::Error;
use std::fmt::Display;

type NodeInt = u16;

// My input leads to 795 trie nodes
const MAX_NODES: usize = 896;

pub fn run(input: &str) -> impl Display {
    let (a, b) = parse_input(input).unwrap();
    part_two(&a, &b)
}

pub fn parse_input(input: &str) -> Result<(Vec<&str>, Vec<&str>), Box<dyn Error>> {
    let mut input = input;
    let mut bytes = input.as_bytes();

    let mut patterns = Vec::with_capacity(460);
    'all: while !bytes.is_empty() {
        for idx in 0..bytes.len() {
            match bytes[idx] {
                b',' => {
                    patterns.push(&input[..idx]);
                    input = &input[idx + 2..];
                    bytes = &bytes[idx + 2..];
                    break;
                }

                b'\n' => {
                    patterns.push(&input[..idx]);
                    input = &input[idx + 2..];
                    bytes = &bytes[idx + 2..];
                    break 'all;
                }
                _ => {}
            }
        }
    }

    let mut towels = Vec::with_capacity(410);
    while !bytes.is_empty() {
        let p = memchr::memchr(b'\n', bytes).unwrap();
        towels.push(&input[..p]);

        input = &input[p + 1..];
        bytes = &bytes[p + 1..];
    }

    Ok((patterns, towels))
}
//
// pub fn part_one(patterns: &[&str], lines: &[&str]) -> usize {
//     let mut trie = [([0; 5], false); MAX_NODES];
//     let mut node_ptr = 0;
//
//     patterns
//         .into_iter()
//         .for_each(|&pattern| insert(&mut trie, &mut node_ptr, pattern.as_bytes()));
//
//     lines
//         .chunks(25)
//         .par_bridge()
//         .map(|chunk| {
//             chunk
//                 .iter()
//                 .filter(|t| contains(&trie, t.as_bytes(), &mut 0))
//                 .count()
//         })
//         .sum()
// }
//
// fn contains(trie: &[([NodeInt; 5], bool)], word: &[u8], failed: &mut u64) -> bool {
//     let mut node = 0;
//
//     for (idx, &ch) in word.iter().enumerate() {
//         let key = index(ch);
//
//         let next = trie[node].0[key] as usize;
//         if next == 0 {
//             return false;
//         }
//
//         node = next;
//         if trie[node].1 {
//             let remaining = &word[idx + 1..];
//             if remaining.is_empty() {
//                 return true;
//             }
//
//             let bit = remaining.len() - 1;
//             if *failed & (1 << bit) != 0 {
//                 continue;
//             }
//
//             if contains(trie, remaining, failed) {
//                 return true;
//             }
//
//             *failed |= 1 << bit;
//         }
//     }
//
//     false
// }

pub fn part_two(patterns: &[&str], lines: &[&str]) -> u64 {
    let mut trie = [[0; 6]; MAX_NODES];
    let mut node_ptr = 0;

    patterns
        .into_iter()
        .for_each(|&pattern| insert(&mut trie, &mut node_ptr, pattern.as_bytes()));

    lines
        .chunks(25)
        .par_bridge()
        .map(|l| {
            let mut sum = 0;
            for &x in l {
                let mut counts = [-1; 64];
                sum += contains_ways(&trie, x.as_bytes(), &mut counts)
            }

            sum
        })
        .sum::<i64>() as u64
}

fn contains_ways(trie: &[[NodeInt; 6]], word: &[u8], cache: &mut [i64]) -> i64 {
    let mut node = 0;
    let mut ways = 0;

    for (idx, &ch) in word.iter().enumerate() {
        let key = index(ch);

        let next = trie[node][key] as usize;
        if next == 0 {
            break;
        }

        node = next;
        if trie[node][5] != 0 {
            let remaining = &word[idx + 1..];
            if remaining.is_empty() {
                ways += 1;
                break;
            }

            ways += if cache[remaining.len()] >= 0 {
                cache[remaining.len()]
            } else {
                contains_ways(trie, remaining, cache)
            };
        }
    }

    cache[word.len()] = ways;
    ways
}

fn insert(trie: &mut [[NodeInt; 6]], last_node: &mut NodeInt, word: &[u8]) {
    let mut node = 0;

    for &ch in word {
        let key = index(ch);

        if trie[node][key] == 0 {
            *last_node += 1;
            trie[node][key] = *last_node;
        }

        node = trie[node][key] as usize;
    }

    trie[node][5] = 1;
}

const fn index(ch: u8) -> usize {
    ((((ch as usize & 31) * 7) >> 4) + 1) % 8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::PART_2_ANSWER;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_two() {
"inputs/input.txt"");
        let (patterns, lines) = parse_input(&input).unwrap();

        let answer = part_two(&patterns, &lines);
        assert_eq!(PART_2_ANSWER, answer);
    }
}
