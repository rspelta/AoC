trait Calc {
    fn fill_region(&mut self, x : i32, y : i32, garden : &mut Vec<Vec<char>> );
    fn calc_perimeter( &self ) -> u32;
    fn extract_sides( &self ) -> Vec<Point>;
    fn calc_num_sides( &self ) -> u32;
    fn triplicate(&mut self);
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
    plots3: Vec<Point>,
    sides: Vec<Point>,
    area: u32,
    perimeter: u32,
    num_sides: u32,
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

fn triple( point : &Point ) -> Vec<Point> {
    let mut points = Vec::new();

    let y_max = point.y * 3 + 2;
    let x_max = point.x * 3 + 2;

    for y in y_max-2..=y_max {
        for x in x_max-2..=x_max {
            points.push( Point { x: x, y: y } );
        }
    }
    points
}

impl Calc for Region {

    fn triplicate(&mut self) {
        for point in &self.plots {
            self.plots3.append( &mut triple(point) );
        }
    }

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

    fn calc_num_sides( &self ) -> u32 {
        let mut sides = 0;
        let max_x = self.sides.iter().max_by_key(|p| p.x ).unwrap().x;
        let max_y = self.sides.iter().max_by_key(|p| p.y ).unwrap().y;
        let min_x = self.sides.iter().min_by_key(|p| p.x ).unwrap().x;
        let min_y = self.sides.iter().min_by_key(|p| p.y ).unwrap().y;

        for y in min_y..=max_y {
            let mut points = 0;
            for x in min_x..=max_x+1 {
                if self.sides.contains( &Point { x: x, y: y } ) {
                    points += 1;
                } else {
                    if points > 1 {
                        sides += 1;
                    }
                    points = 0;
                }
            }
        }

        for x in min_x..=max_x {
            let mut points = 0;
            for y in min_y..=max_y+1 {
                if self.sides.contains( &Point { x: x, y: y } ) {
                    points += 1;
                } else {
                    if points > 1 {
                        sides += 1;
                    }
                    points = 0;
                }
            }
        }
/*
        if self.plots.len() > 1 {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if self.sides.contains( &Point { x: x, y: y } ) {
                        if !self.sides.contains( &Point { x: x + 1, y } ) &&
                        !self.sides.contains( &Point { x: x - 1, y } ) &&
                        !self.sides.contains( &Point { x: x, y: y + 1 } ) &&
                        !self.sides.contains( &Point { x: x, y: y - 1 } ) {
                            sides += 4;
                        }
                    }
                }
            }
        }
*/
        sides
    }

    fn extract_sides( &self ) -> Vec<Point> {
        let mut perimeter = Vec::new();

        for plot in &self.plots3 {
            // sx
            if !self.plots3.contains( &Point { x: plot.x + 1, y: plot.y } ) {
                perimeter.push( Point { x: plot.x + 1, y: plot.y } );
            }
            // dx
            if !self.plots3.contains( &Point { x: plot.x - 1, y: plot.y } ) {
                perimeter.push( Point { x: plot.x - 1, y: plot.y } );
            }
            // down
            if !self.plots3.contains( &Point { x: plot.x, y: plot.y + 1 } ) {
                perimeter.push( Point { x: plot.x, y: plot.y + 1 } );
            }
            // up
            if !self.plots3.contains( &Point { x: plot.x, y: plot.y - 1 } ) {
                perimeter.push( Point { x: plot.x, y: plot.y - 1 } );
            }
            // down - dx
            if !self.plots3.contains( &Point { x: plot.x + 1, y: plot.y + 1 } ) {
                perimeter.push( Point { x: plot.x + 1, y: plot.y + 1 } );
            }
            // down - sx
            if !self.plots3.contains( &Point { x: plot.x - 1, y: plot.y + 1 } ) {
                perimeter.push( Point { x: plot.x - 1, y: plot.y + 1 } );
            }
            // up - sx
            if !self.plots3.contains( &Point { x: plot.x - 1, y: plot.y - 1 } ) {
                perimeter.push( Point { x: plot.x - 1, y: plot.y - 1 } );
            }
            // up - dx
            if !self.plots3.contains( &Point { x: plot.x + 1, y: plot.y - 1 } ) {
                perimeter.push( Point { x: plot.x + 1, y: plot.y - 1 } );
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
                let mut region = Region { plant_type: garden[y][x], plots: Vec::new(), plots3: Vec::new(), area: 0, perimeter: 0, num_sides: 0, sides: Vec::new() };
                region.fill_region(x as i32, y as i32, garden );

                region.triplicate();
                region.sides.append( &mut region.extract_sides() );
                region.perimeter = region.calc_perimeter();
                region.num_sides = region.calc_num_sides();
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

fn part2() -> u64 {
    let mut sum = 0;

    let mut garden = get_garden();

    let regions = get_regions(&mut garden);

    for region in regions {
        //println!("{} * {} = {}", region.area, region.num_sides, region.area * region.num_sides);
        sum += region.area as u64 * region.num_sides as u64;
    }
    sum
}

fn main() {
    println!("{}", part1() );
    println!("{}", part2() );
}
