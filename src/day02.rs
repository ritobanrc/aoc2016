use crate::{aoc, load_input};
use anyhow::{anyhow, Result};
use cgmath::{vec2, Vector2};

aoc!(02);
pub fn day02_main() -> Result<()> {
    let input = load_input(02)?;
    assert_eq!(76792, part1_solution(&input)?);
    assert_eq!("A7AC3", part2_solution(&input)?);
    Ok(())
}

fn part1_solution(input: &str) -> Result<u32> {
    let mut current_pos = vec2(0i8, 0i8); // 5 is (0, 0)
    let password = input
        .lines()
        .map(|line| {
            for c in line.chars() {
                match c {
                    'U' => current_pos.y = (current_pos.y - 1).max(-1),
                    'D' => current_pos.y = (current_pos.y + 1).min(1),
                    'R' => current_pos.x = (current_pos.x + 1).min(1),
                    'L' => current_pos.x = (current_pos.x - 1).max(-1),
                    c => return Err(anyhow!("Unrecognized char: {:?}", c)),
                }
            }
            let digit = (current_pos.y + 1) * 3 + current_pos.x + 2;
            Ok(digit)
        })
        .collect::<Result<Vec<_>>>()?;

    let output = {
        let mut output = 0u32;
        for digit in password.iter() {
            output *= 10;
            output += *digit as u32;
        }
        output
    };

    Ok(output)
}

fn part2_solution(input: &str) -> Result<String> {
    use std::collections::HashMap;
    let keypad = "
    1
  2 3 4
5 6 7 8 9
  A B C
    D    ";

    let keypad_map: HashMap<Vector2<usize>, &str> = keypad
        .trim_start()
        .lines()
        .map(|x| x.split_whitespace())
        .enumerate()
        .map(|(y, line)| {
            let buttons_in_line = line.clone().count();
            line.enumerate().map(move |(i, c)| {
                let x = i + (5 - buttons_in_line) / 2;
                // i'm too lazy to deal with overflow later, so i'm just adding 1 to all
                // the positions so it doesn't overflow
                (vec2(x + 1, y + 1), c)
            })
        })
        .flatten()
        .collect();

    let mut current_pos = *keypad_map
        .iter()
        .find(|(_pos, &val)| val == "5")
        .ok_or(anyhow!("Could not find 5 in {:?}", keypad_map))?
        .0;

    let password = input
        .lines()
        .map(|line| {
            for c in line.chars() {
                let next_pos = match c {
                    'U' => vec2(current_pos.x, current_pos.y - 1),
                    'D' => vec2(current_pos.x, current_pos.y + 1),
                    'R' => vec2(current_pos.x + 1, current_pos.y),
                    'L' => vec2(current_pos.x - 1, current_pos.y),
                    c => return Err(anyhow!("Unrecognized char: {:?}", c)),
                };
                if keypad_map.contains_key(&next_pos) {
                    // this is a valid position to move to
                    current_pos = next_pos;
                }
            }
            let digit = keypad_map[&current_pos];
            Ok(digit)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(password.join(""))
}

#[test]
fn day02_examples() {
    assert_eq!(
        1985,
        part1_solution(
            &"
ULL
RRDDD
LURDL
UUUUD"
                .trim()
        )
        .unwrap()
    );
}
