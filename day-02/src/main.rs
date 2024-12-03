
fn is_incremental(vec: &Vec<i32> ) -> bool {
    for i in 0..vec.len() - 1 {
        if vec[i + 1] < vec[i] {
            return false;
        }
    }
    return true;
}

fn is_decremental( vec: &Vec<i32> ) -> bool {
    for i in 0..vec.len() - 1 {
        if vec[i + 1] > vec[i] {
            return false;
        }
    }
    return true;
}

fn is_in_range(vec: &Vec<i32>, min: i32, max: i32) -> bool {
    for i in 0..vec.len()-1 {

        let diff = (vec[i+1] - vec[i]).abs();
 
        if diff < min || diff > max {
            return false
        }
    }
 
    true
}

fn part1() -> i32 {

    let mut counter : i32 = 0;
    // Leggi il contenuto del file
    let file_txt = include_str!("./input.txt");

    let mut vec_line: Vec<i32> = Vec::new();

    // Stampa il contenuto
    for riga in file_txt.lines() {
        let parti = riga.split_whitespace();

        for first in parti {
            if let Ok(i) = first.parse::<i32>() {
                vec_line.push(i);
            }
        }

        let is_inc = is_incremental(&vec_line);
        let is_dec = is_decremental(&vec_line);
        let is_range = is_in_range(&vec_line, 1, 3);

        if (is_dec || is_inc) && is_range  {
            counter+=1
        }

        vec_line.clear();
    }
    counter

}

fn is_safe( vec: &Vec<i32> ) -> bool {
    (is_incremental(vec) || is_decremental(vec)) && is_in_range(vec, 1, 3)
}


fn part2() -> i32 {

    let mut counter : i32 = 0;
    // Leggi il contenuto del file
    let file_txt = include_str!("./input.txt");

    let mut vec_line: Vec<i32> = Vec::new();

    // Stampa il contenuto
    for riga in file_txt.lines() {
        let parti = riga.split_whitespace();

        for first in parti {
            if let Ok(i) = first.parse::<i32>() {
                vec_line.push(i);
            }
        }

        if is_safe(&vec_line) { 
            counter+=1;
        }else{
            
            for i in 0..vec_line.len() {
                let mut vec_clone = vec_line.clone();
                vec_clone.remove(i);
                if is_safe(&vec_clone) { 
                    counter+=1;
                    break;
                }
            }
        }

        vec_line.clear();
    }
    counter

}

fn main() {
    println!("Part 1: {}", part1() );
    println!("Part 2: {}", part2() );
}
