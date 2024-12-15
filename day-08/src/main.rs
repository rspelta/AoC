#[derive(Debug,PartialEq,Clone)]
struct Antenna(i32, i32);

use std::collections::HashMap;

const SIZE_X : i32 = 50;
const SIZE_Y : i32 = 50;

fn read_map( text: &str) -> HashMap<char, Vec<Antenna>> 
{
    let mut map: HashMap<char, Vec<Antenna>> = HashMap::new();

    for (y, line) in text.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                ant   => {
                    if ant != '.' {
                        map.entry(ant).or_insert(Vec::new()).push( Antenna(x as i32, y as i32) );
                    }
                }
            }
        }
    }
    map
}

fn calc_antinodes_resonance( antennas: &Vec<Antenna>, antinodes: Vec<Antenna>) -> Vec<Antenna> {
    let mut anti = Vec::new();
    
    for source in antennas {
        for target in antennas {
            let mut r = 2;
            if target != source {
                let mut go = true;
                while go {
                    let x = source.0 + ( target.0 - source.0 ) * r;
                    let y = source.1 + ( target.1 - source.1 ) * r;

                    if x < 0 || x >= SIZE_X || y < 0 || y >= SIZE_Y { 
                        go = false;
                    } else {
                        if !antinodes.contains(&Antenna(x, y)) {
                            anti.push( Antenna(x, y) );
                        }
                        r += 1;
                    }
                }
            }
        }
    }
    anti
}


fn calc_antinodes( antennas: &Vec<Antenna>, antinodes: Vec<Antenna>) -> Vec<Antenna> {
    let mut anti = Vec::new();
    
    for source in antennas {
        for target in antennas {
            if target != source {
                let x = source.0 + ( target.0 - source.0 ) * 2;
                let y = source.1 + ( target.1 - source.1 ) * 2;

                if x < 0 || x >= SIZE_X || y < 0 || y >= SIZE_Y { 
                    continue; 
                }

                if !antinodes.contains(&Antenna(x, y)) {
                    anti.push( Antenna(x, y) );
                }
            }
        }
    }
    anti
}

fn remove_antennas( antennas: &Vec<Antenna>, antinodes: Vec<Antenna>) -> Vec<Antenna> { 

    let mut filtered_antinodes = antinodes;

    // Rimuovi le antenne che sono presenti in `antennas`
    //filtered_antinodes.retain(|x| !antennas.contains(x));

    // Filtra le antenne con coordinate valide
    filtered_antinodes.retain(|anti| anti.0 >= 0 && anti.1 >= 0 && anti.0 < SIZE_X && anti.1 < SIZE_Y);

    filtered_antinodes
}

fn part1() -> u32 {
    let text = include_str!("input_example.txt");
    let map = read_map(text);

    let mut anti = Vec::new();
    for (_, antennas) in &map {
        anti.extend( calc_antinodes(&antennas, anti.clone()) );
    }
/*
    for (_, antennas) in &map {
        anti = remove_antennas( antennas,  anti);
    }*/

    anti.len() as u32
}

fn part2() -> u32 {
    let text = include_str!("input.txt");
    let map = read_map(text);

    let mut anti = Vec::new();
    for (_, antennas) in &map {
        anti.extend( calc_antinodes_resonance(&antennas, anti.clone()) );
    }

    for (_, antenna) in &map {
        for ante in antenna {
            if !anti.contains(ante) {
                anti.push( ante.clone() );
            }
        }
    }
/*
    for (_, antennas) in &map {
        anti = remove_antennas( antennas,  anti);
    }*/

    //println!("{:?}", anti);

    anti.len() as u32
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
