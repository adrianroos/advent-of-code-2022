use std::{io, str::{FromStr, from_utf8}, fmt::Debug};

fn eprintlnstacks(stacks: &Vec<Vec<u8>>) {
    for s in stacks {
        eprintln!("{}", from_utf8(s).unwrap())
    }
    eprintln!()
}

fn borrow_two_mut<'vec, 'a, 'b, T>(vec: &'vec mut Vec<T>, indexOne: usize, indexTwo: usize) -> (&'a mut T, &'b mut T) where 'vec : 'a, 'vec : 'b, T: Debug {
    assert!(indexOne != indexTwo);
    let (lower, upper) = vec.split_at_mut(
        usize::max(indexOne, indexTwo)
    );
    if indexOne < indexTwo {
        (&mut lower[indexOne], &mut upper[0])
    } else {
        (&mut upper[0], &mut lower[indexTwo])
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut stacks = Vec::<Vec::<u8>>::new();

    stdin.read_line(&mut input).unwrap();
    for _ in 0..(input.len() / 4) {
        stacks.push(Vec::new())
    }

    'outer: loop {
        for (stack, cargo) in stacks.iter_mut().zip(input.bytes().skip(1).step_by(4)) {
            if cargo.is_ascii_digit() {
                break 'outer;
            }
            if !cargo.is_ascii_whitespace() {
                stack.push(cargo);
            }
        }
        input.clear();
        stdin.read_line(&mut input).unwrap();
    }

    // Drop the empty line
    stdin.read_line(&mut input).unwrap();
    input.clear();

    for stack in &mut stacks {
        stack.reverse();
    }

    eprintlnstacks(&stacks);

    while stdin.read_line(&mut input).unwrap() != 0 {
        let words = input.trim().split(' ').take(6).collect::<Vec::<_>>();
        // eprintln!("{words:?}");
        let count = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;

        let (from, to) = borrow_two_mut(&mut stacks, from, to);
        
        cratemover9001(count, from, to);
        
        eprintlnstacks(&stacks);

        input.clear();
    }

    eprintlnstacks(&stacks);

    for s in stacks {
        print!("{}", *s.last().unwrap() as char);
    }
    println!();
}


fn cratemover9000(count: usize, from: &mut Vec<u8>, to: &mut Vec<u8>) {
    for _ in 0..count {
        to.push(from.pop().unwrap());
    }
}

fn cratemover9001(count: usize, from: &mut Vec<u8>, to: &mut Vec<u8>) {
    let from_len = from.len();
    to.extend(from.drain(from_len - count ..));
}

