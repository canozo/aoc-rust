use aoc_rust::answer::Answer;
use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event15/day02/input.txt")?.read_to_string(&mut puzzle)?;

    let mut answer_p1: usize = 0;
    let mut answer_p2: usize = 0;

    for raw_dimensions in puzzle.lines() {
        let dimensions: Vec<&str> = raw_dimensions.split('x').collect();

        let length: usize = dimensions[0].parse().unwrap();
        let width: usize = dimensions[1].parse().unwrap();
        let height: usize = dimensions[2].parse().unwrap();

        let side1 = length * width;
        let side2 = width * height;
        let side3 = height * length;

        let min_side = cmp::min(side1, cmp::min(side2, side3));
        answer_p1 += 2 * side1 + 2 * side2 + 2 * side3 + min_side;

        let smallest_perim =
            (length + width + height - cmp::max(length, cmp::max(width, height))) * 2;
        answer_p2 += length * width * height + smallest_perim;
    }

    Ok(Answer {
        event: String::from("2015"),
        day: String::from("02"),
        part1: answer_p1.to_string(),
        part2: answer_p2.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event15_day02() {
        let answer = solve().unwrap();
        assert_eq!(answer.part1, String::from("1606483"));
        assert_eq!(answer.part2, String::from("3842356"));
    }
}
