use std::collections::HashMap;
use rand::seq::SliceRandom; // Importa il trait necessario
use rand::thread_rng;

fn extract_numbers(text: &str) -> HashMap<i32, Vec<i32>> {
    let mut dizionario: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in text.lines() 
    {
        if let Some((parte1, parte2)) = line.split_once("|") {
            // Converte entrambe le parti in i32
            let key: i32 = parte1.parse().unwrap();
            let value: i32 = parte2.parse().unwrap();
    
            dizionario.entry(key).or_insert(Vec::new()).push(value);
        } else {
            break;
        }
    } 

    dizionario
}

fn get_line_pages( text: &str ) -> usize
{
    let mut line_count = 0;

    for line in text.lines() {
        if line.is_empty() {
            return line_count + 1;
        }
        line_count += 1;
    }
    0
}

fn are_pages_valid(pages: &[i32], checklist: &Vec<i32>) -> bool {
    for page in pages {
        if checklist.contains(page) {
            return false;
        }
    }
    true
}

fn scramble( pages: &[i32] ) -> Vec<i32> {
    let mut new_pages: Vec<i32> = Vec::new();
    for page in pages {
        new_pages.push(*page);
    }
    new_pages.shuffle(&mut thread_rng());
    new_pages
}

fn part2() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;

    let dizionario = extract_numbers(text);

    let start_line_pages = get_line_pages(text);

    for line in text.lines().skip(start_line_pages) 
    {
        println!("Line: {:?} ", line);
        let mut is_valid;
        let mut pages: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();

        loop {
            is_valid = true;
            for (i, v) in pages.iter().enumerate().rev() { 
                if dizionario.contains_key(&v) {
                    let checklist = dizionario.get(&v).unwrap();
                    if !are_pages_valid( &pages[0..i], checklist ) {
                        is_valid = false;
                        break;
                    }
                }
            }
            if !is_valid {
                pages = scramble(&pages);
                println!("God wants: {:?} ", pages);
            }
            if is_valid {
                break;
            }
        }
        if is_valid {
            sum += pages[pages.len()/2]
        }
    }

    sum
}

fn part1() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;

    let dizionario = extract_numbers(text);

    let start_line_pages = get_line_pages(text);

    for line in text.lines().skip(start_line_pages) 
    {
        let mut is_valid = true;
        let pages: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();


        for (i, v) in pages.iter().enumerate().rev() { 
            if dizionario.contains_key(&v) {
                let checklist = dizionario.get(&v).unwrap();
                if !are_pages_valid( &pages[0..i], checklist ) {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            sum += pages[pages.len()/2]
        }
    }

/*
    for (key, value) in dizionario {
        println!("key: {}, value: {:?} {}", key, value, start_line_pages);
        sum+=1;
    }*/

    sum
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}