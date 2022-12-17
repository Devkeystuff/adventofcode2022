use crate::day::Day;

struct Day01 {}

impl Day01 {
    fn get_total_carriage_per_elf(input: String) -> Vec<i32> {
        input
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
            .collect::<Vec<_>>()
    }
}

impl Day for Day01 {
    fn part_one() -> String {
        let contents = Day01::get_input(1);

        let elves_total_carriage = Day01::get_total_carriage_per_elf(contents);

        // Find the max value
        let max_calories = elves_total_carriage
            .iter()
            .max()
            .expect("Couldn't find the max value");

        max_calories.to_string()
    }

    fn part_two() -> String {
        let contents = Day01::get_input(1);

        let mut elves_total_carriage = Day01::get_total_carriage_per_elf(contents);

        elves_total_carriage.sort();
        elves_total_carriage.reverse();

        let top_three = elves_total_carriage[0..3].into_iter().sum::<i32>();

        top_three.to_string()
    }
}
