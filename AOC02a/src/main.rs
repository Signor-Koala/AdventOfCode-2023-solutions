use regex::Regex;

fn main() {
    let inputs = include_str!("../input").lines();

    let mut sum = 0;
    let mut iter = 0;
    'outer: for input in inputs {
        iter = iter + 1;
        let games = input.split_once(":").unwrap().1.split(";");
        for game in games {
            let re_r = Regex::new(r"(-?\d+) red").unwrap();
            let re_g = Regex::new(r"(-?\d+) green").unwrap();
            let re_b = Regex::new(r"(-?\d+) blue").unwrap();

            let cap_r = re_r.captures(game);
            let cap_g = re_g.captures(game);
            let cap_b = re_b.captures(game);

            if let Some(r) = cap_r {
                if r[1].parse::<i32>().unwrap() > 12 {
                    continue 'outer;
                }
            }
            if let Some(g) = cap_g {
                if g[1].parse::<i32>().unwrap() > 13 {
                    continue 'outer;
                }
            }
            if let Some(b) = cap_b {
                if b[1].parse::<i32>().unwrap() > 14 {
                    continue 'outer;
                }
            }
        }
        sum = sum + iter;
    }
    println!("{}", sum);
}
