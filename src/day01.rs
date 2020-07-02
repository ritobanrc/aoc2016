use crate::{aoc, load_input};
use anyhow::Result;

aoc!(01);
pub fn day01_main() -> Result<()> {
    let input = load_input(01)?;
    let ins = input.split(", ").collect::<Vec<_>>();
    println!("{:?}", ins);

    Ok(())
}
