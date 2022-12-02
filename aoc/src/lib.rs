use std::fmt::Display;

#[derive(Debug)]
pub enum Status<T> {
    Unsolved,
    Solved(T),
}

pub trait Day {
    const NUMBER: u8;
    const TITLE: &'static str;
    const INPUT: &'static str;

    type Input;
    type Output;

    fn parse(_raw: &'static str) -> Self::Input;

    fn part1(_input: &Self::Input) -> Status<Self::Output> {
        Status::Unsolved
    }

    fn part2(_input: &Self::Input) -> Status<Self::Output> {
        Status::Unsolved
    }
}

pub fn day<D>()
where
    D: Day,
    D::Output: Display,
{
    println!("Day {} \"{}\":", D::NUMBER, D::TITLE);

    let status_to_string = |status| match status {
        Status::Solved(result) => {
            format!("Solved! ({})", result)
        }
        Status::Unsolved => "Unsolved".to_string(),
    };

    let input = D::parse(D::INPUT);
    println!("  Part 1: {}", status_to_string(D::part1(&input)));
    println!("  Part 2: {}", status_to_string(D::part2(&input)));
}
