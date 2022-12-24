use core::panic;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug, Clone)]
enum MonkeyOp {
    Literal(i64),
    Op(Op, String, String),
    Unknown,
}

#[derive(Debug, Clone)]
enum MonkeyOpTree {
    Literal(i64),
    Op(Op, Box<MonkeyOpTree>, Box<MonkeyOpTree>),
    Unknown,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Minus,
    Div,
    Mult,
    Eq,
}

type Parsed = HashMap<String, MonkeyOp>;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap());
    let map = parse(lines);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse(lines: impl Iterator<Item = String>) -> Parsed {
    let mut result = Parsed::new();
    for line in lines {
        let line: &str = &*line;
        let (name, monkeyop) = line.split_once(": ").expect(line);
        let monkeyop = if let Ok(c) = monkeyop.parse::<i64>() {
            MonkeyOp::Literal(c)
        } else {
            let parts = monkeyop.split_ascii_whitespace().collect::<Vec<_>>();
            let op = match parts[1] {
                "+" => Op::Plus,
                "-" => Op::Minus,
                "*" => Op::Mult,
                "/" => Op::Div,
                s => panic!("{s} in {line}"),
            };
            MonkeyOp::Op(op, parts[0].to_owned(), parts[2].to_owned())
        };
        result.insert(name.to_owned(), monkeyop);
    }
    result

    // lines
    //     .map(|line| {
    //         let line: &str = &*line;
    //         todo!();
    //     })
    //     .collect::<Parsed>()
}

fn eval(parsed: &Parsed, cache: &mut HashMap<String, i64>, monkey: &str) -> i64 {
    if let Some(&c) = cache.get(monkey) {
        return c;
    }
    let result = match &parsed[monkey] {
        MonkeyOp::Literal(c) => *c,
        MonkeyOp::Op(op, a, b) => {
            let a = eval(parsed, cache, &a);
            let b = eval(parsed, cache, &b);
            match op {
                Op::Plus => a + b,
                Op::Minus => a - b,
                Op::Div => a / b,
                Op::Mult => a * b,
                Op::Eq => todo!(),
            }
        }
        MonkeyOp::Unknown => todo!(),
    };
    cache.insert(monkey.to_owned(), result);
    result
}

fn eval2(parsed: &Parsed, monkey: &str) -> MonkeyOpTree {
    match &parsed[monkey] {
        MonkeyOp::Literal(c) => MonkeyOpTree::Literal(*c),
        MonkeyOp::Op(op, a, b) => {
            let a = eval2(parsed, &a);
            let b = eval2(parsed, &b);
            match (a, b) {
                (MonkeyOpTree::Literal(a), MonkeyOpTree::Literal(b)) => {
                    MonkeyOpTree::Literal(match op {
                        Op::Plus => a + b,
                        Op::Minus => a - b,
                        Op::Div => a / b,
                        Op::Mult => a * b,
                        Op::Eq => todo!(),
                    })
                }
                (a, b) => MonkeyOpTree::Op(*op, Box::new(a), Box::new(b)),
            }
        }
        MonkeyOp::Unknown => MonkeyOpTree::Unknown,
    }
}

fn chop(tree: MonkeyOpTree) -> Result<i64, MonkeyOpTree> {
    match tree {
        MonkeyOpTree::Literal(_) => todo!(),
        MonkeyOpTree::Unknown => todo!(),
        MonkeyOpTree::Op(op, a, b) => match op {
            Op::Eq => match (*a, *b) {
                (MonkeyOpTree::Literal(_), MonkeyOpTree::Literal(_)) => todo!(),
                (MonkeyOpTree::Literal(c), b) => chop(MonkeyOpTree::Op(
                    op,
                    Box::new(b),
                    Box::new(MonkeyOpTree::Literal(c)),
                )),
                (MonkeyOpTree::Unknown, MonkeyOpTree::Literal(c)) => Ok(c),

                (MonkeyOpTree::Op(op, a, b), MonkeyOpTree::Literal(c)) => match (op, *a, *b) {
                    (Op::Plus, MonkeyOpTree::Literal(d), b) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(b),
                        Box::new(MonkeyOpTree::Literal(c - d)),
                    )),
                    (Op::Plus, a, MonkeyOpTree::Literal(d)) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(a),
                        Box::new(MonkeyOpTree::Literal(c - d)),
                    )),
                    (Op::Minus, MonkeyOpTree::Literal(d), b) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(b),
                        Box::new(MonkeyOpTree::Literal(d - c)),
                    )),
                    (Op::Minus, a, MonkeyOpTree::Literal(d)) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(a),
                        Box::new(MonkeyOpTree::Literal(d + c)),
                    )),
                    (Op::Mult, MonkeyOpTree::Literal(d), b) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(b),
                        Box::new(MonkeyOpTree::Literal(c / d)),
                    )),
                    (Op::Mult, a, MonkeyOpTree::Literal(d)) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(a),
                        Box::new(MonkeyOpTree::Literal(c / d)),
                    )),
                    (Op::Div, MonkeyOpTree::Literal(d), b) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(b),
                        Box::new(MonkeyOpTree::Literal(d / c)),
                    )),
                    (Op::Div, a, MonkeyOpTree::Literal(d)) => Err(MonkeyOpTree::Op(
                        Op::Eq,
                        Box::new(a),
                        Box::new(MonkeyOpTree::Literal(d * c)),
                    )),
                    (op, a, b) => panic!("{op:?}, {a:?}, {b:?}"),
                },
                (_, _) => panic!("{op:?}"),
            },
            t => panic!("{t:?}"),
        },
    }
}

fn part1(parsed: &Parsed) -> i64 {
    let mut cache = HashMap::new();
    eval(parsed, &mut cache, "root")
}

fn part2(parsed: &Parsed) -> i64 {
    let mut parsed = parsed.clone();
    parsed.insert("humn".to_owned(), MonkeyOp::Unknown);
    if let MonkeyOp::Op(_, a, b) = parsed.remove("root").unwrap() {
        parsed.insert("root".to_owned(), MonkeyOp::Op(Op::Eq, a, b));
    } else {
        panic!("root not an op");
    }

    let mut x = Err(eval2(&parsed, "root"));
    while let Err(y) = x {
        x = chop(y);
    }
    x.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE_INPUT: &str = "\
    root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";

    fn sample() -> impl Iterator<Item = String> {
        SAMPLE_INPUT.lines().map(|x| x.trim().to_owned())
    }

    #[test]
    fn test_part1() {
        let root = parse(sample());
        assert_eq!(part1(&root), 152);
    }

    #[test]
    fn test_part2() {
        let root = parse(sample());
        assert_eq!(part2(&root), 301);
    }
}
