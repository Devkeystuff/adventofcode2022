use strum_macros::EnumString;

use crate::day::Day;

pub struct Day02 {}

#[derive(EnumString, PartialEq)]
enum Move {
    #[strum(serialize = "A", serialize = "X")]
    Rock = 1,
    #[strum(serialize = "B", serialize = "Y")]
    Paper = 2,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors = 3,
}

#[derive(EnumString)]
enum GameResult {
    #[strum(serialize = "X")]
    Lose = 1,
    #[strum(serialize = "Y")]
    Draw = 2,
    #[strum(serialize = "Z")]
    Win = 3,
}

impl Move {
    fn compare_move(my_move: Move, opponent_move: Move) -> i32 {
        let shape_points = my_move as i32;
        let diff = shape_points - opponent_move as i32;
        if diff == -2 || diff == 1 {
            shape_points + 6
        } else if diff == -1 || diff == 2 {
            shape_points
        } else {
            shape_points + 3
        }
    }

    fn calculate_move(opponent_move: Move, result: GameResult) -> i32 {
        let sum = result as i32 + opponent_move as i32;
        let my_move = if sum == 2 || sum == 5 {
            Move::Scissors
        } else if sum == 3 || sum == 6 {
            Move::Rock
        } else {
            Move::Paper
        };
        Move::compare_move(my_move, opponent_move)
    }
}

impl Day02 {
    fn get_rounds(input: String) -> Vec<String> {
        input
            .split("\n")
            .map(|r| r.replace(" ", ""))
            .collect::<Vec<String>>()
    }
}

impl Day for Day02 {
    fn part_one() -> String {
        let input = Day02::get_input(2);
        let rounds = Day02::get_rounds(input);

        let total_points = rounds.into_iter().fold(0, |sum, val| {
            let opponent_move: Move = (&val[0..1]).parse().unwrap();
            let my_move: Move = (&val[1..2]).parse().unwrap();

            sum + Move::compare_move(my_move, opponent_move)
        });

        total_points.to_string()
    }

    fn part_two() -> String {
        let input = Day02::get_input(2);
        let rounds = Day02::get_rounds(input);

        let total_points = rounds.into_iter().fold(0, |sum, val| {
            let opponent_move: Move = (&val[0..1]).parse().unwrap();
            let game_result: GameResult = (&val[1..2]).parse().unwrap();

            sum + Move::calculate_move(opponent_move, game_result)
        });

        total_points.to_string()
    }
}
