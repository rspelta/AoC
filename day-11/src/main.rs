//use memoize::memoize;
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

fn count_digits(mut n: u64) -> u64 {
    if n == 0 {
        return 1; // Se il numero è 0, ha 1 cifra
    }

    let mut count = 0;
    while n > 0 {
        n /= 10; // Rimuove l'ultima cifra
        count += 1; // Incrementa il conteggio
    }
    count
}

//#[memoize]
fn foo( value : u64, mut step : u8, counter : &mut u64, memo : &mut HashMap<u64, Vec<u64>> )
{
    step += 1;

    if step == 76 {
        *counter+=1;
        return;
    }

    if let Some(result) = memo.get(&value).cloned() {
        for res in result {
            foo( res, step, counter, memo );
        }
        return;
    }

    let num_len: usize = count_digits(value) as usize;

    if value == 0 {
        memo.insert(value, Vec::from([1]));
        foo( 1, step, counter, memo );
    } else if num_len % 2 == 0 {
        let str_num = value.to_string();
        let left = str_num.split_at(num_len >> 1).0.parse::<u64>().unwrap();
        let right = str_num.split_at(num_len >> 1).1.parse::<u64>().unwrap();
        memo.insert(value, Vec::from([left, right]));
        foo( left, step, counter, memo );
        foo( right, step, counter, memo );
    } else {
        memo.insert(value, Vec::from([value*2024]));
        foo( value * 2024, step, counter, memo );
    }
}

fn part2() -> u64 {
    let mut memo = HashMap::new();
    let mut sum = 0;

    let stones: Vec<u64> = include_str!("input.txt")
        .split_whitespace() // Divide la stringa usando gli spazi
        .filter_map(|s| s.parse::<u64>().ok()) // Converte in i32, ignorando errori
        .collect();

    for stone in stones {
        let mut counter = 0;
        foo( stone, 0, &mut counter, &mut memo );
        sum += counter;
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
