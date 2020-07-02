use crate::{aoc, load_input};
use anyhow::{anyhow, Result};
use cgmath::vec2;
use std::collections::HashSet;

aoc!(01);
pub fn day01_main() -> Result<()> {
    let input = load_input(01)?;
    let instructions = input.trim().split(", ").collect::<Vec<_>>();

    assert_eq!(307, part1_solution(&instructions)?);
    assert_eq!(165, part2_solution(&instructions)?);

    Ok(())
}

fn part1_solution(instructions: &[&str]) -> Result<i32> {
    let mut current_dir = vec2(0i32, 1); // north is positive Y
    let mut current_loc = vec2(0i32, 0); // start at 0, 0

    for ins in instructions {
        match ins.split_at(1) {
            ("R", x) => {
                // turn direction right
                current_dir = match current_dir.into() {
                    (1, 0) => vec2(0, 1),   // North => East
                    (-1, 0) => vec2(0, -1), // South => West
                    (0, 1) => vec2(-1, 0),  // East => South
                    (0, -1) => vec2(1, 0),  // West => North
                    d => return Err(anyhow!("Impossible current_dir: {:?}", d)),
                };
                let x = x.parse::<i32>()?;
                current_loc += x * current_dir;
            }
            ("L", x) => {
                // turn direction right
                current_dir = match current_dir.into() {
                    (1, 0) => vec2(0, -1),  // North => West
                    (-1, 0) => vec2(0, 1),  // South => East
                    (0, 1) => vec2(1, 0),   // East => North
                    (0, -1) => vec2(-1, 0), // West => South
                    d => return Err(anyhow!("Impossible current_dir: {:?}", d)),
                };
                let x = x.parse::<i32>()?;
                current_loc += x * current_dir;
            }
            e => return Err(anyhow!("Unrecognized instruction: {:?}", e)),
        }
    }

    Ok(current_loc.x.abs() + current_loc.y.abs())
}

fn part2_solution(instructions: &[&str]) -> Result<i32> {
    let mut current_dir = vec2(0i32, 1); // north is positive Y
    let mut current_loc = vec2(0i32, 0); // start at 0, 0

    let mut set = HashSet::new();

    for ins in instructions {
        let x = match ins.split_at(1) {
            ("R", x) => {
                // turn direction right
                current_dir = match current_dir.into() {
                    (1, 0) => vec2(0, 1),   // North => East
                    (-1, 0) => vec2(0, -1), // South => West
                    (0, 1) => vec2(-1, 0),  // East => South
                    (0, -1) => vec2(1, 0),  // West => North
                    d => return Err(anyhow!("Impossible current_dir: {:?}", d)),
                };
                x.parse::<i32>()?
            }
            ("L", x) => {
                // turn direction right
                current_dir = match current_dir.into() {
                    (1, 0) => vec2(0, -1),  // North => West
                    (-1, 0) => vec2(0, 1),  // South => East
                    (0, 1) => vec2(1, 0),   // East => North
                    (0, -1) => vec2(-1, 0), // West => South
                    d => return Err(anyhow!("Impossible current_dir: {:?}", d)),
                };
                x.parse::<i32>()?
            }
            e => return Err(anyhow!("Unrecognized instruction: {:?}", e)),
        };

        // We have to add _every_ visited square to the set, not just the ones where we
        // changed direction.
        for _ in 0..x {
            current_loc += current_dir;
            if set.contains(&current_loc) {
                return Ok(current_loc.x.abs() + current_loc.y.abs());
            } else {
                set.insert(current_loc);
            }
        }
    }

    Err(anyhow!(
        "Did not visit any location twice. Current Loc: {:?}, Current Dir: {:?}, Visited: {:?}",
        current_loc,
        current_dir,
        set
    ))
}

#[test]
fn day01_examples() {
    dbg!(part2_solution(&["R8", "R4", "R4", "R8"]).unwrap());
}
