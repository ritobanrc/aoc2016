pub mod day01;
pub mod day02;
pub mod day03;

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
