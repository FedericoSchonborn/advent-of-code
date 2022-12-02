use std::fmt::Display;

#[derive(Debug)]
pub enum Status<T> {
    Unsolved,
    Solving(T),
    Solved(T, T),
}

pub trait Day
where
    Self::Output: PartialEq,
{
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

    let status_to_string = |status: Status<D::Output>| match status {
        Status::Unsolved => "❔".to_string(),
        Status::Solving(result) => format!("⏳ (guess: {})", result),
        Status::Solved(guess, answer) => {
            if guess == answer {
                format!("✅ (answer: {answer})")
            } else {
                format!("❌ (guess: {guess}; answer: {answer})")
            }
        }
    };

    let input = D::parse(D::INPUT);
    println!("  Part 1: {}", status_to_string(D::part1(&input)));
    println!("  Part 2: {}", status_to_string(D::part2(&input)));
}
