use crate::{aoc, load_input};
use anyhow::Result;
use itertools::{iproduct, Itertools};

aoc!(08);
pub fn day08_main() -> Result<()> {
    let input = load_input(08)?;
    assert_eq!(116, part1_solution(&input));
    // NOTE: The Part 2 solution should be UPOJFLBCEZ

    Ok(())
}

fn part1_solution(input: &str) -> usize {
    let mut grid = vec![[false; 6]; 50]; // NOTE: this is column major order
    for mut line in input.lines().map(|x| x.split_whitespace()) {
        match line.next().unwrap() {
            "rect" => {
                let coord_str = line.next().unwrap();
                let (width, height) = coord_str.split_at(coord_str.find('x').unwrap());
                let (width, height) = (
                    width.parse::<usize>().unwrap(),
                    height[1..].parse::<usize>().unwrap(),
                );
                for (x, y) in iproduct!(0..width, 0..height) {
                    grid[x][y] = true;
                }
            }
            "rotate" => match line.next().unwrap() {
                "row" => {
                    let (location, _, amount) = line.next_tuple().unwrap();
                    let location: usize = location.rsplit('=').next().unwrap().parse().unwrap();
                    let amount: isize = amount.parse().unwrap();

                    fn rotate_col(col: usize, amount: isize) -> usize {
                        (((col as isize - amount) + 50) % 50) as usize
                    }

                    let mut next_grid = grid.clone();
                    for (col, _) in grid.iter().enumerate() {
                        next_grid[col][location] = grid[rotate_col(col, amount)][location];
                    }

                    grid = next_grid;
                }
                "column" => {
                    let (location, _, amount) = line.next_tuple().unwrap();
                    let location: usize = location.rsplit('=').next().unwrap().parse().unwrap();
                    let amount: usize = amount.parse().unwrap();

                    grid[location].rotate_right(amount);
                }
                _ => eprintln!("Unrecognized rotate command: {:?}", line),
            },
            _ => eprintln!("Unrecognized instruction: {:?}", line),
        }

        // display the grid
    }

    for y in 0..6 {
        for x in 0..50 {
            print!("{}", if grid[x][y] { '█' } else { ' ' })
        }
        println!();
    }

    grid.iter()
        .map(|col| col.iter())
        .flatten()
        .filter(|x| **x == true)
        .count()
}
