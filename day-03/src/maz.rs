use regex::Regex;

fn main() {
    part1_maz();
}

fn part1_maz() {
    let re: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let file_txt = include_str!("input.txt");

    let mut res : i32 = 0;

    for val in re.find_iter(&file_txt) {
        let vals: Vec<&str> = val.as_str().trim_start_matches("mul(").trim_end_matches(")").split(",").collect();
        
        let x: i32 = vals[0].parse().unwrap();
        let y: i32 = vals[1].parse().unwrap();

        res += x*y
    }

    println!("{}", res)
}