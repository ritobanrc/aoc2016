use crate::aoc;
use anyhow::Result;
use cgmath::{vec2, Vector2};
use std::collections::VecDeque;

aoc!(17);
pub fn day17_main() -> Result<()> {
    const INPUT: &str = "ioramepc";

    assert_eq!("RDDRULDDRR", dbg!(part1_solution(INPUT)));
    assert_eq!(766, dbg!(part2_solution(INPUT)));

    Ok(())
}

const DIRECTIONS: [(Vector2<isize>, char); 4] = [
    (vec2(0, -1), 'U'),
    (vec2(0, 1), 'D'),
    (vec2(-1, 0), 'L'),
    (vec2(1, 0), 'R'),
];

fn part1_solution(password: &str) -> String {
    let start = vec2(0, 0);
    let mut queue = VecDeque::new();
    queue.push_back((start, String::new()));

    while let Some((current, path)) = queue.pop_front() {
        if current == vec2(3, 3) {
            return path;
        }

        let digest = md5::compute(format!("{}{}", password, path));
        format!("{:x}", digest)[0..4]
            .chars()
            .zip(DIRECTIONS.iter())
            .filter(|(c, _dir)| *c > 'a')
            .for_each(|(_c, (dir, dir_char))| {
                let next = current + dir;
                if next.x < 0 || next.y < 0 || next.x > 3 || next.y > 3 {
                    return;
                }
                let mut next_path = path.clone();
                next_path.push(*dir_char);
                queue.push_back((next, next_path));
            });
    }

    String::new()
}

fn part2_solution(password: &str) -> usize {
    let start = vec2(0, 0);
    let mut stack = Vec::new();
    stack.push((start, String::new()));

    let mut longest_len = 0;

    while let Some((current, path)) = stack.pop() {
        if current == vec2(3, 3) {
            if path.len() > longest_len {
                assert!(test_path(&path));
                longest_len = path.len();
            }
            continue;
        }

        let digest = md5::compute(format!("{}{}", password, path));
        format!("{:x}", digest)[0..4]
            .chars()
            .zip(DIRECTIONS.iter())
            .filter(|(c, _dir)| *c > 'a')
            .for_each(|(_c, (dir, dir_char))| {
                let next = current + dir;
                if next.x < 0 || next.y < 0 || next.x > 3 || next.y > 3 {
                    return;
                }
                let mut next_path = path.clone();
                next_path.push(*dir_char);
                stack.push((next, next_path));
            });
    }

    longest_len
}

fn test_path(path: &str) -> bool {
    let mut current = vec2(0, 0);
    for (i, c) in path.chars().enumerate() {
        match c {
            'U' => current.y = current.y - 1,
            'D' => current.y = current.y + 1,
            'R' => current.x = current.x + 1,
            'L' => current.x = current.x - 1,
            _ => unreachable!(),
        }
        if current.x < 0 || current.y < 0 || current.x > 3 || current.y > 3 {
            println!("We went off the board somehow!");
            return false;
        }

        if current == vec2(3, 3) && i != path.len() - 1 {
            println!("Arrived at (3, 3) too early: {:?}", i);
            return false;
        }
    }
    current == vec2(3, 3)
}
