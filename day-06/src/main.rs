

fn part1() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;
    #[derive(Debug)]
    struct Location(i32, i32, char);

    let mut map = Vec::new();
    let mut start: Location = Location(0, 0, '#');

    for (y, line) in text.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '#' => {
                    map.push(Location(x as i32, y as i32, '#'));
                }
                '^' => {
                    start = Location(x as i32, y as i32, '^')
                }
                _ => {}

            }
        }
    }
    println!("map size: {:#?}, start: {:?}", map, start);
    sum
}

fn main() {
    println!("{}", part1());
    //println!("{}", part2());
}