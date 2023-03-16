use aoc_rust::answer::Answer;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event22/day02/input.txt")?.read_to_string(&mut puzzle)?;

    let mut answer_p1 = 0;
    let mut answer_p2 = 0;

    for strategy in puzzle.lines() {
        let (result_p1, result_p2) = rps_result(strategy);
        answer_p1 += result_p1;
        answer_p2 += result_p2;
    }

    Ok(Answer {
        event: String::from("2022"),
        day: String::from("02"),
        part1: answer_p1.to_string(),
        part2: answer_p2.to_string(),
    })
}

fn rps_result(strategy: &str) -> (i32, i32) {
    match strategy {
        "A Z" => (0 + 3, 6 + 2),
        "B X" => (0 + 1, 0 + 1),
        "C Y" => (0 + 2, 3 + 3),
        "A X" => (3 + 1, 0 + 3),
        "B Y" => (3 + 2, 3 + 2),
        "C Z" => (3 + 3, 6 + 1),
        "A Y" => (6 + 2, 3 + 1),
        "B Z" => (6 + 3, 6 + 3),
        "C X" => (6 + 1, 0 + 2),
        _ => panic!("Invalid strategy '{}'", strategy),
    }
}
