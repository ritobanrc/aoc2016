use crate::{aoc, load_input};
use anyhow::Result;

aoc!(12);

pub fn day12_main() -> Result<()> {
    let input = load_input(12)?;
    let instructions: Vec<_> = input.lines().collect();

    assert_eq!(318083, part1_solution(&instructions));
    assert_eq!(9227737, part2_solution(&instructions));

    Ok(())
}

#[derive(Default)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Registers {
    fn get_from_str(&self, s: &str) -> Option<i32> {
        Some(match s {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            _ => return None,
        })
    }

    fn get_mut_from_str(&mut self, s: &str) -> Option<&mut i32> {
        Some(match s {
            "a" => &mut self.a,
            "b" => &mut self.b,
            "c" => &mut self.c,
            "d" => &mut self.d,
            _ => return None,
        })
    }
}

fn run_vm(input: &[&str], mut regs: Registers) -> i32 {
    let mut i = 0;

    while i < input.len() {
        let mut ins = input[i].split_whitespace();

        match ins.next().unwrap() {
            "cpy" => {
                let source = ins.next().unwrap();
                let source_val = source
                    .parse::<i32>()
                    .unwrap_or_else(|_| regs.get_from_str(source).unwrap());

                let dest = regs.get_mut_from_str(ins.next().unwrap()).unwrap();
                *dest = source_val;
            }
            "inc" => {
                let val = regs.get_mut_from_str(ins.next().unwrap()).unwrap();
                *val += 1;
            }
            "dec" => {
                let val = regs.get_mut_from_str(ins.next().unwrap()).unwrap();
                *val -= 1;
            }
            "jnz" => {
                let source = ins.next().unwrap();
                let source_val = source
                    .parse::<i32>()
                    .unwrap_or_else(|_| regs.get_from_str(source).unwrap());

                if source_val != 0 {
                    let y = ins.next().unwrap().parse::<i32>().unwrap();
                    i = (i as i32 + y) as usize;
                    continue;
                }
            }
            s => panic!("Unrecognized instruction: {:?}", s),
        }

        i += 1;
    }

    regs.a
}

fn part1_solution(input: &[&str]) -> i32 {
    let regs = Registers::default();
    run_vm(input, regs)
}

fn part2_solution(input: &[&str]) -> i32 {
    let mut regs = Registers::default();
    regs.c = 1;
    run_vm(input, regs)
}
