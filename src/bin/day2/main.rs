use std::{fs::read_to_string, str::FromStr};

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Move {
    fn asdasd(&self, mov: &Self) -> GameResult {
        if self == mov {
            return GameResult::Draw;
        };
        match (self, mov) {
            (Self::Rock, Self::Scissors) => GameResult::Win,
            (Self::Paper, Self::Rock) => GameResult::Win,
            (Self::Scissors, Self::Paper) => GameResult::Win,
            _ => GameResult::Loss,
        }
    }
}

fn main() {
    let input = read_to_string("src/bin/day2/input.txt").expect("Reading input file");

    let result: u32 = input.lines().fold(0, |res, l| {
        let (a, b) = l.split_once(" ").unwrap();
        let (a_move, b_move) = (Move::from_str(a).unwrap(), Move::from_str(b).unwrap());
        let mut points = b_move as u32;
        points += b_move.asdasd(&a_move) as u32;
        res + points
    });

    println!("{:?}", result);
}
