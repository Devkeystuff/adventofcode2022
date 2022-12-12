use std::fs;

pub fn main() -> i32 {
    let contents = fs::read_to_string("src/day_01/input.txt").expect("Failed to read file");

    let elves_total_carriage = contents
        // Split by empty lines
        .split("\n\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|c| {
            // Split every elf's inventory by newline
            c.split("\n")
                // Iterate over inventory and parse every calorie value to i32
                .map(|item| item.parse::<i32>().expect("Couldn't parse contents"))
                // Sum the inventory
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    // Find the max value
    let max_calories = elves_total_carriage
        .iter()
        .max()
        .expect("Couldn't find the max value");

    *max_calories
}
