use crate::{aoc, load_input};
use anyhow::Result;
use std::collections::HashMap;

aoc!(06);

pub fn day06_main() -> Result<()> {
    let input = load_input(06)?;
    assert_eq!("xhnqpqql", part1_solution(&input));
    assert_eq!("brhailro", part2_solution(&input));
    Ok(())
}

fn part1_solution(input: &str) -> String {
    let mut counters = vec![HashMap::new(); input.lines().next().unwrap().len()];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            counters[i]
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    counters
        .iter()
        .map(|counter| counter.iter().max_by_key(|(_k, v)| **v).unwrap().0)
        .collect::<String>()
}

fn part2_solution(input: &str) -> String {
    let mut counters = vec![HashMap::new(); input.lines().next().unwrap().len()];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            counters[i]
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    counters
        .iter()
        .map(|counter| counter.iter().min_by_key(|(_k, v)| **v).unwrap().0)
        .collect::<String>()
}
