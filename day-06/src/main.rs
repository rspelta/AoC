

#[derive(Debug)]
#[derive(PartialEq)]
struct Location(i32, i32);
struct Segment(i32, i32, i32, i32);

const SIZE_X : i32 = 10;
const SIZE_Y : i32 = 10;

enum Directions {
    Up,
    Right,
    Down,
    Left
}


fn read_map( text: &str) -> (Vec<Location>, Location) {

    let mut map = Vec::new();
    let mut start: Location = Location(0, 0);

    for (y, line) in text.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '#' => {
                    map.push(Location(x as i32, y as i32));
                }
                '^' => {
                    start = Location(x as i32, y as i32)
                }
                _ => {}

            }
        }
    }
    ( map, start)
}

fn go_up( pos: &Location, map: &Vec<Location>) -> Segment {
    let end = map.iter().filter(|loc| loc.0 == pos.0 && loc.1 < pos.1).min_by_key(|loc| (pos.1 - loc.1).abs());
    if end.is_some()  {
        return Segment(pos.0, pos.1, end.unwrap().0, end.unwrap().1+1);
    }
    Segment( pos.0, pos.1, pos.0, 0)
}

fn go_right( pos: &Location, map: &Vec<Location>) -> Segment {
    let end = map.iter().filter(|loc| loc.1 == pos.1 && loc.0 > pos.0 ).min_by_key(|loc| (pos.0 - loc.0).abs());
    if end.is_some()  {
        return Segment(pos.0, pos.1, end.unwrap().0-1, end.unwrap().1);
    }
    Segment( pos.0, pos.1, SIZE_X, pos.1)
}

fn go_down( pos: &Location, map: &Vec<Location>) -> Segment {
    let end = map.iter().filter(|loc| loc.0 == pos.0 && loc.1 > pos.1).min_by_key(|loc| (pos.1 - loc.1).abs() );
    if end.is_some()  {
        return Segment(pos.0, pos.1, end.unwrap().0, end.unwrap().1-1);
    }
    Segment( pos.0, pos.1, pos.0, SIZE_Y)
}

fn go_left( pos: &Location, map: &Vec<Location>) -> Segment {
    let end = map.iter().filter(|loc| loc.1 == pos.1 && loc.0 < pos.0).min_by_key(|loc| (pos.0 - loc.0).abs());
    if end.is_some()  {
        return Segment(pos.0, pos.1, end.unwrap().0+1, end.unwrap().1);
    }
    Segment( 0, pos.1, pos.0, pos.1)
}

fn is_border( pos: &Location) -> bool {
    pos.0 == 0 || pos.0 == SIZE_X || pos.1 == 0 || pos.1 == SIZE_Y
}

fn calc_path( start: &Location, map: &Vec<Location>) -> Vec<Segment> {

    let mut dir = Directions::Up;

    let mut path = Vec::new();
    let mut pos: Location = Location( start.0, start.1);
    let mut seg: Segment;

    loop {
        match dir {
            Directions::Up => {
                seg = go_up( &pos, &map );
                dir = Directions::Right;
            }
            Directions::Right => {
                seg = go_right( &pos, &map );
                dir = Directions::Down;
            }
            Directions::Down => {
                seg = go_down( &pos, &map );
                dir = Directions::Left;
            }
            Directions::Left => {
                seg = go_left( &pos, &map );
                dir = Directions::Up;
            }
        }
        pos = Location(seg.2, seg.3);
        path.push( seg );

        if is_border( &pos ) {
            break;
        }
    }
    return path;
}

/* 
fn calc_distance( path: &Vec<Segment>) -> i32 {
    path.iter().fold(0, |sum, seg| sum + (seg.0 - seg.2).abs() + (seg.1 - seg.3).abs())
}
*/
fn create_all_points( path: &Vec<Segment>) -> Vec<Location> {
    let mut all = Vec::new();

    for seg in path {
        if seg.0 == seg.2 {
            let min: i32 = if seg.1 < seg.3 { seg.1 } else { seg.3 };
            let max: i32 = if seg.1 < seg.3 { seg.3 } else { seg.1 };
            for y in min..max {
                if !all.contains(&Location(seg.0, y)) {
                    all.push(Location(seg.0, y));    
                }
            }
            continue;
        }
        if seg.1 == seg.3 {
            let min: i32 = if seg.0 < seg.2 { seg.0 } else { seg.2 };
            let max: i32 = if seg.0 < seg.2 { seg.2 } else { seg.0 };
            for x in min..max {
                if !all.contains(&Location(x, seg.1)) {
                    all.push(Location(x, seg.1));
                }
            }
            continue;
        }
    }
    println!("all: {:#?}", all);
    all
}

fn part1() -> i32 {
    let text = include_str!("./input_example.txt");

    let (map, start) = read_map(&text);

    let path = calc_path( &start, &map );

    //println!("map size: {:#?}, start: {:?}", map, start);
    create_all_points(&path).len() as i32
    //calc_distance( &path )
}

fn main() {
    println!("{}", part1());
    //println!("{}", part2());
}