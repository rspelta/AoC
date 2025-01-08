
#[derive(Debug,PartialEq,Clone)]
struct Points {
    x: i32,
    y: i32,
}

struct Robot {
    position: Points,
    speed: Points,
}

struct Map {
    robots: Vec<Robot>,
    map_size: Points,
}

trait Move {
    fn step1s( &self );
}

impl Move for Robot {
    fn step1s( &self ) {

    }
}

fn part1() -> u64 {

    let text = include_str!("input.txt");

    0
}

fn main() {
    println!("{}", part1() );
    //println!("{}", part2() );
}
