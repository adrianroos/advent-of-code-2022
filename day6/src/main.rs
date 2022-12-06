use core::panic;
use std::io::{self};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let packet_start = find_start(line.as_bytes(), 4);
    let message_start = find_start(line.as_bytes(), 14);

    println!("packet: {packet_start} message: {message_start}");
}

fn find_start(input: &[u8], size: usize) -> usize {
    let mut unique = 0;
    let mut counts = [0usize; u8::MAX as usize + 1];
    for i in 0..input.len() {
        {
            let x = input[i];
            let x_count = &mut counts[x as usize];
            if *x_count == 0 {
                unique += 1;
            }
            *x_count += 1;
        }

        if i >= size {
            let y = input[i - size];
            let y_count = &mut counts[y as usize];
            *y_count -= 1;
            if *y_count == 0 {
                unique -= 1;
            }
        }

        if unique == size {
            return i + 1;
        }
    }
    panic!("no packet start found")
}

#[cfg(test)]
mod test {
    use super::find_start;

    #[test]
    fn sample_part1() {
        assert_eq!(
            find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 4),
            7
        );
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 4), 5);
        assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 4), 6);
        assert_eq!(
            find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 4),
            10
        );
        assert_eq!(
            find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 4),
            11
        );
    }

    #[test]
    fn sample_part2() {
        assert_eq!(
            find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 14),
            19
        );
        assert_eq!(
            find_start("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 14),
            23
        );
        assert_eq!(
            find_start("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 14),
            23
        );
        assert_eq!(
            find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 14),
            29
        );
        assert_eq!(
            find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 14),
            26
        );
    }

    #[test]
    fn other() {
        assert_eq!(find_start("abcd".as_bytes(), 4), 4);
    }
}
