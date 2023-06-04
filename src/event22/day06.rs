use aoc_rust::answer::Answer;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event22/day06/input.txt")?.read_to_string(&mut puzzle)?;

    Ok(Answer {
        event: String::from("2022"),
        day: String::from("06"),
        part1: find_start_of_marker(&puzzle, 4).to_string(),
        part2: find_start_of_marker(&puzzle, 14).to_string(),
    })
}

fn find_start_of_marker(input: &String, slice_size: usize) -> usize {
    let mut answer = 0;
    let mut slice_from = 0;

    while answer == 0 {
        let slice = &input[slice_from..slice_from + slice_size];

        let has_repetition = slice
            .chars()
            .enumerate()
            .find(|&(first_idx, first)| {
                let (idx_found, val_found) = slice
                    .chars()
                    .enumerate()
                    .find(|&(second_idx, second)| first_idx != second_idx && first == second)
                    .unwrap_or_default();

                if val_found != char::default() {
                    slice_from = std::cmp::min(first_idx, idx_found) + slice_from + 1;
                    true
                } else {
                    false
                }
            })
            .unwrap_or_default()
            .1
            != char::default();

        if !has_repetition {
            answer = slice_from + slice_size;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event22_day06() {
        let answer = solve().unwrap();
        assert_eq!(answer.part1, String::from("1647"));
        assert_eq!(answer.part2, String::from("2447"));
    }
}
