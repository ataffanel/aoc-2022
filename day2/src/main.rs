// https://adventofcode.com/2022/day/2

use std::{fs::File, io::Read};

#[derive(Copy, Clone, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Sissors = 3,
}

impl From<&str> for Shape {
    fn from(raw: &str) -> Self {
        match raw {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Sissors,
            _ => panic!("Wrong input format"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(raw: &str) -> Self {
        match raw {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Wrong input format"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Round {
    mine: Shape,
    theire: Shape,
}

impl Round {
    fn play(&self) -> Outcome {
        match (self.mine, self.theire) {
            (Shape::Rock, Shape::Paper) => Outcome::Loss,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Sissors) => Outcome::Loss,
            (Shape::Sissors, Shape::Paper) => Outcome::Win,
            (Shape::Sissors, Shape::Rock) => Outcome::Loss,
            (Shape::Rock, Shape::Sissors) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn new_from_indended_outcome(theire: Shape, outcome: Outcome) -> Self {
        for mine in [Shape::Rock, Shape::Paper, Shape::Sissors] {
            let round = Round { mine, theire };
            if round.play() == outcome {
                return round;
            }
        }

        unreachable!();
    }
}

fn main() {
    // Part 1: entry are interpreted as shapes
    let input: Vec<Round> = {
        let mut file = File::open("input").unwrap();
        let mut raw = String::new();
        file.read_to_string(&mut raw).unwrap();
        raw.lines()
            .map(|line| {
                let mut hands = line.split(' ');
                Round {
                    theire: hands.next().unwrap().into(),
                    mine: hands.next().unwrap().into(),
                }
            })
            .collect()
    };

    let score = input
        .iter()
        .map(|round| round.mine as u64 + round.play() as u64)
        .fold(0, |score, round| score + round);

    println!("Part 1: My final score is: {}", score);

    // Part 2: Entries are interpreted as their play and the intended result
    let input: Vec<Round> = {
        let mut file = File::open("input").unwrap();
        let mut raw = String::new();
        file.read_to_string(&mut raw).unwrap();
        raw.lines()
            .map(|line| {
                let mut hands = line.split(' ');
                let theire = hands.next().unwrap().into();
                let outcome = hands.next().unwrap().into();
                Round::new_from_indended_outcome(theire, outcome)
            })
            .collect()
    };

    let score = input
        .iter()
        .map(|round| round.mine as u64 + round.play() as u64)
        .fold(0, |score, round| score + round);

    println!("Part 2: My final score is: {}", score);

}
