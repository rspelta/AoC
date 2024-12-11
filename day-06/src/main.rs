



#[derive(Debug,PartialEq,Clone)]
struct Location(i32, i32);

#[derive(Debug,PartialEq,Clone)]
struct Segment(i32, i32, i32, i32);

//const SIZE_X : i32 = 9;
//const SIZE_Y : i32 = 9;

const SIZE_X : i32 = 129;
const SIZE_Y : i32 = 129;

#[derive(Debug,PartialEq,Clone)]
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
    ( map, start )
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

fn calc_path_with_direction( start: &Location, map: &Vec<Location>) -> Option<Vec<(Segment,Directions)>> {

    let mut dir = Directions::Up;

    let mut path = Vec::new();
    let mut pos: Location = Location( start.0, start.1);
    let mut seg: (Segment, Directions);

    loop {
        match dir {
            Directions::Up => {
                seg = (go_up( &pos, &map ), Directions::Up);
                dir = Directions::Right;
            }
            Directions::Right => {
                seg = (go_right( &pos, &map ), Directions::Right);
                dir = Directions::Down;
            }
            Directions::Down => {
                seg = (go_down( &pos, &map ), Directions::Down);
                dir = Directions::Left;
            }
            Directions::Left => {
                seg = (go_left( &pos, &map ), Directions::Left);
                dir = Directions::Up;
            }
        }
        pos = Location(seg.0.2, seg.0.3);

        if path.contains( &seg ) {
        //if path.iter().filter(|(s, p)| s == &seg.0 && p == &seg.1).count() > 0 {
            return None;
        }

        path.push( seg );

        if is_border( &pos ) {
            break;
        }
/*
        if path.len() > 130*130 {
            return None;
        }*/
    }
    return Some(path);
}

fn create_all_points( path: &Vec<Segment>) -> Vec<Location> {
    let mut all = Vec::new();

    for seg in path {
        if seg.0 == seg.2 {
            let min: i32 = if seg.1 < seg.3 { seg.1 } else { seg.3 };
            let max: i32 = if seg.1 < seg.3 { seg.3 } else { seg.1 };
            for y in min..=max {
                if !all.contains(&Location(seg.0, y)) {
                    all.push(Location(seg.0, y));    
                }
            }
            continue;
        }
        if seg.1 == seg.3 {
            let min: i32 = if seg.0 < seg.2 { seg.0 } else { seg.2 };
            let max: i32 = if seg.0 < seg.2 { seg.2 } else { seg.0 };
            for x in min..=max {
                if !all.contains(&Location(x, seg.1)) {
                    all.push(Location(x, seg.1));
                }
            }
            continue;
        }
    }
    //println!("all: {:#?}", all);
    all
}

fn create_all_points_no_guard( path: &Vec<Segment>) -> Vec<Location> {
    let mut all = Vec::new();

    let mut guard : Vec<Location> = Vec::new();
    for y in path[0].1..=path[0].3 {
        guard.push(Location(path[0].0, y));
    };
    

    for seg in path {
        if seg.0 == seg.2 {
            let min: i32 = if seg.1 < seg.3 { seg.1 } else { seg.3 };
            let max: i32 = if seg.1 < seg.3 { seg.3 } else { seg.1 };
            for y in min..=max {
                if !all.contains(&Location(seg.0, y)) && !guard.contains(&Location(seg.0, y)) {
                    all.push(Location(seg.0, y));    
                }
            }
            continue;
        }
        if seg.1 == seg.3 {
            let min: i32 = if seg.0 < seg.2 { seg.0 } else { seg.2 };
            let max: i32 = if seg.0 < seg.2 { seg.2 } else { seg.0 };
            for x in min..=max {
                if !all.contains(&Location(x, seg.1)) && !guard.contains(&Location(x, seg.1)) {
                    all.push(Location(x, seg.1));
                }
            }
            continue;
        }
    }
    //println!("all: {:#?}", all);
    all
}


fn part1() -> i32 {
    //let text = include_str!("./input_example.txt");
    let text = include_str!("./input.txt");

    let (map, start) = read_map(&text);

    let path = calc_path( &start, &map );

    create_all_points(&path).len() as i32
}


fn extract_path( path_dir: &Vec<(Segment,Directions)>) -> Vec<Segment> {
    path_dir.iter().map(|seg| seg.0.clone()).collect()
}

fn part2() -> i32 {
    //let text = include_str!("./input_example.txt");
    let text = include_str!("./input.txt");

    let (map, start) = read_map(&text);

    let path_dir = calc_path_with_direction( &start, &map ).unwrap();

    let all_points = create_all_points_no_guard(&extract_path(&path_dir));

    let mut infinite_loop = 0;

    for loc in all_points {
        if loc.0 != start.0 || loc.1 != start.1 {
            let mut map_with_i_hope_infinite_loop = map.clone();
            map_with_i_hope_infinite_loop.push(loc.clone());
            if calc_path_with_direction( &start, &map_with_i_hope_infinite_loop ).is_none() {
                infinite_loop+=1;
            }
        }
    }

    infinite_loop
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
