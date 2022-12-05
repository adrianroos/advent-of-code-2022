use std::io;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Outcome {
    Win, Loss, Draw
}

static ALL_SHAPES: [Shape; 3] = [Shape::Rock, Shape::Paper, Shape::Scissors];

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn beats(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn parse(c: char) -> Shape {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Unknown char: {c:?}")
        }
    }
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
    
    fn of(opponent: Shape, me: Shape) -> Outcome {
        if opponent.beats() == me {
            Outcome::Loss
        } else if me.beats() == opponent {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }

    fn parse(c: char) -> Outcome {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Unknown char: {c:?}")
        }
    }

    fn my_shape(self, opponent: Shape) -> Shape {
        ALL_SHAPES.iter()
            .filter(|&me| Outcome::of(opponent, *me) == self)
            .next()
            .unwrap()
            .to_owned()
    }

}

fn score(opponent: Shape, me: Shape) -> i32 {
    let outcome = Outcome::of(opponent, me);
    eprintln!("{outcome:?}");
    outcome.score() + me.score()
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    let mut acc = 0;
    
    while let Ok(count) = stdin.read_line(&mut input) {
        if count == 0 {
            break;
        }
        
        let mut chars = input.chars();
        let opponent = Shape::parse(chars.next().unwrap());
        chars.next();
        let outcome = Outcome::parse(chars.next().unwrap());
        let me = outcome.my_shape(opponent);

        acc += score(opponent, me);

        input.clear();
        
    }
    
    println!("{acc}");
}

fn part1() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    let mut acc = 0;
    
    while let Ok(count) = stdin.read_line(&mut input) {
        if count == 0 {
            break;
        }
        
        let mut chars = input.chars();
        let opponent = Shape::parse(chars.next().unwrap());
        chars.next();
        let me = Shape::parse(chars.next().unwrap());

        acc += score(opponent, me);

        input.clear();
        
    }
    
    println!("{acc}");
}
