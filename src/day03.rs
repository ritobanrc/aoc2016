use crate::{aoc, load_input};
use anyhow::Result;

aoc!(03);

pub fn day03_main() -> Result<()> {
    let input = load_input(03)?;

    assert_eq!(1050, part1_solution(&input)?);
    assert_eq!(1921, part2_solution(&input)?);

    Ok(())
}

pub fn part1_solution(input: &str) -> Result<usize> {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(input
        .iter()
        .filter(|triangle| {
            triangle[0] + triangle[1] > triangle[2]
                && triangle[0] + triangle[2] > triangle[1]
                && triangle[1] + triangle[2] > triangle[0]
        })
        .count())
}

pub fn part2_solution(input: &str) -> Result<usize> {
    // god this would be so much cleaner in python
    use itertools::Itertools;
    let input = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|lines| {
            // this is a 2D array of the 3x3 box
            let three_lines = lines
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            // now transpose the 3x3 matrix
            (0..3).map(move |i| [three_lines[0][i], three_lines[1][i], three_lines[2][i]])
        })
        .flatten()
        .collect::<Vec<_>>();

    Ok(input
        .iter()
        .filter(|triangle| {
            triangle[0] + triangle[1] > triangle[2]
                && triangle[0] + triangle[2] > triangle[1]
                && triangle[1] + triangle[2] > triangle[0]
        })
        .count())
}
