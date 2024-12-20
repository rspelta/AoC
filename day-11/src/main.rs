

fn step( stones : &mut Vec<u64> ) {
    let mut index = 0;
    let mut len = stones.len();

    while index < len {

        let num = stones[index];
        let num_len = num.to_string().len();

        if num == 0 {  // se 0 diventa 1
            stones[index] = 1;
        } else if num_len % 2 == 0 { // se è pari viene splittato in 2
            let str_num = num.to_string();
            let left = str_num.split_at(num_len / 2).0.parse::<u64>().unwrap();
            let right = str_num.split_at(num_len / 2).1.parse::<u64>().unwrap();
            stones[index] = left;
            stones.insert(index + 1, right);
            len += 1;
            index += 1;
        } else {
            stones[index] = stones[index] * 2024;
        }

        index += 1;
    }
}

fn part2() -> usize {


    let mut stones: Vec<u64> = include_str!("input.txt")
        .split_whitespace() // Divide la stringa usando gli spazi
        .filter_map(|s| s.parse::<u64>().ok()) // Converte in i32, ignorando errori
        .collect();

    for _ in 0..75 {
        //println!("{:?}", stones);
        step(&mut stones);
    }

    stones.len()
}

fn part1() -> usize {


    let mut stones: Vec<u64> = include_str!("input.txt")
        .split_whitespace() // Divide la stringa usando gli spazi
        .filter_map(|s| s.parse::<u64>().ok()) // Converte in i32, ignorando errori
        .collect();

    for _ in 0..25 {
        //println!("{:?}", stones);
        step(&mut stones);
    }

    stones.len()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
