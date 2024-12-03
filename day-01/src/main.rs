
use std::fs;
use std::io;

fn part1() -> Result<i32, String> {
    let file_path = "input1.txt";
    let file_txt : String;

    // Leggi il contenuto del file
    if let Err(file_txt) = fs::read_to_string(file_path) {
        return Err(file_txt.to_string());
    } else {
        file_txt = fs::read_to_string(file_path).unwrap();
    }

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Stampa il contenuto
    for riga in file_txt.lines() {
        let mut parti = riga.split_whitespace();

        if let Some(first) = parti.next() {
            if let Ok(i) = first.parse::<i32>() {
                left.push(i);
            }
        }

        if let Some(second) = parti.next() {
            if let Ok(i) = second.parse::<i32>() {
                right.push(i);
            }
        }
    }

    left.sort();
    right.sort();

    let mut sum:i32 = 0;

    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    Ok(sum)
}

fn part2() -> Result<i32, String> {
    let file_path = "input1.txt";
    let file_txt : String;

    // Leggi il contenuto del file
    if let Err(file_txt) = fs::read_to_string(file_path) {
        return Err(file_txt.to_string());
    } else {
        file_txt = fs::read_to_string(file_path).unwrap();
    }

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Stampa il contenuto
    for riga in file_txt.lines() {
        let mut parti = riga.split_whitespace();

        if let Some(first) = parti.next() {
            if let Ok(i) = first.parse::<i32>() {
                left.push(i);
            }
        }

        if let Some(second) = parti.next() {
            if let Ok(i) = second.parse::<i32>() {
                right.push(i);
            }
        }
    }

    left.sort();
    right.sort();

    let mut sum:i32 = 0;

    for num in &left {
        let count: i32 = right.iter().filter(|&x| x == num).count() as i32;
        sum += num*count;
    }
    Ok(sum)
}


fn main() -> io::Result<()> {
    // Specifica il percorso del file


    println!("{}", part1().unwrap());
    println!("{}", part2().unwrap());

    Ok(())
}

