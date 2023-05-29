use aoc_rust::answer::Answer;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event22/day03/input.txt")?.read_to_string(&mut puzzle)?;

    let mut answer_p1 = 0;
    let mut answer_p2 = 0;

    let mut possible_badges = String::new();

    for (index_ruckpack, ruckpack) in puzzle.lines().enumerate() {
        // part 1
        let (first_group, second_group) = ruckpack.split_at(ruckpack.len() / 2);

        for item in first_group.chars() {
            if second_group.contains(item) {
                answer_p1 += get_priority(item);
                break;
            }
        }

        // part 2
        if index_ruckpack % 3 == 0 {
            possible_badges = String::from(ruckpack);
        } else {
            for possible_badge in possible_badges.clone().chars() {
                if !ruckpack.contains(possible_badge) {
                    possible_badges = possible_badges.replace(possible_badge, "");
                }
            }

            if index_ruckpack % 3 == 2 {
                let badge = possible_badges.chars().nth(0).unwrap();
                answer_p2 += get_priority(badge);
            }
        }
    }

    Ok(Answer {
        event: String::from("2022"),
        day: String::from("03"),
        part1: answer_p1.to_string(),
        part2: answer_p2.to_string(),
    })
}

fn get_priority(item: char) -> i32 {
    let ascii_value = (item as u8 as i32) - 65;
    let offset = if item.is_lowercase() { -31 } else { 27 };

    ascii_value + offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event22_day03() {
        let answer = solve().unwrap();
        assert_eq!(answer.part1, String::from("7997"));
        assert_eq!(answer.part2, String::from("2545"));
    }
}
