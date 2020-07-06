use crate::{aoc, load_input};
use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::HashMap;

aoc!(04);

pub fn day04_main() -> Result<()> {
    let input = load_input(04)?;
    let entries = input
        .lines()
        .map(|line| {
            let open_bracket = line.find('[')?;
            let mut pieces = line[..open_bracket].split('-').collect::<Vec<_>>();
            let sector_id = pieces.pop()?.parse::<u32>().ok()?;
            Some(RoomEntry {
                name: pieces,
                sector_id,
                checksum: &line[open_bracket + 1..line.len() - 1],
            })
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(anyhow!("Failed to parse"))?;

    assert_eq!(137896, part1_solution(&entries));
    assert_eq!(501, part2_solution(&entries)?);
    Ok(())
}

#[derive(Clone, Debug)]
struct RoomEntry<'a> {
    name: Vec<&'a str>,
    sector_id: u32,
    checksum: &'a str,
}

fn is_room_real(entry: &RoomEntry) -> bool {
    let frequencies = {
        // first build a hashmap to store the frequences
        // then convert into a vector
        let mut m = HashMap::new();
        for c in entry.name.iter().map(|x| x.chars()).flatten() {
            m.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut v = m.into_iter().collect::<Vec<_>>();
        v.sort_by(
            |(letter1, freq1), (letter2, freq2)| match freq1.cmp(freq2) {
                // NOTE: this is intentionally backwards, so that later, when we
                // `rev` the vector, it becomes the right way around
                Ordering::Equal => letter2.cmp(letter1),
                ordering => ordering,
            },
        );
        v
    };

    frequencies
        .iter()
        .rev()
        .map(|(letter, _freq)| letter)
        .take(5)
        .collect::<String>()
        == entry.checksum
}

fn part1_solution(entries: &[RoomEntry]) -> u32 {
    entries
        .iter()
        .filter_map(|entry| {
            // tabulate the frequences in entry.name
            if is_room_real(entry) {
                Some(entry.sector_id)
            } else {
                None
            }
        })
        .sum()
}

fn part2_solution(entries: &[RoomEntry]) -> Result<u32> {
    for room in entries.iter().filter(|x| is_room_real(x)) {
        let name = room
            .name
            .iter()
            .map(|piece| {
                let mut s = String::from_utf8(
                    piece
                        .bytes()
                        .map(|c| (((c - b'a') as u32 + (room.sector_id % 26)) % 26) as u8 + b'a')
                        .collect(),
                )?;
                s.push(' ');
                Ok::<_, std::string::FromUtf8Error>(s)
            })
            .collect::<Result<String, _>>()?;

        if name.contains("northpole") {
            return Ok(room.sector_id);
        }
    }
    Err(anyhow!("Could not find northpole object storage"))
}
