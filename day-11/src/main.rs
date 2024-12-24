use std::collections::HashMap;

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

fn split_number( value : u64 ) -> (u64, u64) {
    let str_num = value.to_string();
    let left = str_num.split_at(str_num.len() >> 1).0.parse::<u64>().unwrap();
    let right = str_num.split_at(str_num.len() >> 1).1.parse::<u64>().unwrap();
    (left, right)
}


fn foo( value : u64, step : u8, memo : &mut HashMap<(u64,u8), u64> ) -> u64
{
    if memo.contains_key(&(value, step)) {
        return memo[&(value, step)];
    }

    if step == 75 {
        memo.insert((value, step), 1);
        return 1;
    }

    if value == 0 {
       let ret = foo( 1, step+1, memo );
       memo.insert((value, step), ret);
       return ret;
    } else if value.to_string().len() % 2 == 0 {
        let (left, right) = split_number( value );
        let ret = foo( left, step+1, memo ) + foo( right, step+1, memo );
        memo.insert((value, step), ret);
        return ret;
    } else {
        let ret = foo( value * 2024, step+1, memo );
        memo.insert((value, step), ret);
        return ret;
    }
}

fn part2() -> u64 {
    let mut sum = 0;
    let mut memo = HashMap::new();

    let stones: Vec<u64> = include_str!("input.txt")
        .split_whitespace() // Divide la stringa usando gli spazi
        .filter_map(|s| s.parse::<u64>().ok()) // Converte in i32, ignorando errori
        .collect();

    for stone in stones {
        sum += foo( stone, 0, &mut memo );
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
