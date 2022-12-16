use std::fs;

struct Day02 {}

enum MyMove {
    X,
    Y,
    Z,
}

enum OpponentMove {
    A,
    B,
    C,
}

impl Day02 {
    fn get_input() -> String {
        fs::read_to_string("src/day_02/input.txt").expect("Failed to read file")
    }
}

pub fn part_one() -> i32 {
    let input = Day02::get_input();
    let rounds = input
        .split("\n")
        .map(|r| r.replace(" ", ""))
        .collect::<Vec<String>>();

    let total_points = rounds.into_iter().fold(0, |sum, val| {
        let opponent_move = val.chars().nth(0).unwrap();
        let my_move = val.chars().nth(1).unwrap();

        sum + match opponent_move {
            'A' => match my_move {
                'X' => 1 + 3,
                'Y' => 2 + 6,
                'Z' => 3 + 0,
                _ => 0,
            },
            'B' => match my_move {
                'X' => 1 + 0,
                'Y' => 2 + 3,
                'Z' => 3 + 6,
                _ => 0,
            },
            'C' => match my_move {
                'X' => 1 + 6,
                'Y' => 2 + 0,
                'Z' => 3 + 3,
                _ => 0,
            },
            _ => 0,
        }
    });

    total_points
}
