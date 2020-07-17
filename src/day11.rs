use crate::aoc;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt;

aoc!(11);

pub fn day11_main() -> Result<()> {
    let input = {
        use Element::*;
        use Item::*;
        vec![
            vec![
                Generator(Polonium),
                Generator(Thulium),
                Microchip(Thulium),
                Generator(Promethium),
                Generator(Ruthenium),
                Microchip(Ruthenium),
                Generator(Cobalt),
                Microchip(Cobalt),
            ],
            vec![Microchip(Polonium), Microchip(Promethium)],
            vec![],
            vec![],
        ]
    };

    //let input = {
    //use Element::*;
    //use Item::*;
    //vec![
    //vec![Microchip(Hydrogen), Microchip(Lithium)],
    //vec![Generator(Hydrogen)],
    //vec![Generator(Lithium)],
    //vec![],
    //]
    //};

    dbg!(part1_solution(input.clone()));

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Element {
    Polonium,
    Thulium,
    Promethium,
    Ruthenium,
    Cobalt,
    // For the sample
    Hydrogen,
    Lithium,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Item {
    Generator(Element),
    Microchip(Element),
}

type Map = Vec<Floor>;
type Floor = Vec<Item>;

fn display_map(map: &Map) {
    for floor in map.iter().rev() {
        for item in floor.iter() {
            match item {
                Item::Generator(elem) => print!("{}G", &elem.to_string()[0..2]),
                Item::Microchip(elem) => print!("{}M", &elem.to_string()[0..2]),
            }
            print!(" ");
        }
        println!();
    }
    println!("--------------");
}

/// Checks if a certain arrangement of Generators and Microchips is safe.
/// Returns empty `Ok` variant if safe, else, returns Err varriant containing
/// the location of the first fried microchip.
fn check_safety(map: &[Floor]) -> Result<(), (usize, usize)> {
    for (floor, items) in map.iter().enumerate() {
        // first, check if this floor is being irradiated by a generator
        if items.iter().any(|i| matches!(i, Item::Generator(_))) {
            // then make sure that each microchip has its corresponding generator
            let failure = items.iter().enumerate().find_map(|(i, item)| {
                // if this isn't a microchip, it can't be fried at all
                if let Item::Microchip(elem) = item {
                    if !items.contains(&Item::Generator(*elem)) {
                        // this floor doesn't have the right generator
                        return Some(i);
                    }
                }
                None
            });

            if let Some(idx) = failure {
                return Err((floor, idx));
            }
        }
    }

    Ok(())
}

fn part1_solution(map: Map) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, map.clone()));

    let mut visited = HashSet::new();
    visited.insert((0, map.clone()));

    while let Some((steps, elevator, map)) = queue.pop_front() {
        //println!("Elevator: {}, Steps: {}", elevator, steps);
        //display_map(&map);
        // check if all the floors except the last are empty
        if map[..map.len() - 1].iter().all(|floor| floor.is_empty()) {
            return steps;
        }

        if elevator < map.len() - 1 {
            // first try moving up
            for (item1, item2) in map[elevator].iter().tuple_combinations() {
                // prefer a pair
                let mut next_map = map.clone();
                next_map[elevator].retain(|x| x != item1 && x != item2);

                next_map[elevator + 1].push(*item1);
                next_map[elevator + 1].push(*item2);

                let in_visited = visited.iter().any(|(e, m)| {
                    *e == elevator + 1 && {
                        m.iter().enumerate().all(|(floor, items)| {
                            items.len() == next_map[floor].len()
                                && items.iter().all(|i| next_map[floor].contains(i))
                        })
                    }
                });

                if !in_visited && matches!(check_safety(&next_map), Ok(())) {
                    visited.insert((elevator + 1, next_map.clone()));
                    queue.push_back((steps + 1, elevator + 1, next_map));
                }
            }

            for (idx, item) in map[elevator].iter().enumerate() {
                let mut next_map = map.clone();
                next_map[elevator].remove(idx);
                next_map[elevator + 1].push(*item);

                let in_visited = visited.iter().any(|(e, m)| {
                    *e == elevator + 1 && {
                        m.iter().enumerate().all(|(floor, items)| {
                            items.len() == next_map[floor].len()
                                && items.iter().all(|i| next_map[floor].contains(i))
                        })
                    }
                });

                if !in_visited && matches!(check_safety(&next_map), Ok(())) {
                    visited.insert((elevator + 1, next_map.clone()));
                    queue.push_back((steps + 1, elevator + 1, next_map));
                }
            }
        }

        if elevator > 0 {
            for (item1, item2) in map[elevator].iter().tuple_combinations() {
                // prefer a pair
                let mut next_map = map.clone();
                next_map[elevator].retain(|x| x != item1 && x != item2);

                next_map[elevator - 1].push(*item1);
                next_map[elevator - 1].push(*item2);

                let in_visited = visited.iter().any(|(e, m)| {
                    *e == elevator + 1 && {
                        m.iter().enumerate().all(|(floor, items)| {
                            items.len() == next_map[floor].len()
                                && items.iter().all(|i| next_map[floor].contains(i))
                        })
                    }
                });

                if !in_visited && matches!(check_safety(&next_map), Ok(())) {
                    visited.insert((elevator - 1, next_map.clone()));
                    queue.push_back((steps + 1, elevator - 1, next_map));
                }
            }

            for (idx, item) in map[elevator].iter().enumerate() {
                let mut next_map = map.clone();
                next_map[elevator].remove(idx);
                next_map[elevator - 1].push(*item);

                let in_visited = visited.iter().any(|(e, m)| {
                    *e == elevator + 1 && {
                        m.iter().enumerate().all(|(floor, items)| {
                            items.len() == next_map[floor].len()
                                && items.iter().all(|i| next_map[floor].contains(i))
                        })
                    }
                });

                if !in_visited && matches!(check_safety(&next_map), Ok(())) {
                    visited.insert((elevator - 1, next_map.clone()));
                    queue.push_back((steps + 1, elevator - 1, next_map));
                }
            }
        }
    }
    0
}
