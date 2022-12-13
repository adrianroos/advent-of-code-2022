use core::panic;
use std::{
    collections::{HashSet},
    io::{self, BufRead},
    ops::Deref, mem::take,
};

#[derive(Clone, Copy)]
enum Op {
    Add(u64),
    Mult(u64),
    Square,
}

impl Op {
    fn op(self, level: u64) -> u64 {
        match self {
            Op::Add(c) => level + c,
            Op::Mult(c) => level * c,
            Op::Square => level * level,
        }
    }
}

impl Default for Op {
    fn default() -> Self {
        Self::Add(0)
    }
}

#[derive(Default)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    div: u64,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

type Parsed = Vec<Monkey>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let mut map = parse(lines);
    // println!("part1: {}", part1(&mut map));
    println!("part2: {}", part2(&mut map));
}

fn parse(lines: impl Iterator<Item = impl Deref<Target = str>>) -> Parsed {
    let mut result = Parsed::new();
    let mut monkey = Monkey::default();
    for line in lines {
        let line: &str = line.trim();
        if let Some(rest) = line.strip_prefix("Starting items: ") {
            monkey.items = rest.split(", ").map(|x|x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        } else if let Some(rest) = line.strip_prefix("Operation: new = ") {
            monkey.op = if rest == "old * old" {
                Op::Square
            } else if let Some(c) = rest.strip_prefix("old * ") {
                Op::Mult(c.parse().unwrap())
            } else if let Some(c) = rest.strip_prefix("old + ") {
                Op::Add(c.parse().unwrap())
            } else {
                panic!("Unexpected: {rest}");
            }
        } else if let Some(rest) = line.strip_prefix("Test: divisible by ") {
            monkey.div = rest.parse().unwrap();
        } else if let Some(rest) = line.strip_prefix("If true: throw to monkey ") {
            monkey.if_true = rest.parse().unwrap();
        } else if let Some(rest) = line.strip_prefix("If false: throw to monkey ") {
            monkey.if_false = rest.parse().unwrap();

            result.push(take(&mut monkey))
        }
    }
    result
}

fn part1(monkeys: &mut Parsed) -> usize {
    let mut monkeys = monkeys;
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];
            let mut items = take(&mut monkey.items);
            for item in &mut items {
                *item = monkey.op.op(*item) / 3;
            }
            monkey.inspections += items.len();
            let (div, if_true, if_false) = (monkey.div, monkey.if_true, monkey.if_false);
            drop(monkey);
            for item in items {
                let test = (item % div) == 0;
                let next = if test { if_true } else { if_false };
                monkeys[next].items.push(item);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|x| x.inspections).collect::<Vec<_>>();
    inspections.sort();
    inspections[inspections.len() - 2..].iter().product()
}

fn part2(monkeys: &mut Parsed) -> usize {
    let mut monkeys = monkeys;
    let prod: u64 = monkeys.iter().map(|x| x.div).product();
    eprintln!("prod: {prod}");
    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];
            let mut items = take(&mut monkey.items);
            for item in &mut items {
                *item = monkey.op.op(*item) % prod;
            }
            monkey.inspections += items.len();
            let (div, if_true, if_false) = (monkey.div, monkey.if_true, monkey.if_false);
            drop(monkey);
            for item in items {
                let test = (item % div) == 0;
                let next = if test { if_true } else { if_false };
                monkeys[next].items.push(item);
            }
        }
    }
    let mut inspections = monkeys.iter().map(|x| x.inspections).collect::<Vec<_>>();
    inspections.sort();
    inspections[inspections.len() - 2..].iter().product()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    fn sample() -> impl Iterator<Item = &'static str> {
        SAMPLE_INPUT.lines().map(|x| x.trim())
    }

    #[test]
    fn test_part1() {
        let mut root = parse(sample());
        assert_eq!(part1(&mut root), 10605);
    }

    #[test]
    fn test_part2() {
        let mut root = parse(sample());
        assert_eq!(part2(&mut root), 2713310158);
    }
}
