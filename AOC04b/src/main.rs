fn main() {
    let inputs: Vec<&str> = include_str!("../input").lines().collect();
    let mut number_of_cards = vec![1; inputs.len()];
    for i in 0..inputs.len() {
        let (_, inp) = inputs[i].split_once(':').unwrap();
        let (a, b) = inp.split_once("|").unwrap();
        let winnings: Vec<usize> = a
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        let numbers: Vec<usize> = b
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        let mut n = 0;
        for winning in winnings {
            if numbers.contains(&winning) {
                n = n + 1;
            }
        }
        for j in i + 1..n + i + 1 {
            number_of_cards[j] = number_of_cards[j] + number_of_cards[i]
        }
    }
    let sum: usize = number_of_cards.iter().sum();
    println!("{}", sum);
}
