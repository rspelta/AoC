#[derive(Debug,PartialEq,Clone)]
struct Points ( usize, usize );


fn find_start_points( text: &str) -> (Vec<Vec<u32>>, Vec<Points>)
{
    let mut start = Vec::new();
    let mut array: Vec<Vec<u32>> = Vec::new();
    
    for line in text.lines() {
        let row: Vec<u32> = line.chars().map(|ch| ch as u32 - 48).collect();
        array.push(row);
    }

    for (y, row) in array.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                start.push(Points(x, y));
            }
        }
    }

    //println!("{:?} {:?}", array, start);

    (array, start)
}

fn foo2( map: &Vec<Vec<u32>>, point: Points, counter: &mut Vec<Points> ) {

    //println!("{}", map[point.1][point.0]);
    
    if map[point.1][point.0] == 9 {
        counter.push(point);
        return
    }

    let next_step = map[point.1][point.0]+1;

    // su
    if point.1 > 0 {
        if map[point.1-1][point.0] == next_step {
            foo2( map, Points(point.0, point.1-1), counter );
        }
    }

    // dx
    if point.0 < map[0].len()-1 {
        if map[point.1][point.0+1] == next_step {
            foo2( map, Points(point.0+1, point.1), counter );
        }
    }

    // sx
    if point.0 > 0 {
        if map[point.1][point.0-1] == next_step {
            foo2( map, Points(point.0-1, point.1), counter );
        }
    }

    // dn
    if point.1 < map.len()-1 {
        if map[point.1+1][point.0] == next_step {
            foo2( map, Points(point.0, point.1+1), counter );
        }
    }
}


fn foo( map: &Vec<Vec<u32>>, point: Points, counter: &mut Vec<Points> ) {

    //println!("{}", map[point.1][point.0]);
    
    if map[point.1][point.0] == 9 {
        if !counter.contains(&point) {
            counter.push(point);
        }
        return
    }

    let next_step = map[point.1][point.0]+1;

    // su
    if point.1 > 0 {
        if map[point.1-1][point.0] == next_step {
            foo( map, Points(point.0, point.1-1), counter );
        }
    }

    // dx
    if point.0 < map[0].len()-1 {
        if map[point.1][point.0+1] == next_step {
            foo( map, Points(point.0+1, point.1), counter );
        }
    }

    // sx
    if point.0 > 0 {
        if map[point.1][point.0-1] == next_step {
            foo( map, Points(point.0-1, point.1), counter );
        }
    }

    // dn
    if point.1 < map.len()-1 {
        if map[point.1+1][point.0] == next_step {
            foo( map, Points(point.0, point.1+1), counter );
        }
    }
}

fn part2() -> u32 {
    let text = include_str!("input.txt");
    let (map, start) = find_start_points(text);
    let mut counter: Vec<Points> = Vec::new();
    let mut sum = 0;

    for point in start {
        foo2( &map, point, &mut counter );
        sum += counter.len() as u32;
        counter.clear();
    }
    //println!("{:?} {:?}", map, start);
    sum
}


fn part1() -> u32 {
    let text = include_str!("input.txt");
    let (map, start) = find_start_points(text);
    let mut counter: Vec<Points> = Vec::new();
    let mut sum = 0;

    for point in start {
        foo( &map, point, &mut counter );
        sum += counter.len() as u32;
        counter.clear();
    }
    //println!("{:?} {:?}", map, start);
    sum
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}