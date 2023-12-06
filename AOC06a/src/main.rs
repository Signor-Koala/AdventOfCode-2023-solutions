use regex::Regex;

fn main() {
    let mut inputs = include_str!("../input").lines();
    let re = Regex::new(r"(\d+)").unwrap();
    let mut x_vec = vec![];
    for cap in re.find_iter(inputs.next().unwrap()).map(|c| c.as_str()) {
        x_vec.push(cap.parse::<i32>().unwrap());
    }
    let mut y_vec = vec![];
    for cap in re.find_iter(inputs.next().unwrap()).map(|c| c.as_str()) {
        y_vec.push(cap.parse::<i32>().unwrap());
    }
    let mut ans = 1;
    for (x, y) in x_vec.iter().zip(y_vec.iter()) {
        for a in 1..(x / 2) {
            if a * x - a * a > *y {
                ans = ans * (x - 2 * a + 1);
                break;
            }
        }
    }
    println!("{}", ans);
}
