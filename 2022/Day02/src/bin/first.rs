#![allow(non_snake_case)]

mod data;
use data::DATA;

#[derive(Copy, Clone, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

const VALUE_WIN: u32 = 6;
const VALUE_DRAW: u32 = 3;
const VALUE_LOSS: u32 = 0;

impl Choice {
    pub(crate) fn challenge(&self, other: &Self) -> u32 {
        let outcome = match self {
            Self::Rock => match other {
                Self::Rock => VALUE_DRAW,
                Self::Paper => VALUE_LOSS,
                Self::Scissors => VALUE_WIN
            }
            Self::Paper => match other {
                Self::Rock => VALUE_WIN,
                Self::Paper => VALUE_DRAW,
                Self::Scissors => VALUE_LOSS,
            }
            Self::Scissors => match other {
                Self::Rock => VALUE_LOSS,
                Self::Paper => VALUE_WIN,
                Self::Scissors => VALUE_DRAW,
            }
        };

        outcome + self.clone() as u32
    }
}

impl From<&str> for Choice {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Bad data happened somehow from a constant?!")
        }
    }
}

impl core::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissors => "Scissors",
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
