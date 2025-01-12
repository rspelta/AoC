use regex::Regex;

const MAP_X: i32 = 101;
const MAP_Y: i32 = 103;

#[derive(Debug,PartialEq,Clone)]
struct Points {
    x: i32,
    y: i32,
}

#[derive(Debug,PartialEq,Clone)]
struct Robot {
    position: Points,
    speed: Points,
}

#[derive(Debug,PartialEq,Clone)]
struct Map {
    robots: Vec<Robot>,
}

trait Move {
    fn step1s( &mut self );
}

impl Move for Robot {
    fn step1s( &mut self ) {
        self.position.x = self.position.x + self.speed.x;
        self.position.y = self.position.y + self.speed.y;

        if self.position.x >= MAP_X {
            self.position.x -= MAP_X;
        }
        if self.position.x < 0 {
            self.position.x = MAP_X - self.position.x.abs();
        }
        if self.position.y >= MAP_Y {
            self.position.y -= MAP_Y;
        }
        if self.position.y < 0 {
            self.position.y = MAP_Y - self.position.y.abs();
        }
    }
}

fn create_robots( text : &str ) -> Vec<Robot> {
    let re = Regex::new(r"[+-]?\d+").unwrap();
    let mut robots : Vec<Robot> = Vec::new();
    
    for line in text.lines() {
        let numbers: Vec<i32> = re
        .find_iter(line)                         // Trova tutte le corrispondenze
        .filter_map(|mat| mat.as_str().parse::<i32>().ok()) // Converte le stringhe numeriche in i32
        .collect();

        robots.push( Robot{
            position: Points{ x: numbers[0], y: numbers[1] },
            speed: Points{ x: numbers[2], y: numbers[3] },
        } );
    }
    robots
}

fn show_map( robots : Vec<Robot> ) {
    for y in 0..MAP_Y {
        for x in 0..MAP_X {
            let mut robot = false;
            for r in &robots {
                if r.position.x == x && r.position.y == y {
                    robot = true;
                    break;
                }
            }
            if robot {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn sum_area( robots : &Vec<Robot>, x1 : i32, y1 : i32, x2 : i32, y2 : i32 ) -> u64 {
    let mut sum = 0;

    for r in robots {
        if r.position.x >= x1 && r.position.x <= x2 && r.position.y >= y1 && r.position.y <= y2 {
            sum+=1;
        }
    }
    sum
}

fn part1() -> u64 {
    let text = include_str!("input.txt");

    let mut robots = create_robots(text);

    println!("{:?}", robots);

    for _ in 0..100 {        
        for i in 0..robots.len() {
            robots[i].step1s();
        }
    }

    //show_map(robots);

    sum_area( &robots, 0, 0, (MAP_X/2)-1, (MAP_Y/2)-1) * sum_area( &robots, (MAP_X/2)+1, 0, MAP_X, (MAP_Y/2)-1) * sum_area( &robots, 0, (MAP_Y/2)+1, (MAP_X/2)-1, MAP_Y) * sum_area( &robots, (MAP_X/2)+1, (MAP_Y/2)+1, MAP_X, MAP_Y)
}

fn main() {

    println!("{}", part1() );
    //println!("{}", part2() );
}
