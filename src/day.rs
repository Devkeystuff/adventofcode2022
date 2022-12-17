use std::fs;

pub trait Day {
    fn get_input(day: i32) -> String {
        let day = if day < 10 {
            format!("0{}", day)
        } else {
            day.to_string()
        };
        fs::read_to_string(format!("src/day_{}/input.txt", day)).expect("Failed to read file")
    }

    fn part_one() -> String;

    fn part_two() -> String;
}
