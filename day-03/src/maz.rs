use regex::Regex;

fn main() {
    part1_maz();
}

fn part1_maz() {
    let re: Regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let file_txt = include_str!("input.txt");

    let mut res : i32 = 0;
    let mut compute: bool = true;

    for val in re.find_iter(&file_txt) {
        match val.as_str() {
            "do()" => {compute = true; continue},
            "don't()" => {compute = false; continue;},
            &_ => {
                if compute {
                    let vals: Vec<&str> = val.as_str().trim_start_matches("mul(").trim_end_matches(")").split(",").collect();
            
                    let x: i32 = vals[0].parse().unwrap();
                    let y: i32 = vals[1].parse().unwrap();
            
                    res += x*y
                }
            },
        }
    }

    println!("{}", res)
}