use crate::day::Day;

pub struct Day03 {}

struct Rucksack(String, String);

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

impl Day03 {
    fn get_item_priority(item: char) -> i32 {
        let lowercase_char = item.to_ascii_lowercase();
        let mut priority = (LETTERS.find(lowercase_char).unwrap() + 1) as i32;
        if item.is_uppercase() {
            priority += LETTERS.len() as i32
        }
        priority
    }
}

impl Day for Day03 {
    fn part_one() -> String {
        let input = Day03::get_input(3);
        let rucksack_compartments = input
            .split("\n")
            .map(|row| {
                let first_compartment = &row[..row.len() / 2];
                let second_compartment = &row[row.len() / 2..];
                Rucksack(
                    first_compartment.to_string(),
                    second_compartment.to_string(),
                )
            })
            .collect::<Vec<Rucksack>>();

        let priority_sum = rucksack_compartments
            .iter()
            .map(|rucksack| {
                let mut priority: i32 = 0;
                for item in rucksack.0.chars() {
                    if rucksack.1.contains(item) {
                        priority = Day03::get_item_priority(item);
                        break;
                    }
                }
                priority
            })
            .sum::<i32>();

        priority_sum.to_string()
    }

    fn part_two() -> String {
        let input = Day03::get_input(3);
        let elf_groups = input
            .split("\n")
            .fold(Vec::<Vec<&str>>::new(), |mut acc, rucksack| {
                if acc.is_empty() || acc.last().unwrap().len() % 3 == 0 {
                    acc.push(Vec::new());
                }
                acc.last_mut().unwrap().push(rucksack);
                acc
            });
        let priority_sum = elf_groups
            .into_iter()
            .map(|group| {
                let mut matching_item = 0;
                for item in group.iter().next().unwrap().chars() {
                    println!("{}", item);
                    if group.iter().nth(1).unwrap().contains(item)
                        && group.iter().nth(2).unwrap().contains(item)
                    {
                        matching_item = Day03::get_item_priority(item);
                        break;
                    };
                }
                matching_item
            })
            .sum::<i32>();

        priority_sum.to_string()
    }
}
