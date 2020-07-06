use crate::{aoc, load_input};
use anyhow::Result;
use itertools::Itertools;

aoc!(07);
pub fn day07_main() -> Result<()> {
    let input = load_input(07)?;
    assert_eq!(105, part1_solution(&input));
    assert_eq!(258, part2_solution(&input));

    Ok(())
}

fn part1_solution(input: &str) -> usize {
    fn supports_tls(ip_addr: &str) -> bool {
        let mut inside_brackets = false;
        let mut valid_abba = false;
        for (a, b, c, d) in ip_addr.chars().tuple_windows() {
            if a == '[' {
                inside_brackets = true;
            }
            if d == ']' {
                inside_brackets = false;
            }

            if a == d && b == c && a != b {
                // this is a valid ABBA
                if inside_brackets {
                    return false; // we are NEVER allowed to have an ABBA inside brackets
                } else {
                    valid_abba = true; // we have to keep going to make sure there are no valid abbas inside brackets later on
                }
            }
        }

        valid_abba
    }

    input.lines().filter(|x| supports_tls(x)).count()
}

fn part2_solution(input: &str) -> usize {
    fn supports_ssl(line: &str) -> bool {
        let bracketed = line
            .match_indices('[')
            .zip(line.match_indices(']'))
            .map(|((open_idx, _open), (close_idx, _close))| &line[open_idx + 1..close_idx])
            .collect::<Vec<_>>();
        let mut inside_brackets = false;
        for (a, b, c) in line.chars().tuple_windows() {
            if a == '[' {
                inside_brackets = true;
            }
            if c == ']' {
                inside_brackets = false;
            }

            if !inside_brackets && a == c && a != b {
                let bab = [b, a, b].iter().collect::<String>();
                if bracketed.iter().any(|s| s.contains(&bab)) {
                    return true;
                }
            }
        }

        false
    }

    input.lines().filter(|x| supports_ssl(x)).count()
}
