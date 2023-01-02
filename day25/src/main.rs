use std::{
    fmt::Display,
    io::{self, BufRead},
    iter::Sum,
    mem::take,
    ops::AddAssign,
};

type Parsed = Vec<Snafu>;

#[derive(Debug, Default, PartialEq, Eq)]
struct Snafu {
    digits: Vec<i8>,
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.digits.iter().rev() {
            write!(
                f,
                "{}",
                match d {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    other => panic!("{other}"),
                }
            )?
        }
        Ok(())
    }
}

impl Snafu {
    fn parse(s: &str) -> Snafu {
        let digits = s
            .chars()
            .rev()
            .map(|c| match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                c => panic!("{c}"),
            })
            .collect::<Vec<_>>();
        Snafu { digits }
    }
}

impl AddAssign<&Self> for Snafu {
    fn add_assign(&mut self, rhs: &Self) {
        let mut carry = 0;
        let mut idx = 0;

        while idx < rhs.digits.len() || carry != 0 {
            if self.digits.len() <= idx {
                self.digits.push(0);
            }
            let mut d = take(&mut carry) + self.digits[idx] + rhs.digits.get(idx).unwrap_or(&0);
            if d > 2 {
                d -= 5;
                carry = 1;
            } else if d < -2 {
                d += 5;
                carry = -1;
            }
            self.digits[idx] = d;
            idx += 1;
        }
    }
}

impl<'a> Sum<&'a Snafu> for Snafu {
    fn sum<I: Iterator<Item = &'a Snafu>>(iter: I) -> Self {
        let mut acc = Snafu::default();
        for it in iter {
            acc += it;
        }
        acc
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let mut result = Parsed::new();
    for line in lines {
        let line: &str = &*line;
        result.push(Snafu::parse(line));
    }
    result
}

fn part1(parsed: &Parsed) -> Snafu {
    parsed.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), Snafu::parse("2=-1=0"));
    }
}
