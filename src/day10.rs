use crate::{aoc, load_input};
use anyhow::{anyhow, Result};
use itertools::Itertools;

aoc!(10);
pub fn day10_main() -> Result<()> {
    let input = load_input(10)?;

    let mut robots = vec![Robot::default(); 210];

    for line in input.lines() {
        let mut words = line.split_whitespace();
        if line.starts_with("value") {
            let _ = words.next();
            let value = words.next().unwrap().parse::<Value>()?;
            let bot = words.last().unwrap().parse::<BotIdx>()?;

            robots[bot].add_value(value);
        } else {
            let (subject, low, high) = words
                .tuple_windows()
                .filter_map(|(w1, w2)| {
                    if w1 == "bot" {
                        Some(Destination::Robot(w2.parse::<BotIdx>().ok()?))
                    } else if w1 == "output" {
                        Some(Destination::Output(w2.parse::<BotIdx>().ok()?))
                    } else {
                        None
                    }
                })
                .collect_tuple()
                .ok_or(anyhow!("Failed to parse: {:?}", line))?;

            if let Destination::Robot(subject) = subject {
                robots[subject].rule = Rule { low, high };
            }
        }
    }

    assert_eq!(116, simulate(&robots, Part::Part1));
    assert_eq!(23903, simulate(&robots, Part::Part2));

    Ok(())
}

type Value = usize;
type BotIdx = usize;

#[derive(Clone, Debug, PartialEq, Default)]
struct Rule {
    high: Destination,
    low: Destination,
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Destination {
    Robot(BotIdx),
    Output(usize),
}

impl Default for Destination {
    fn default() -> Self {
        Destination::Robot(0)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
struct Robot {
    values: [Option<Value>; 2],
    rule: Rule,
}

impl Robot {
    /// Adds a value to this robot. Returns true if both values are filled.
    fn add_value(&mut self, v: Value) {
        if let Some(_) = self.values[0] {
            // the first one is full, place `v` in teh second
            self.values[1] = Some(v);
        } else {
            self.values[0] = Some(v);
        }
    }

    fn is_ready(&self) -> bool {
        matches!(self.values, [Some(_), Some(_)])
    }

    fn max(&self) -> Value {
        self.values[0].max(self.values[1]).unwrap()
    }

    fn min(&self) -> Value {
        self.values[0].min(self.values[1]).unwrap()
    }

    fn clear(&mut self) {
        self.values = [None, None];
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Part {
    Part1,
    Part2,
}

fn simulate(robots: &[Robot], part: Part) -> usize {
    let mut robots = robots.to_vec();
    let mut i = 0;

    let mut outputs = vec![None; 21];

    loop {
        if robots[i].is_ready() {
            let high_val = robots[i].max();
            let high_dest = robots[i].rule.high;
            match high_dest {
                Destination::Robot(high_robot) => robots[high_robot].add_value(high_val),
                Destination::Output(output) => outputs[output] = Some(high_val),
            };

            let low_val = robots[i].min();
            let low_dest = robots[i].rule.low;
            match low_dest {
                Destination::Robot(low_robot) => robots[low_robot].add_value(low_val),
                Destination::Output(output) => outputs[output] = Some(low_val),
            };

            robots[i].clear();

            if part == Part::Part1 && high_val == 61 && low_val == 17 {
                return i;
            }

            if part == Part::Part2 {
                if let [Some(a), Some(b), Some(c)] = outputs[0..3] {
                    return a * b * c;
                }
            }
        }

        i += 1;
        i = i % robots.len();
    }
}
