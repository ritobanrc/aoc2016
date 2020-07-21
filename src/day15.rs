use crate::aoc;
use anyhow::Result;

aoc!(15);
pub fn day15_main() -> Result<()> {
    let mut discs = vec![
        Disc::new(17, 15), // Disc #1 has 17 positions; at time=0, it is at position 15.
        Disc::new(3, 2),   // Disc #2 has 3 positions; at time=0, it is at position 2.
        Disc::new(19, 4),  // Disc #3 has 19 positions; at time=0, it is at position 4.
        Disc::new(13, 2),  // Disc #4 has 13 positions; at time=0, it is at position 2.
        Disc::new(7, 2),   // Disc #5 has 7 positions; at time=0, it is at position 2.
        Disc::new(5, 0),   // Disc #6 has 5 positions; at time=0, it is at position 0.
    ];

    assert_eq!(400589, dbg!(solution(discs.clone())));

    discs.push(Disc::new(11, 0));
    assert_eq!(3045959, dbg!(solution(discs)));
    Ok(())
}

#[derive(Debug, Clone)]
struct Disc {
    num_positions: usize,
    hole_position: usize,
}

impl Disc {
    fn new(num_positions: usize, hole_position: usize) -> Self {
        Self {
            num_positions,
            hole_position,
        }
    }

    fn rotate(&mut self) {
        self.hole_position = (self.hole_position + 1) % self.num_positions
    }
}

fn simulate(discs: &mut [Disc]) -> bool {
    for i in 0..discs.len() {
        if discs[i].hole_position != 0 {
            return false;
        }

        for disc in &mut *discs {
            disc.rotate();
        }
    }
    true
}

fn solution(mut discs: Vec<Disc>) -> usize {
    for wait_time in 0.. {
        discs.iter_mut().for_each(Disc::rotate);

        if simulate(&mut discs.clone()) {
            return wait_time;
        }
    }
    0
}
