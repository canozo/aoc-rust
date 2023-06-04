pub mod event15;
pub mod event22;

use aoc_rust::answer::Answer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let answers: Vec<Answer> = vec![
        // 2015
        event15::day01::solve()?,
        event15::day02::solve()?,
        event15::day03::solve()?,

        // 2022
        event22::day01::solve()?,
        event22::day02::solve()?,
        event22::day03::solve()?,
        event22::day04::solve()?,
        event22::day05::solve()?,
        event22::day06::solve()?,
    ];

    println!("+{:->39}+", '-');
    println!(
        "| {:<5} | {:<3} | {:<10} | {:<10} |",
        "event", "day", "part 1", "part 2"
    );
    println!("+{:->39}+", '-');

    for answer in answers {
        answer.print();
    }

    println!("+{:->39}+", '-');

    Ok(())
}
