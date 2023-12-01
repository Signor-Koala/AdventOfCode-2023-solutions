fn main() {
    let numbers = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let inputs = include_str!("../input").lines();
    let mut sum = 0;
    for input in inputs {
        let mut first: (i32, i32) = (i32::MAX, -1);
        let mut last: (i32, i32) = (-1, -1);
        for i in 0..18 {
            let n = input.find(numbers[i]);
            if let Some(n) = n {
                if (n as i32) < first.0 {
                    first = (n as i32, i as i32);
                }
            }
            let m = input.rfind(numbers[i]);
            if let Some(m) = m {
                if (m as i32) > last.0 {
                    last = (m as i32, i as i32);
                }
            }
        }
        if first.1 >= 9 {
            sum = sum + (first.1 - 8) * 10;
        } else {
            sum = sum + (first.1 + 1) * 10;
        }
        if last.1 >= 9 {
            sum = sum + (last.1 - 8);
        } else {
            sum = sum + (last.1 + 1);
        }
    }
    println!("{}", sum);
}
