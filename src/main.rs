pub mod event15;

use std::error::Error;
use aoc_rust::answer::Answer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut answers: Vec<Answer> = Vec::new();

    answers.push(event15::day01::solve()?);
    answers.push(event15::day02::solve()?);

    println!("+{:->39}+", '-');
    println!("| {:<5} | {:<3} | {:<10} | {:<10} |", "event", "day", "part 1", "part 2");
    println!("+{:->39}+", '-');

    for answer in answers {
        answer.print();
    }

    println!("+{:->39}+", '-');

    Ok(())
}
