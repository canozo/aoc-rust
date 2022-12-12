pub mod event15;
pub mod event22;

use std::error::Error;
use aoc_rust::answer::Answer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut answers: Vec<Answer> = Vec::new();

    // 2015
    answers.push(event15::day01::solve()?);
    answers.push(event15::day02::solve()?);
    answers.push(event15::day03::solve()?);

    // 2022
    answers.push(event22::day01::solve()?);

    println!("+{:->39}+", '-');
    println!("| {:<5} | {:<3} | {:<10} | {:<10} |", "event", "day", "part 1", "part 2");
    println!("+{:->39}+", '-');

    for answer in answers {
        answer.print();
    }

    println!("+{:->39}+", '-');

    Ok(())
}
