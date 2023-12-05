use std::str::Lines;

use regex::Regex;

fn main() {
    let mut inputs = include_str!("../input").lines();

    let re_initial = Regex::new(r"\d+").unwrap();
    let seeds: Vec<Option<i64>> = re_initial
        .find_iter(inputs.next().unwrap())
        .map(|s| s.as_str().parse::<i64>().ok())
        .collect();

    inputs.next();
    inputs.next();

    let n = seeds.len();

    let mut soil: Vec<Option<i64>> = vec![None; n];
    let mut fert: Vec<Option<i64>> = vec![None; n];
    let mut water: Vec<Option<i64>> = vec![None; n];
    let mut light: Vec<Option<i64>> = vec![None; n];
    let mut temp: Vec<Option<i64>> = vec![None; n];
    let mut humid: Vec<Option<i64>> = vec![None; n];
    let mut loctn: Vec<Option<i64>> = vec![None; n];

    x_to_y(&mut inputs, seeds, &mut soil);
    inputs.next();
    x_to_y(&mut inputs, soil, &mut fert);
    inputs.next();
    x_to_y(&mut inputs, fert, &mut water);
    inputs.next();
    x_to_y(&mut inputs, water, &mut light);
    inputs.next();
    x_to_y(&mut inputs, light, &mut temp);
    inputs.next();
    x_to_y(&mut inputs, temp, &mut humid);
    inputs.next();
    x_to_y(&mut inputs, humid, &mut loctn);

    println!("{}", loctn.iter_mut().map(|n| n.unwrap()).min().unwrap());
}

fn x_to_y(inputs: &mut Lines, x: Vec<Option<i64>>, y: &mut Vec<Option<i64>>) {
    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let n = x.len();

    while let Some(inp) = inputs.next() {
        if inp == "" {
            break;
        }

        let cap = re.captures(inp).unwrap();

        let (dest, source, range) = (
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
            cap[3].parse::<i64>().unwrap(),
        );

        for i in 0..n {
            if source <= x[i].unwrap() && x[i].unwrap() < source + range {
                y[i] = Some(dest - source + x[i].unwrap());
            }
        }
    }
    for i in 0..n {
        if y[i] == None {
            y[i] = Some(x[i].unwrap());
        }
    }
}
