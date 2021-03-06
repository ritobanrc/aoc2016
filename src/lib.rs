pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
#[allow(non_upper_case_globals)]
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;

use std::io;

pub fn load_input(day: usize) -> io::Result<String> {
    use std::fs::read_to_string;
    use std::path::PathBuf;
    let path: PathBuf = ["input", &format!("day{:02}.txt", day)].iter().collect();
    read_to_string(path)
}

#[macro_export]
macro_rules! aoc {
    ($day: expr) => {
        paste::item! {
            #[test]
            fn [<day $day _test>]() -> anyhow::Result<()> {
                [<day $day _main>]()
            }
        }
    };
}
