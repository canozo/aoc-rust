use aoc_rust::answer::Answer;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event22/day04/input.txt")?.read_to_string(&mut puzzle)?;

    let mut answer_p1 = 0;
    let mut answer_p2 = 0;

    for assignments in puzzle.lines() {
        let (first_pair, second_pair) = assignments.split_once(',').unwrap();

        let (fp_from, fp_to) = first_pair.split_once('-').unwrap();
        let (sp_from, sp_to) = second_pair.split_once('-').unwrap();

        let fp_from: i32 = fp_from.parse().unwrap();
        let fp_to: i32 = fp_to.parse().unwrap();
        let sp_from: i32 = sp_from.parse().unwrap();
        let sp_to: i32 = sp_to.parse().unwrap();

        // part 1
        if (fp_from <= sp_from && fp_to >= sp_to) || (sp_from <= fp_from && sp_to >= fp_to) {
            answer_p1 += 1;
        }

        // part 2
        if fp_to >= sp_from && sp_to >= fp_from {
            answer_p2 += 1;
        }
    }

    Ok(Answer {
        event: String::from("2022"),
        day: String::from("04"),
        part1: answer_p1.to_string(),
        part2: answer_p2.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event22_day04() {
        let answer = solve().unwrap();
        assert_eq!(answer.part1, String::from("462"));
        assert_eq!(answer.part2, String::from("835"));
    }
}
