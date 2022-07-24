pub struct Answer {
    pub event: String,
    pub day: String,
    pub part1: String,
    pub part2: String,
}

impl Answer {
    pub fn print(self) {
        println!("| {:<5} | {:<3} | {:<10} | {:<10} |", self.event, self.day, self.part1, self.part2);
    }
}
