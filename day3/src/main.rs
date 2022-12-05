use core::panic;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    let mut acc = 0;

    'outer: loop {
        let mut seen = [0u8; 256];
        for i in 0..=2 {
            if stdin.read_line(&mut input).unwrap() == 0 {
                break 'outer;
            }
            let trimmed = input.trim_end();
            for b in trimmed.bytes() {
                seen[b as usize] |= 1 << i;
            }
            input.clear();
        }

        let badge = seen.into_iter().position(|x| x == 7).unwrap() as u8;

        acc += match badge as char {
            'a'..='z' => badge - 'a' as u8 + 1,
            'A'..='Z' => badge - 'A' as u8 + 27,
            c => panic!("Unexpected: {c:?}"),
        } as u32;
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

        let trimmed = input.trim_end();
        let (head, tail) = trimmed.split_at(trimmed.len() / 2);
        let mut seen = [false; 256];
        for b in head.bytes() {
            seen[b as usize] = true;
        }
        let mut dupe = 0;
        for b in tail.bytes() {
            if seen[b as usize] {
                dupe = b;
                break;
            }
        }

        acc += match dupe as char {
            'a'..='z' => dupe - 'a' as u8 + 1,
            'A'..='Z' => dupe - 'A' as u8 + 27,
            c => panic!("Unexpected: {c:?}"),
        } as u32;

        input.clear();
    }

    println!("{acc}");
}
