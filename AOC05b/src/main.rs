use std::str::Lines;

use regex::Regex;

fn main() {
    let mut inputs = include_str!("../input").lines();

    let re = Regex::new(r"\d+ \d+").unwrap();
    let seed_inp: Vec<&str> = re
        .find_iter(inputs.next().unwrap())
        .map(|s| s.as_str())
        .collect();

    let mut seeds: Vec<(i64, i64)> = vec![];

    for s in seed_inp {
        let (lower, range) = s.split_once(" ").unwrap();
        seeds.push((lower.parse().unwrap(), range.parse().unwrap()));
    }

    inputs.next();
    inputs.next();
    let seed_soil = get_inputs(&mut inputs);
    let soil_fert = get_inputs(&mut inputs);
    let fert_water = get_inputs(&mut inputs);
    let water_light = get_inputs(&mut inputs);
    let light_temp = get_inputs(&mut inputs);
    let temp_humid = get_inputs(&mut inputs);
    let humid_loctn = get_inputs(&mut inputs);

    for i in 0.. {
        let temp = get_source(&i, &humid_loctn);
        let temp = get_source(&temp, &temp_humid);
        let temp = get_source(&temp, &light_temp);
        let temp = get_source(&temp, &water_light);
        let temp = get_source(&temp, &fert_water);
        let temp = get_source(&temp, &soil_fert);
        let temp = get_source(&temp, &seed_soil);

        for (seedl, seedr) in &seeds {
            if seedl <= &temp && temp < seedl + seedr {
                println!("{}", i);
                return;
            }
        }
    }
}

fn get_source(i: &i64, m: &Vec<(i64, i64, i64)>) -> i64 {
    for (d, s, r) in m {
        if d <= i && i < &(d + r) {
            return s - d + i;
        }
    }
    *i
}

fn get_inputs(inputs: &mut Lines) -> Vec<(i64, i64, i64)> {
    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut ranges = vec![];

    while let Some(s) = inputs.next() {
        if s == "" {
            break;
        }
        let cap = re.captures(s).unwrap();

        let e = (
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
            cap[3].parse::<i64>().unwrap(),
        );
        ranges.push(e);
    }
    inputs.next();
    ranges
}
