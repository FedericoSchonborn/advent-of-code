use std::{convert::Infallible, str::FromStr};

use aoc::{Day, Status};

pub struct Day02;

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    pub fn compare(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => Outcome::Won,
            (Shape::Paper, Shape::Rock) => Outcome::Won,
            (Shape::Scissors, Shape::Paper) => Outcome::Won,
            (Shape::Rock, Shape::Paper) => Outcome::Lost,
            (Shape::Paper, Shape::Scissors) => Outcome::Lost,
            (Shape::Scissors, Shape::Rock) => Outcome::Lost,
            _ => Outcome::Draw,
        }
    }
}

impl FromStr for Shape {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl FromStr for Outcome {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Lost,
            "Y" => Self::Draw,
            "Z" => Self::Won,
            _ => unreachable!(),
        })
    }
}

pub fn guess(shape: Shape, outcome: Outcome) -> Shape {
    match (shape, outcome) {
        (Shape::Rock, Outcome::Won) => Shape::Paper,
        (Shape::Rock, Outcome::Lost) => Shape::Scissors,
        (Shape::Paper, Outcome::Won) => Shape::Scissors,
        (Shape::Paper, Outcome::Lost) => Shape::Rock,
        (Shape::Scissors, Outcome::Won) => Shape::Rock,
        (Shape::Scissors, Outcome::Lost) => Shape::Paper,
        (shape, Outcome::Draw) => shape,
    }
}

impl Day for Day02 {
    const NUMBER: u8 = 2;
    const TITLE: &'static str = "Rock Paper Scissors";
    const INPUT: &'static str = include_str!("input.txt");
    type Input = Vec<(Shape, (Shape, Outcome))>;
    type Output = usize;

    fn parse(raw: &'static str) -> Self::Input {
        let mut pairs = vec![];
        for line in raw.trim().split('\n') {
            let (their, our) = line.split_once(' ').unwrap();
            let (their, our) = (
                their.parse().unwrap(),
                (our.parse().unwrap(), our.parse().unwrap()),
            );
            pairs.push((their, our))
        }

        pairs
    }

    fn part1(input: &Self::Input) -> Status<Self::Output> {
        let mut total_score = 0;
        for (their, (our, _)) in input {
            let outcome = our.compare(their);
            total_score += *our as usize + outcome as usize;
        }

        Status::Solved(total_score, 9241)
    }

    fn part2(input: &Self::Input) -> Status<Self::Output> {
        let mut total_score = 0;
        for (their, (_, want)) in input {
            let our = guess(*their, *want);
            total_score += our as usize + *want as usize;
        }

        Status::Solved(total_score, 14610)
    }
}
