use crate::aoc;
use anyhow::Result;
use cgmath::{vec2, Vector2};
use std::collections::{HashSet, VecDeque};

aoc!(13);

pub fn day13_main() -> Result<()> {
    assert_eq!(82, solutions(Some(vec2(31, 39))));
    assert_eq!(138, solutions(None));

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TileType {
    Open,
    Wall,
}

fn tile_type(c: Vector2<isize>) -> TileType {
    static INPUT: isize = 1362;
    // x*x + 3*x + 2*x*y + y + y*y.
    let a = c.x * c.x + 3 * c.x + 2 * c.x * c.y + c.y + c.y * c.y;

    if (a + INPUT).count_ones() % 2 == 0 {
        TileType::Open
    } else {
        TileType::Wall
    }
}

fn solutions(target: Option<Vector2<usize>>) -> usize {
    let start = vec2(1, 1);
    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut discovered = HashSet::new();
    discovered.insert(start);

    while let Some((steps, current)) = queue.pop_front() {
        if let Some(target) = target {
            if current == target {
                return steps;
            }
        } else {
            if steps == 50 {
                return discovered.len();
            }
        }

        for offset in [
            Vector2::<isize>::unit_x(),
            Vector2::unit_y(),
            -Vector2::unit_x(),
            -Vector2::unit_y(),
        ]
        .iter()
        {
            let neighbor: Vector2<isize> = offset + current.cast().unwrap();
            if neighbor.x < 0 || neighbor.y < 0 || tile_type(neighbor) != TileType::Open {
                continue;
            }
            let next = neighbor.cast().unwrap();
            if !discovered.contains(&next) {
                discovered.insert(next);
                queue.push_back((steps + 1, next));
            }
        }
    }

    0
}
