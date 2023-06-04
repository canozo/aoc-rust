use aoc_rust::answer::Answer;
use std::fs::File;
use std::io::{self, prelude::*};
use std::str::Lines;

struct Directory {
    path: String,
    size: usize,
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Directory {{ size={:0>8}, path={} }}", self.size, self.path)
    }
}

const DISK_SIZE: usize = 70000000;
const MIGRATION_SIZE: usize = 30000000;

pub fn solve() -> Result<Answer, io::Error> {
    let mut puzzle = String::new();

    File::open("inputs/event22/day07/input.txt")?.read_to_string(&mut puzzle)?;

    let mut directories: Vec<Directory> = Vec::new();
    let mut commands = puzzle.lines();

    commands.next(); // skip "cd /"

    change_directory(String::from('/'), &mut directories, &mut commands);

    // directories.iter().for_each(|d| println!("{}", d));

    let unused_space = DISK_SIZE - directories.last().unwrap().size;
    let necessary_space = MIGRATION_SIZE - unused_space;

    let answer_p1 = directories
        .iter()
        .filter(|dir| dir.size <= 100000)
        .map(|dir| dir.size)
        .reduce(|a, b| a + b)
        .unwrap();

    let mut answer_p2 = std::usize::MAX;

    directories.iter().for_each(|dir| {
        if dir.size >= necessary_space {
            answer_p2 = std::cmp::min(answer_p2, dir.size);
        }
    });

    Ok(Answer {
        event: String::from("2022"),
        day: String::from("07"),
        part1: answer_p1.to_string(),
        part2: answer_p2.to_string(),
    })
}

fn change_directory(path: String, directories: &mut Vec<Directory>, commands: &mut Lines) -> usize {
    let mut size = 0;

    while let Some(command) = commands.next() {
        if command.starts_with("$ cd") {
            let (_, directory) = command.split_at(5);

            if directory.eq("..") {
                break;
            }

            let mut new_path = path.clone();
            new_path.push_str(directory);
            new_path.push('/');

            size += change_directory(new_path, directories, commands);
        } else if command.starts_with('d') || command.starts_with('$') {
            continue;
        } else {
            let (file_size, _) = command.split_once(" ").unwrap();
            size += file_size.parse::<usize>().unwrap();
        };
    }

    directories.push(Directory { path, size });

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event22_day07() {
        let answer = solve().unwrap();
        assert_eq!(answer.part1, String::from("1427048"));
        assert_eq!(answer.part2, String::from("2940614"));
    }
}
