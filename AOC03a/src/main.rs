use regex::Regex;
use std::collections::VecDeque;

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn main() {
    let inputs: Vec<&str> = include_str!("../input").lines().collect();
    let re = Regex::new(r"(\d+)").unwrap();
    let mut numbers = vec![];

    for input in &inputs {
        for (_, [cap_num]) in re.captures_iter(input).map(|c| c.extract()) {
            let x = cap_num.parse::<usize>().unwrap();
            numbers.push((x, x.checked_ilog10().unwrap_or(0) + 1));
        }
    }

    let (mut x, mut y) = (0, 0);
    let mut positions = VecDeque::new();
    for input in &inputs {
        for c in input.chars() {
            if c.is_digit(10) {
                positions.push_back((x, y));
            }
            x = x + 1;
        }
        x = 0;
        y = y + 1;
    }

    let mut num_w_pos = vec![];
    for num in numbers {
        let mut pos = vec![];
        for _ in 0..num.1 {
            let p = positions.pop_front().unwrap();
            pos.push(p);
        }
        num_w_pos.push((num.0, false, pos));
    }

    let (mut x, mut y): (i32, i32) = (0, 0);
    for input in inputs {
        for c in input.chars() {
            if !c.is_digit(10) && !(c == '.') {
                for (xo, yo) in OFFSETS {
                    for element in &mut num_w_pos {
                        if element.2.contains(&(x + xo, y + yo)) {
                            element.1 = true;
                        }
                    }
                }
            }
            x = x + 1;
        }
        x = 0;
        y = y + 1;
    }

    let mut sum = 0;
    for element in num_w_pos {
        if element.1 {
            sum = sum + element.0
        }
    }
    println!("{}", sum);
}
