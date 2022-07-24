use std::fs::File;
use std::io::{prelude::*, self};
use aoc_rust::answer::Answer;

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event15/day02/input.txt")?
        .read_to_string(&mut puzzle)?;

    Ok(Answer {
        event: String::from("2015"),
        day: String::from("02"),
        part1: String::from("nothing"),
        part2: String::from("nothing"),
    })
}
