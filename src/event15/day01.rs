use std::fs::File;
use std::io::{prelude::*, self};
use aoc_rust::answer::Answer;

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event15/day01/input.txt")?
        .read_to_string(&mut puzzle)?;

    let answer_p1 = puzzle.matches('(').count() as isize - puzzle.matches(')').count() as isize;

    let mut current_floor = 0;
    let mut answer_p2 = 0;

    for next_floor in puzzle.chars() {
        answer_p2 += 1;
        current_floor += if next_floor == '(' { 1 } else { -1 };

        if current_floor == -1 {
            break;
        }
    }

    Ok(Answer {
        event: String::from("2015"),
        day: String::from("01"),
        part1: answer_p1.to_string(),
        part2: answer_p2.to_string(),
    })
}
