use crate::aoc;
use itertools::Itertools;

aoc!(18);

pub fn day18_main() -> anyhow::Result<()> {
    const INPUT: &str = r"^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^";

    let input: Vec<_> = INPUT
        .chars()
        .map(|c| match c {
            '^' => Tile::Trap,
            '.' => Tile::Safe,
            _ => unreachable!(),
        })
        .collect();

    assert_eq!(1974, dbg!(solutions(&input, 40)));
    assert_eq!(19991126, dbg!(solutions(&input, 400_000)));

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

fn next_row(row: &[Tile]) -> Vec<Tile> {
    let mut next = Vec::new();

    next.push(next_tile(Tile::Safe, row[0], row[1]));

    next.extend(
        row.iter()
            .tuple_windows()
            .map(|(&left, &center, &right)| next_tile(left, center, right)),
    );

    next.push(next_tile(
        row[row.len() - 2],
        row[row.len() - 1],
        Tile::Safe,
    ));

    assert_eq!(row.len(), next.len());

    next
}

fn next_tile(left: Tile, center: Tile, right: Tile) -> Tile {
    use Tile::*;
    match (left, center, right) {
        (Trap, Trap, Safe) => Trap,
        (Safe, Trap, Trap) => Trap,
        (Trap, Safe, Safe) => Trap,
        (Safe, Safe, Trap) => Trap,
        _ => Safe,
    }
}

fn count_safe(row: &[Tile]) -> usize {
    row.iter().filter(|x| **x == Tile::Safe).count()
}

fn solutions(input: &[Tile], num_rows: usize) -> usize {
    (0..num_rows - 1)
        .fold((input.to_owned(), count_safe(input)), |(last, safe), _| {
            let next = next_row(&last);
            let next_safe = count_safe(&next);
            (next, safe + next_safe)
        })
        .1
}
