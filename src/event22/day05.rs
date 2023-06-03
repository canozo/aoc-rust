use aoc_rust::answer::Answer;
use std::fs::File;
use std::io::{self, prelude::*};
use std::str::FromStr;

/// I messed up saving the input files when my editor trimmed the spaces from the "crates drawing",
/// by the time I realized I had already implemented most of the solution so I just rolled with it
pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event22/day05/input.txt")?.read_to_string(&mut puzzle)?;

    let (raw_stacks, procedures) = puzzle.split_once("\n\n").unwrap();

    Ok(Answer {
        event: String::from("2022"),
        day: String::from("05"),
        part1: cratemover(raw_stacks, procedures, 9000),
        part2: cratemover(raw_stacks, procedures, 9001),
    })
}

fn cratemover(raw_stacks: &str, procedures: &str, version: u16) -> String {
    let stacks_count = raw_stacks.lines().last().unwrap().replace(" ", "").len();

    let mut stacks: Vec<Vec<char>> = std::iter::repeat(Vec::new()).take(stacks_count).collect();

    raw_stacks.lines().rev().skip(1).for_each(|line| {
        let mut stack_pointer = 0;
        let mut parsed_line = String::from_str(line).unwrap();

        while parsed_line.contains("     ") {
            parsed_line = parsed_line.replace("     ", " [0] ");
        }

        while parsed_line.contains("    ") {
            parsed_line = parsed_line.replace("    ", "[0] ");
        }

        parsed_line.chars().skip(1).for_each(|c| {
            if c.is_alphabetic() {
                stacks.get_mut(stack_pointer).unwrap().push(c);
                stack_pointer += 1;
            } else if c.is_numeric() {
                stack_pointer += 1;
            }
        });
    });

    procedures.lines().for_each(|procedure| {
        let parsed_procedure = String::from_str(procedure)
            .unwrap()
            .replace("move ", "")
            .replace(" from ", "|")
            .replace(" to ", "|");

        let (move_count, rest) = parsed_procedure.split_once("|").unwrap();
        let (move_from, move_to) = rest.split_once("|").unwrap();

        let mut move_count: usize = move_count.parse().unwrap();
        let move_from: usize = move_from.parse().unwrap();
        let move_to: usize = move_to.parse().unwrap();

        if version == 9000 {
            while move_count > 0 {
                let move_from = stacks.get_mut(move_from - 1).unwrap();
                let popped_value = move_from.pop().unwrap();

                let move_to = stacks.get_mut(move_to - 1).unwrap();
                move_to.push(popped_value);

                move_count -= 1;
            }

            return;
        }

        if version == 9001 {
            let move_from = stacks.get_mut(move_from - 1).unwrap();

            let drain_to = move_from.len();
            let drain_from = drain_to - move_count;

            let picked_crates = move_from.drain(drain_from..drain_to);

            let mut buffer_crates: Vec<char> = Vec::new();
            buffer_crates.extend(picked_crates);

            let move_to = stacks.get_mut(move_to - 1).unwrap();
            move_to.extend(buffer_crates);

            return;
        }

        panic!("Invalid version \"{version}\"");
    });

    stacks
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.get(stack.len() - 1).unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event22_day05() {
        let answer = solve().unwrap();
        assert_eq!(answer.part1, String::from("SVFDLGLWV"));
        assert_eq!(answer.part2, String::from("DCVTCVPCL"));
    }
}
