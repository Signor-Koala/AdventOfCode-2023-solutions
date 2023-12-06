use regex::Regex;

fn main() {
    let mut inputs = include_str!("../input").lines();
    let re = Regex::new(r"([\d ]+)").unwrap();
    let mut x = 0;
    for cap in re.find_iter(inputs.next().unwrap()).map(|c| c.as_str()) {
        let cap = cap.replace(" ", "");
        x = cap.parse::<i64>().unwrap();
    }
    let mut y = 0;
    for cap in re.find_iter(inputs.next().unwrap()).map(|c| c.as_str()) {
        let cap = cap.replace(" ", "");
        y = cap.parse::<i64>().unwrap();
    }
    for a in 1..(x / 2) {
        if a * x - a * a > y {
            println!("{}", x - 2 * a + 1);
            break;
        }
    }
}
