fn main() {
    let inputs = include_str!("../input").lines();
    let mut sum = 0;
    for input in inputs {
        let chars = input.chars().collect::<Vec<char>>();
        for c in chars {
            if c.is_ascii_digit() {
                sum = sum + 10 * c.to_digit(10).unwrap();
                break;
            }
        }
        let rev = input.chars().rev().collect::<Vec<char>>();
        for c in rev {
            if c.is_ascii_digit() {
                if c.is_ascii_digit() {
                    sum = sum + c.to_digit(10).unwrap();
                    break;
                }
            }
        }
    }
    println!("{}", sum);
}
