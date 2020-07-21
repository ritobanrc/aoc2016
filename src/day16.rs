use crate::aoc;
use anyhow::Result;
use itertools::Itertools;

aoc!(16);
pub fn day16_main() -> Result<()> {
    let input = "10111100110001111"
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    assert_eq!("11100110111101110", dbg!(solution(&input, 272)));
    assert_eq!("10001101010000101", dbg!(solution(&input, 35651584)));

    Ok(())
}

#[test]
fn day16_example() {
    let input = "10000"
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    dbg!(solution(&input, 20));
}

fn apply_dragon(input: &[bool]) -> Vec<bool> {
    let mut output = input.to_owned();
    output.push(false);
    output.extend(input.iter().rev().map(std::ops::Not::not));
    output
}

fn apply_checksum(input: &[bool]) -> impl Iterator<Item = bool> + '_ {
    input.iter().tuples().map(|(a, b)| a == b)
}

fn display(input: &[bool]) -> String {
    input
        .iter()
        .map(|b| match b {
            true => '1',
            false => '0',
        })
        .collect()
}

fn solution(input: &[bool], size: usize) -> String {
    let mut output = input.to_owned();
    while output.len() < size {
        output = apply_dragon(&output);
    }

    output.truncate(size);

    while output.len() % 2 == 0 {
        output = apply_checksum(&output).collect();
    }

    display(&output)
}
