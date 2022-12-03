#![allow(non_snake_case)]

mod data;
use data::DATA;

const VALUE_WIN: u32 = 6;
const VALUE_DRAW: u32 = 3;
const VALUE_LOSS: u32 = 0;

#[derive(Copy, Clone, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    Win = 50,
    Draw = 51,
    Lose = 52,
}

impl Choice {
    pub(crate) fn challenge(&self, other: &Self) -> u32 {
        let outcome = match other {
            Self::Rock => match self {
                Self::Win => Self::Paper as u32,
                Self::Draw => Self::Rock as u32,
                Self::Lose => Self::Scissors as u32,
                _ => panic!("bad data")
            }
            Self::Paper => match self {
                Self::Win => Self::Scissors as u32,
                Self::Draw => Self::Paper as u32,
                Self::Lose => Self::Rock as u32,
                _ => panic!("bad data")
            }
            Self::Scissors => match self {
                Self::Win => Self::Rock as u32,
                Self::Draw => Self::Scissors as u32,
                Self::Lose => Self::Paper as u32,
                _ => panic!("bad data")
            }
            _ => panic!("bad data")
        };

        let bonus = match self {
            Self::Win => VALUE_WIN,
            Self::Draw => VALUE_DRAW,
            Self::Lose => VALUE_LOSS,
            _ => panic!("bad data")
        };

        outcome + bonus
    }
}

impl From<&str> for Choice {
    fn from(value: &str) -> Self {
        match value {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            "Z" => Choice::Win,
            "Y" => Choice::Draw,
            "X" => Choice::Lose,
            _ => panic!("Bad data happened somehow from a constant?!")
        }
    }
}

impl core::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Rock => "Rock",
            Self::Paper => "Paper",
            Self::Scissors => "Scissors",
            Self::Win => "Win",
            Self::Draw => "Draw",
            Self::Lose => "Lose",
        };

        write!(f, "{}", value)
    }
}

fn main() {
    let mut total = 0;

    for line in DATA.split('\n') {
        let (them, me) = line.split_once(' ').unwrap();

        let them = Choice::from(them);
        let me = Choice::from(me);
        let result = me.challenge(&them);

        println!("Challenge: {0:^10} ? {1:^10} => {2:}", format!("{}", them), format!("{}", me), result);
        total += result;
    }

    println!("Total score is {total}.");
}
