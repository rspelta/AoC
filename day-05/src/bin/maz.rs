use std::collections::HashMap;

fn main() {
    let ordering = load_ordering();

    let update_input = include_str!("../updates.maz.txt");
    let update_lines: Vec<&str> = update_input.lines().collect();

    let mut res: i32 = 0;

    for line in update_lines {
        let mut numbers: Vec<i32> = Vec::new();

        for n in line.split(",") {
            let value: i32 = n.parse().unwrap();
            numbers.push(value);
        }

        let mut is_valid = true;
        for (i, v) in numbers.iter().enumerate().rev() {
            if ordering.contains_key(v) {
                if !is_update_valid(numbers[..i].to_vec(), ordering.get(v).unwrap()) {
                    is_valid = false;
                    break
                }
            }
        }

        if is_valid {
            res += numbers[(numbers.len())/2];
        }
    }

    println!("{}", res);
}

fn is_update_valid(sub_update: Vec<i32>, numbers: &Vec<i32>) -> bool {
    !numbers.iter().any(|&x| sub_update.contains(&x))
}

fn load_ordering() -> HashMap<i32, Vec<i32>> {
    let text = include_str!("../ordering.maz.txt");
    let lines: Vec<&str> = text.lines().collect();

    let mut res: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in lines {
        let values: (&str, &str) = line.split_once("|").unwrap();
        let key: i32 = values.0.parse().unwrap();
        let value: i32 = values.1.parse().unwrap();

        // insert only if it does not already exist
        res.entry(key).or_insert(Vec::new()).push(value);
    }

    res
}