trait Calc {
    fn fill_region(&mut self, x : i32, y : i32, garden : &mut Vec<Vec<char>> );
    fn calc_perimeter( &self ) -> u32;
}

#[derive(Debug,PartialEq,Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Region {
    // String is a struct
    plant_type: char,
    plots: Vec<Point>,
    area: u32,
    perimeter: u32,
}

fn is_valid( x : i32, y : i32, plant : char, garden : &mut Vec<Vec<char>> ) -> bool {
    if x >= garden[0].len() as i32 || y >= garden.len() as i32 {
        return false;
    }
    if x < 0 || y < 0 {
        return false;
    }
    if garden[y as usize][x as usize] == plant {
        return true;
    }

    false
}


impl Calc for Region {
    fn fill_region(&mut self, x : i32, y : i32, garden : &mut Vec<Vec<char>> ) {

        if !is_valid(x, y, self.plant_type, garden) {
            return;
        }
        garden[y as usize][x as usize] = '.';
        self.area += 1;
        self.plots.push( Point { x: x, y: y } );

        self.fill_region( x+1, y, garden );
        self.fill_region( x-1, y, garden );
        self.fill_region( x, y+1, garden );
        self.fill_region( x, y-1, garden );
    }

    fn calc_perimeter( &self ) -> u32 {
        let mut perimeter = 0;

        for plot in &self.plots {
            if !self.plots.contains( &Point { x: plot.x + 1, y: plot.y } ) {
                perimeter += 1;
            }
            if !self.plots.contains( &Point { x: plot.x - 1, y: plot.y } ) {
                perimeter += 1;
            }
            if !self.plots.contains( &Point { x: plot.x, y: plot.y + 1 } ) {
                perimeter += 1;
            }
            if !self.plots.contains( &Point { x: plot.x, y: plot.y - 1 } ) {
                perimeter += 1;                
            }
        }
        perimeter
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
                let mut region = Region { plant_type: garden[y][x], plots: Vec::new(), area: 0, perimeter: 0 };
                region.fill_region(x as i32, y as i32, garden );
                region.perimeter = region.calc_perimeter();
                regions.push(region);
            }
        }
    }
    regions
}

fn part1() -> u64 {
    let mut sum = 0;

    let mut garden = get_garden();

    let regions = get_regions(&mut garden);

    for region in regions {
        //println!("{} * {} = {}", region.area, region.perimeter, region.area * region.perimeter);
        sum += region.area as u64 * region.perimeter as u64;
    }
    sum
}

fn main() {
    println!("{}", part1() );
}
