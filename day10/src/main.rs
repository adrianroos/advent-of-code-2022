use std::{
    io::{self, BufRead},
    ops::Deref,
};

#[derive(Debug, Copy, Clone)]
enum Op {
    Noop,
    Addx(i32),
}

impl Op {
    fn cycles(self) -> i32 {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }

    fn op(self, reg: &mut i32) {
        match self {
            Op::Noop => (),
            Op::Addx(c) => *reg += c,
        }
    }
}

type Parsed = Vec<Op>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: \n{}", part2(&map));
}

fn parse(lines: impl Iterator<Item = impl Deref<Target = str>>) -> Parsed {
    let mut result = Parsed::new();
    for line in lines {
        let line: &str = line.trim();
        if line == "noop" {
            result.push(Op::Noop);
        } else if let Some(rest) = line.strip_prefix("addx ") {
            result.push(Op::Addx(rest.parse::<i32>().unwrap()))
        } else {
            panic!("Unexpected: {line}");
        }
    }
    result
}

fn part1(parsed: &Parsed) -> i32 {
    let interesting = [20, 60, 100, 140, 180, 220];
    let mut cycle = 0;
    let mut reg = 1;
    let mut signal = 0;
    for op in parsed {
        for _ in 0..op.cycles() {
            cycle += 1;
            if interesting.contains(&cycle) {
                eprintln!("{cycle}: {reg} = {}", cycle * reg);
                signal += cycle * reg;
            }
        }
        op.op(&mut reg);
    }
    signal
}

fn part2(parsed: &Parsed) -> String {
    let mut cycle = 0;
    let mut reg = 1;
    let mut result = String::new();
    for op in parsed {
        for _ in 0..op.cycles() {
            if (cycle % 40i32 - reg).abs() <= 1 {
                result.push('#');
            } else {
                result.push('.');
            }
            cycle += 1;

            if (cycle % 40) == 0 {
                result.push('\n');
            }
        }
        op.op(&mut reg);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    fn sample() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT.lines().map(|x| x.trim())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 13140);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(
            part2(&root),
            "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n"
        );
    }
}
