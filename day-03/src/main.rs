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

fn part2() -> i32 {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let file_txt = include_str!("input.txt");

    let mut res : i32 = 0;
    let mut _do = true;

    for val in re.find_iter(&file_txt) {
        let matched = val.as_str();

        match matched {
            s if s.starts_with("mul") => {
                if _do {
                    let vals: Vec<&str> = val.as_str().trim_start_matches("mul(").trim_end_matches(")").split(",").collect();
            
                    let x: i32 = vals[0].parse().unwrap();
                    let y: i32 = vals[1].parse().unwrap();
            
                    res += x*y
                }
            }
            "do()" => {
                _do = true
            }
            "don't()" => {
                _do = false
            }
            _ => {}
        }
    }

    res
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}