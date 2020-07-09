use crate::{aoc, load_input};
use anyhow::Result;

aoc!(09);
pub fn day09_main() -> Result<()> {
    let input = load_input(09)?;

    assert_eq!(97714, part1_solution(&input));

    assert_eq!(10762972461, part2_solution(&input.trim())); // ngl i don't know why you have to subtract 1

    Ok(())
}

fn decompress_string(s: &str) -> String {
    let mut output = s.to_owned();
    let mut current = 0;

    while let Some(open_paren) = output[current..].find('(') {
        let open_paren = open_paren + current;
        let close_paren = output[open_paren..].find(')').unwrap() + open_paren;
        let marker = &output[open_paren + 1..close_paren];
        let x_location = marker.find('x').unwrap();
        let num_chars = marker[..x_location].parse::<usize>().unwrap();
        let num_repeats = marker[x_location + 1..].parse::<usize>().unwrap();

        let repeat_sequence = output[close_paren + 1..close_paren + 1 + num_chars].to_owned();

        for _ in 0..num_repeats - 1 {
            output.insert_str(close_paren + num_chars + 1, &repeat_sequence);
        }
        output.replace_range(open_paren..=close_paren, "");
        current += repeat_sequence.len() * num_repeats;

        if current > output.len() {
            break;
        }
    }

    output
}

fn part1_solution(input: &str) -> usize {
    decompress_string(input).len()
}

fn part2_solution(input: &str) -> usize {
    // Form a tree with each marker in the original string representing a node, and regular
    // characters leafs. Then, depth first search through that tree, multiplying all the repeat
    // counts together on the way down, and adding them together
    let mut total_length = 0;
    let mut current = 0;

    while let Some(open_paren_after_current) = input[current..].find('(') {
        println!("{:?}", &input[current..]);
        // we just skipped over an unrepeated segment, add it to the total length
        let open_paren = open_paren_after_current + current;
        total_length += open_paren_after_current;
        println!("Total length: {:?}", total_length);
        let close_paren = input[open_paren..].find(')').unwrap() + open_paren;
        let marker = &input[open_paren + 1..close_paren];
        println!("Marker: {:?}", marker);
        let x_location = marker.find('x').unwrap();
        let num_chars = marker[..x_location].parse::<usize>().unwrap();
        let num_repeats = marker[x_location + 1..].parse::<usize>().unwrap();

        let repeat_sequence = &input[close_paren + 1..close_paren + 1 + num_chars];

        println!(
            "Repeating {:?} {:?} times. Current = {:?}",
            repeat_sequence, num_repeats, current
        );

        let expanded_repeat_sequence_length = part2_solution(repeat_sequence);

        total_length += num_repeats * expanded_repeat_sequence_length;
        current = close_paren + 1 + repeat_sequence.len(); // +2 for the parens

        println!(
            "Total length: {:?} after {:?} repeats of {:?} is {:?} chars. Current = {:?}",
            total_length, num_repeats, repeat_sequence, expanded_repeat_sequence_length, current,
        );

        if current >= input.len() {
            println!("Returning because current >= len");
            return total_length;
        }
    }

    println!(
        "Returning. current = {:?}, input.len() = {:?}",
        current,
        input.len()
    );
    total_length + input.len() - current
}

#[test]
fn day09_examples() {
    assert_eq!("XYZXYZXYZ", decompress_string("(3x3)XYZ"));
    assert_eq!("ABCBCDEFEFG", decompress_string("A(2x2)BCD(2x2)EFG"));
    assert_eq!("(1x3)A", decompress_string("(6x1)(1x3)A"));
    assert_eq!("X(3x3)ABC(3x3)ABCY", decompress_string("X(8x2)(3x3)ABCY"));

    assert_eq!(20, part2_solution("X(8x2)(3x3)ABCY"));

    println!();
    assert_eq!(241920, part2_solution("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
    println!();
    assert_eq!(
        445,
        part2_solution("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
    );
}
