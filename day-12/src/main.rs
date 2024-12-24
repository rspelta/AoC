trait Calc {
    fn fill_region(&self, x : u32, y : u32, garden : &mut Vec<Vec<char>> );
}

struct Point {
    x: i32,
    y: i32,
}

struct Region {
    // String is a struct
    plant_type: char,
    plots: Vec<Point>,
    area: u32,
    perimeter: i32,
}

impl Calc for Region {
    fn fill_region(&self, x : u32, y : u32, garden : &mut Vec<Vec<char>> ) {
    }
}

fn get_garden() -> Vec<Vec<char>> {
    let text = include_str!("input.txt");
    let mut garden: Vec<Vec<char>> = Vec::new();
    
    for line in text.lines() {
        let row: Vec<char> = line.chars().map(|ch| ch ).collect();
        garden.push(row);
    }
    garden
}

fn get_regions(garden: &mut Vec<Vec<char>>) -> Vec<Region> {

    let mut regions: Vec<Region> = Vec::new();

    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if garden[y][x] != '.' {
                let region = Region { plant_type: garden[y][x], plots: Vec::new(), area: 0, perimeter: 0 };
                region.fill_region(x as u32, y as u32, garden );
                regions.push(region);
            }
        }
    }
    regions
}

fn part1() -> u64 {

    let mut garden = get_garden();

    let regions = get_regions(&mut garden);
    0
}

fn main() {
    println!("{}", part1() );
}
