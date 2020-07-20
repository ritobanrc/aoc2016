use crate::aoc;
use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;

aoc!(14);

pub fn day14_main() -> Result<()> {
    const INPUT: &str = "ahsbgdzn";

    assert_eq!(23890, solutions(INPUT, Part::Part1));
    assert_eq!(22696, solutions(INPUT, Part::Part2));
    Ok(())
}

fn part1_hash(suffix: usize, input: &str) -> String {
    let digest = md5::compute(input.to_string() + &suffix.to_string());
    format!("{:x}", digest)
}

fn part2_hash(suffix: usize, input: &str) -> String {
    let mut current = input.to_string() + &suffix.to_string();
    for _ in 0..2017 {
        let digest = md5::compute(current);
        current = format!("{:x}", digest);
    }
    current
}

enum Part {
    Part1,
    Part2,
}

fn solutions(input: &str, part: Part) -> usize {
    let mut storage: VecDeque<(usize, String)> = VecDeque::with_capacity(1001);
    let mut num_found = 0;

    for suffix in 0.. {
        if storage.len() > 1000 {
            let (idx, first) = storage.pop_front().unwrap();
            for (a, b, c) in first.bytes().tuple_windows() {
                if a == b && a == c {
                    // Great, this has 3 in a row
                    // Now check is one of the next 1000 in the storage has 5 in a row
                    let found_in_next_thousand = storage.iter().any(|(_, s)| {
                        let s = s.as_bytes();
                        for (i, &c) in s.iter().enumerate() {
                            if i < s.len() - 4
                                && c == a
                                && c == s[i + 1]
                                && c == s[i + 2]
                                && c == s[i + 3]
                                && c == s[i + 4]
                            {
                                return true;
                            }
                        }
                        false
                    });

                    if found_in_next_thousand {
                        num_found += 1;
                        if num_found == 64 {
                            return idx;
                        }
                    }
                    break;
                }
            }
        }

        let hash = match part {
            Part::Part1 => part1_hash(suffix, input),
            Part::Part2 => part2_hash(suffix, input),
        };

        storage.push_back((suffix, hash));
    }
    0
}
