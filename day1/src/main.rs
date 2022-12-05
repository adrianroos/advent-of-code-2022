use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    let mut cur = 0;
    let mut calories = Vec::<i32>::new();
    
    while let Ok(count) = stdin.read_line(&mut input) {
        let trimmed = input.trim_end();
        if (!trimmed.is_empty()) {
            cur += trimmed.parse::<i32>().expect("Bad input");
        } else {
            calories.push(cur);
            cur = 0;
        }
        input.clear();
        if count == 0 {
            break;
        }
    }
    
    calories.sort_by_key(|a| -a);
    let top_three: i32 = calories.into_iter().take(3).sum();

    println!("{top_three}");
}


fn part1() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    let mut cur = 0;
    let mut max = 0;
    
    
    while let Ok(count) = stdin.read_line(&mut input) {
        let trimmed = input.trim_end();
        if (!trimmed.is_empty()) {
            cur += trimmed.parse::<i32>().expect("Bad input");
        } else {
            max = i32::max(max, cur);
            cur = 0;
        }
        input.clear();
        if count == 0 {
            break;
        }
    }
    
    println!("{max}");
}