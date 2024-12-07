use std::collections::HashMap;

fn extract_numbers(text: &str) -> HashMap<i32, Vec<i32>> {
    let mut dizionario: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in text.lines() 
    {
        if let Some((parte1, parte2)) = line.split_once("|") {
            // Converte entrambe le parti in i32
            let key: i32 = parte1.parse().expect("Conversione a i32 fallita");
            let value: i32 = parte2.parse().expect("Conversione a i32 fallita");
    
            dizionario.entry(key).or_insert(Vec::new()).push(value);
        } else {
            break;
        }
    } 

    dizionario
}

fn get_line_pages( text: &str ) -> i32
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

fn part1() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;

    let dizionario = extract_numbers(text);

    let start_line_pages = get_line_pages(text);


    for (key, value) in dizionario {
        println!("key: {}, value: {:?} {}", key, value, start_line_pages);
        sum+=1;
    }

    sum
}

fn main() {
    println!("{}", part1());
    // println!("{}", part2());
}