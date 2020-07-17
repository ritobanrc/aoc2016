use crate::aoc;
use anyhow::Result;
use bitflags::bitflags;
use itertools::{iproduct, Itertools};
use std::collections::{HashSet, VecDeque};
use std::fmt;

aoc!(11);

pub fn day11_main() -> Result<()> {
    let input = {
        use Elements as E;
        Map {
            floors: vec![
                Floor {
                    generators: E::Polonium | E::Thulium | E::Promethium | E::Ruthenium | E::Cobalt,
                    microchips: E::Thulium | E::Ruthenium | E::Cobalt,
                },
                Floor {
                    generators: E::empty(),
                    microchips: E::Polonium | E::Promethium,
                },
                Floor {
                    generators: E::empty(),
                    microchips: E::empty(),
                },
                Floor {
                    generators: E::empty(),
                    microchips: E::empty(),
                },
            ],
            elevator: 0,
        }
    };

    assert_eq!(true, input.check_safety());

    assert_eq!(47, part1_solution(input, Elements::MAIN_ALL));

    Ok(())
}

bitflags! {
    struct Elements: u8 {
        const Hydrogen   = 0b0000_0001;
        const Lithium    = 0b0000_0010;
        const Polonium   = 0b0000_0100;
        const Thulium    = 0b0000_1000;
        const Promethium = 0b0001_0000;
        const Ruthenium  = 0b0010_0000;
        const Cobalt     = 0b0100_0000;

        const MAIN_ALL = Elements::Polonium.bits | Elements::Thulium.bits | Elements::Promethium.bits | Elements::Ruthenium.bits | Elements::Cobalt.bits;
        const EXAMPLE_ALL = Elements::Hydrogen.bits | Elements::Lithium.bits;
    }
}

impl fmt::Display for Elements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Elements {
    fn split(self) -> impl Iterator<Item = Elements> + Clone {
        (0..8u8).filter_map(move |i| {
            if ((1 << i) & self.bits) != 0 {
                Elements::from_bits(1 << i)
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Map {
    floors: Vec<Floor>,
    elevator: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Floor {
    generators: Elements,
    microchips: Elements,
}

impl Map {
    /// Checks if a certain arrangement of Generators and Microchips is safe.
    fn check_safety(&self) -> bool {
        self.floors
            .iter()
            .all(|floor| floor.generators.is_empty() || floor.generators.contains(floor.microchips))
    }

    fn all_possible_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let items = &self.floors[self.elevator];

        // there are three possibilities for moving a pair
        // - 2 microchips
        // - 2 generators
        // - a pair -- it doesn't matter which pair, because the two pathways are
        // equivalent and will take the same number of steps
        //
        // alternatively, move 1 microchip or 1 generator

        // It doesn't matter which pair we choose, they are all equal
        if let Some(elem) = (items.microchips & items.generators).split().next() {
            moves.push(Move::Pair(elem))
        }

        for (m1, m2) in items.microchips.split().tuple_combinations() {
            moves.push(Move::TwoMicrochips(m1, m2));
        }

        for (g1, g2) in items.generators.split().tuple_combinations() {
            moves.push(Move::TwoGenerators(g1, g2));
        }

        moves.extend(items.microchips.split().map(Move::OneMicrochip));
        moves.extend(items.generators.split().map(Move::OneGenerator));

        moves
    }

    fn _display_map(&self) {
        for (i, floor) in self.floors.iter().enumerate().rev() {
            if i == self.elevator {
                print!("E -- ");
            }
            for item in floor.microchips.split() {
                print!("{}M ", &item.to_string()[0..2]);
            }
            for item in floor.generators.split() {
                print!("{}G ", &item.to_string()[0..2]);
            }
            println!();
        }
    }
}

#[derive(Clone, Debug)]
enum Move {
    TwoMicrochips(Elements, Elements),
    TwoGenerators(Elements, Elements),
    Pair(Elements),
    OneMicrochip(Elements),
    OneGenerator(Elements),
}

fn part1_solution(map: Map, all: Elements) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, map.clone()));

    let mut visited = HashSet::new();
    visited.insert(map.clone());

    while let Some((steps, map)) = queue.pop_front() {
        //println!("Elevator: {}, Steps: {}", elevator, steps);
        //display_map(&map);
        // check if all the floors except the last are empty
        let last_floor = &map.floors[map.floors.len() - 1];
        if last_floor.microchips == all && last_floor.generators == all {
            return steps;
        }

        let destinations = match map.elevator {
            0 => vec![1],
            x if x == map.floors.len() - 1 => vec![map.elevator - 1],
            _ => vec![map.elevator + 1, map.elevator - 1],
        };

        //map.display_map();
        //println!("{:?}", map.all_possible_moves());

        for (possibility, dest) in iproduct!(map.all_possible_moves(), destinations) {
            let mut next_map = map.clone();
            next_map.elevator = dest;

            match possibility {
                Move::TwoMicrochips(m1, m2) => {
                    next_map.floors[map.elevator].microchips.remove(m1);
                    next_map.floors[map.elevator].microchips.remove(m2);

                    next_map.floors[dest].microchips.insert(m1);
                    next_map.floors[dest].microchips.insert(m2);
                }
                Move::TwoGenerators(g1, g2) => {
                    next_map.floors[map.elevator].generators.remove(g1);
                    next_map.floors[map.elevator].generators.remove(g2);

                    next_map.floors[dest].generators.insert(g1);
                    next_map.floors[dest].generators.insert(g2);
                }
                Move::Pair(elem) => {
                    next_map.floors[map.elevator].microchips.remove(elem);
                    next_map.floors[map.elevator].generators.remove(elem);

                    next_map.floors[dest].microchips.insert(elem);
                    next_map.floors[dest].generators.insert(elem);
                }
                Move::OneMicrochip(elem) => {
                    next_map.floors[map.elevator].microchips.remove(elem);

                    next_map.floors[dest].microchips.insert(elem);
                }
                Move::OneGenerator(elem) => {
                    next_map.floors[map.elevator].generators.remove(elem);

                    next_map.floors[dest].generators.insert(elem);
                }
            }

            if !visited.contains(&next_map) && next_map.check_safety() {
                //println!("Move: {:?}, Steps: {:?}", possibility, steps + 1);
                //next_map.display_map();
                queue.push_back((steps + 1, next_map.clone()));
                visited.insert(next_map.clone());
            }
        }
        //break;
    }
    0
}

#[test]
fn day11_example() {
    let input = {
        use Elements as E;
        Map {
            floors: vec![
                Floor {
                    generators: E::empty(),
                    microchips: E::Hydrogen | E::Lithium,
                },
                Floor {
                    generators: E::Hydrogen,
                    microchips: E::empty(),
                },
                Floor {
                    generators: E::Lithium,
                    microchips: E::empty(),
                },
                Floor {
                    generators: E::empty(),
                    microchips: E::empty(),
                },
            ],
            elevator: 0,
        }
    };

    assert_eq!(11, part1_solution(input, Elements::EXAMPLE_ALL));
}
