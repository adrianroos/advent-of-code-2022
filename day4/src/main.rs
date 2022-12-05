use std::{io, str::FromStr};

struct Section(i32, i32);

impl Section {
    fn fully_contains(&self, other: &Section) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }
    fn overlaps_with(&self, other: &Section) -> bool {
        !(self.1 < other.0 || other.1 < self.0)
    }
}

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        let start = start.parse::<i32>().unwrap();
        let end = end.parse::<i32>().unwrap();
        Ok(Section(start, end))
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    let mut acc = 0;

    while stdin.read_line(&mut input).unwrap() != 0 {
        let trimmed = input.trim_end();
        let (first, second) = trimmed.split_once(',').unwrap();
        let first: Section = first.parse().unwrap();
        let second: Section = second.parse().unwrap();
        if first.overlaps_with(&second) {
            acc += 1;
        }
        input.clear();
    }
    
    println!("{acc}");
}

fn part1() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    let mut acc = 0;

    while stdin.read_line(&mut input).unwrap() != 0 {
        let trimmed = input.trim_end();
        let (first, second) = trimmed.split_once(',').unwrap();
        let first: Section = first.parse().unwrap();
        let second: Section = second.parse().unwrap();
        if first.fully_contains(&second) || second.fully_contains(&first) {
            acc += 1;
        }
        input.clear();
    }
    
    println!("{acc}");
}
