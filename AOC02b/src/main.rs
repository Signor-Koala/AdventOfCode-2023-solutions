use regex::Regex;

fn main() {
    let inputs = include_str!("../input").lines();

    let mut sum = 0;
    for input in inputs {
        let mut rgb = (0, 0, 0);
        let games = input.split_once(":").unwrap().1.split(";");
        for game in games {
            let re_r = Regex::new(r"(-?\d+) red").unwrap();
            let re_g = Regex::new(r"(-?\d+) green").unwrap();
            let re_b = Regex::new(r"(-?\d+) blue").unwrap();

            let cap_r = re_r.captures(game);
            let cap_g = re_g.captures(game);
            let cap_b = re_b.captures(game);

            if let Some(r) = cap_r {
                let red = r[1].parse::<i32>().unwrap();
                if red > rgb.0 {
                    rgb.0 = red
                }
            }
            if let Some(g) = cap_g {
                let green = g[1].parse::<i32>().unwrap();
                if green > rgb.1 {
                    rgb.1 = green
                }
            }
            if let Some(b) = cap_b {
                let blue = b[1].parse::<i32>().unwrap();
                if blue > rgb.2 {
                    rgb.2 = blue
                }
            }
        }
        sum = sum + rgb.0 * rgb.1 * rgb.2;
    }
    println!("{}", sum);
}
