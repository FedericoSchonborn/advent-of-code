use std::{collections::HashMap, iter::repeat};

use aoc::{Day, Status};

pub struct Day03;

pub const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn score(c: char) -> usize {
    CHARS.find(c).unwrap() + 1
}

impl Day for Day03 {
    const NUMBER: u8 = 3;
    const TITLE: &'static str = "Rucksack Reorganization";
    const INPUT: &'static str = include_str!("input.txt");
    type Input = Vec<(&'static str, Vec<char>, Vec<char>)>;
    type Output = usize;

    fn parse(raw: &'static str) -> Self::Input {
        let mut sacks = vec![];
        for line in raw.trim().lines() {
            let line = line.trim();
            let mid = line.len() / 2;
            let first = line[..mid].chars().collect();
            let last = line[mid..].chars().collect();

            sacks.push((line, first, last));
        }

        sacks
    }

    // TODO: This is bad.
    fn part1(input: &Self::Input) -> aoc::Status<Self::Output> {
        let mut repeated = vec![];
        for (_, first, second) in input {
            let mut chars = HashMap::new();
            for c in first {
                chars.insert(*c, (true, false));
            }

            for c in second {
                chars.insert(
                    *c,
                    (chars.get(c).map(|tuple| tuple.0).unwrap_or_default(), true),
                );
            }

            for (c, n) in chars {
                if n == (true, true) {
                    repeated.push(score(c));
                }
            }
        }

        Status::Solved(repeated.iter().sum(), 7878)
    }
}
