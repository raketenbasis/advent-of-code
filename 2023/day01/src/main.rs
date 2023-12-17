fn calibration_value(input: &str) -> Option<u32> {
    let digits = input
        .chars()
        .into_iter()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let first = match digits.first() {
        Some(value) => value,
        None => return None,
    };

    let last = match digits.last() {
        Some(value) => value,
        None => return None,
    };

    return Some(first * 10 + last);
}

fn main() {
    let inputs = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    let mut sum = 0;
    for input in inputs {
        let value = match calibration_value(input) {
            Some(value) => value,
            None => {
                println!("{}: invalid", input);
                continue;
            }
        };

        println!("{}: {}", input, value);

        sum += value;
    }

    println!("Sum: {}", sum);
}
