use crate::aoc;
use anyhow::Result;

aoc!(05);

pub fn day05_main() -> Result<()> {
    const INPUT: &'static str = "ffykfhsq";
    assert_eq!("c6697b55", part1_solution(INPUT));
    assert_eq!("8c35d1ab", part2_solution(INPUT));
    Ok(())
}

fn part1_solution(input: &str) -> String {
    let mut password = String::new();
    for suffix in 1u64.. {
        let digest = md5::compute(input.to_owned() + &suffix.to_string());
        let hex = format!("{:x}", digest);
        if &hex[0..5] == "00000" {
            password.push(hex.as_bytes()[5] as char);
            if password.len() == 8 {
                break;
            }
        }
    }
    password
}

fn part2_solution(input: &str) -> String {
    let mut password = vec![b' '; 8];

    for suffix in 1u64.. {
        let digest = md5::compute(input.to_owned() + &suffix.to_string());
        let hex = format!("{:x}", digest);
        if &hex[0..5] == "00000" {
            let pos = (hex.as_bytes()[5] - b'0') as usize;
            if pos >= password.len() || password[pos] != b' ' {
                continue;
            }
            password[pos] = hex.as_bytes()[6];
            if password.iter().all(|x| *x != b' ') {
                break;
            }
        }
    }
    String::from_utf8(password).unwrap()
}
