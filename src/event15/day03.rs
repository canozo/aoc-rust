use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, self};
use aoc_rust::answer::Answer;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

fn insert_or_modify_house(passport: &mut HashMap<Point, i32>, point: Point) {
    passport.entry(point)
        .and_modify(|p| { *p += 1 })
        .or_insert(1);
}

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event15/day03/input.txt")?
        .read_to_string(&mut puzzle)?;

    let mut loc_x: isize = 0;
    let mut loc_y: isize = 0;

    let mut santa_loc_x: isize = 0;
    let mut santa_loc_y: isize = 0;

    let mut robo_loc_x: isize = 0;
    let mut robo_loc_y: isize = 0;

    let mut visited_houses = HashMap::from([
        (Point::new(0, 0), 1),
    ]);

    let mut robo_santa_houses = HashMap::from([
        (Point::new(0, 0), 1),
    ]);

    let mut robo_turn = false;

    for direction in puzzle.chars() {
        let mut diff_x: isize = 0;
        let mut diff_y: isize = 0;

        match direction {
            '^' => diff_y += 1,
            'v' => diff_y += -1,
            '>' => diff_x += 1,
            '<' => diff_x += -1,
            _ => continue,
        }

        loc_x += diff_x;
        loc_y += diff_y;

        let point = Point::new(loc_x, loc_y);
        let robo_point: Point;

        if robo_turn {
            santa_loc_x += diff_x;
            santa_loc_y += diff_y;
            robo_point = Point::new(santa_loc_x, santa_loc_y);
        } else {
            robo_loc_x += diff_x;
            robo_loc_y += diff_y;
            robo_point = Point::new(robo_loc_x, robo_loc_y);
        }

        insert_or_modify_house(&mut visited_houses, point);
        insert_or_modify_house(&mut robo_santa_houses, robo_point);

        robo_turn = !robo_turn;
    }

    Ok(Answer {
        event: String::from("2015"),
        day: String::from("03"),
        part1: visited_houses.len().to_string(),
        part2: robo_santa_houses.len().to_string(),
    })
}
