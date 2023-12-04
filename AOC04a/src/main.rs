fn main() {
    let inputs = include_str!("../input").lines();
    let mut sum = 0;
    for input in inputs {
        let (_, inp) = input.split_once(':').unwrap();
        let (a, b) = inp.split_once("|").unwrap();
        let winnings: Vec<usize> = a
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        let numbers: Vec<usize> = b
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        let mut i = 0;
        for winning in winnings {
            if numbers.contains(&winning) {
                if i == 0 {
                    i = 1;
                } else {
                    i = i * 2;
                }
            }
        }
        sum = sum + i;
    }
    println!("{}", sum);
}
