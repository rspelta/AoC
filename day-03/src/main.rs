use regex::Regex;

fn part1() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;

    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Itera su tutte le corrispondenze della regex
    for captures in re_mul.captures_iter(text) {
        // Estrai i numeri come stringhe e convertili in i32
        if let (Some(first), Some(second)) = (captures.get(1), captures.get(2)) {
            let first_number = first.as_str().parse::<i32>().unwrap();
            let second_number = second.as_str().parse::<i32>().unwrap();
            sum += first_number * second_number;
        }
    }
    sum
}

fn main() {

    println!("{}", part1());
}