use aoc_rust::answer::Answer;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event22/day01/input.txt")?.read_to_string(&mut puzzle)?;

    let mut elfs_calories: Vec<isize> = Vec::new();
    let mut calories_count: isize = 0;

    for calorie_or_nothing in puzzle.lines() {
        if calorie_or_nothing.is_empty() {
            elfs_calories.push(calories_count);
            calories_count = 0;
        } else {
            let calories: isize = calorie_or_nothing.parse().unwrap();
            calories_count += calories;
        }
    }

    elfs_calories.sort_by(|a, b| b.cmp(a));

    let answer_p1 = elfs_calories[0];
    let answer_p2 = elfs_calories[0] + elfs_calories[1] + elfs_calories[2];

    Ok(Answer {
        event: String::from("2022"),
        day: String::from("01"),
        part1: answer_p1.to_string(),
        part2: answer_p2.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event22_day01() {
        let answer = solve().unwrap();
        assert_eq!(answer.part1, String::from("68802"));
        assert_eq!(answer.part2, String::from("205370"));
    }
}
