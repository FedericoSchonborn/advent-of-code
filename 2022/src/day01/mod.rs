use aoc::{Day, Status};

pub struct Day01;

impl Day for Day01 {
    const NUMBER: u8 = 1;
    const TITLE: &'static str = "Calorie Counting";
    const INPUT: &'static str = include_str!("input.txt");
    type Input = Vec<usize>;
    type Output = usize;

    fn parse(raw: &'static str) -> Self::Input {
        let elves = raw.split("\n\n").collect::<Vec<_>>();

        let mut totals = vec![];
        for elf in elves {
            let items = elf.split('\n').collect::<Vec<_>>();

            let mut total = 0;
            for item in items {
                let calories = item.parse::<usize>().unwrap();
                total += calories
            }

            totals.push(total);
        }

        totals.sort();
        totals.reverse();
        totals
    }

    fn part1(input: &Self::Input) -> Status<Self::Output> {
        Status::Solved(*input.iter().max().unwrap())
    }

    fn part2(input: &Self::Input) -> Status<Self::Output> {
        Status::Solved(input.iter().take(3).sum())
    }
}
