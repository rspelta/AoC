
fn isXmasHorizontal(x: usize, y:  usize, input: Vec<&str>) -> bool {
    (input[y].chars().nth(x).unwrap() == 'X' && input[y].chars().nth(x+1).unwrap() == 'M' && input[y].chars().nth(x+2).unwrap() == 'A' && input[y].chars().nth(x+3).unwrap() == 'S') || 
            (input[y].chars().nth(x).unwrap() == 'S' && input[y].chars().nth(x+1).unwrap() == 'A' && input[y].chars().nth(x+2).unwrap() == 'M' && input[y].chars().nth(x+3).unwrap() == 'X')
}

// not working
fn isXmasVertical(x i32, y i32, input Vec<Vec<i32>>) -> bool {
    return (input[y][x] == 'X' && input[y+1][x] == 'M' && input[y+2][x] == 'A' && input[y+1][x] == 'S') || (input[y][x] == 'S' && input[y+1][x] == 'A' && input[y+1][x+2] == 'M' && input[y+1][x] == 'X')
}

// not working
fn isXmasDiagonal(x i32, y i32, input Vec<Vec<i32>>) -> bool {
    return (input[y][x] == 'X' && input[y+1][x+1] == 'M' && input[y+1][x+1] == 'A' && input[y+1][x] == 'S') || (input[y][x] == 'S' && input[y+1][x] == 'A' && input[y+1][x+2] == 'M' && input[y+1][x] == 'X')
}


fn part1() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;

    let lines: Vec<&str> = text.lines().collect();

    let sizex: usize = lines[0].len() as usize;
    let sizey: usize = lines.len() as usize;

    for y in 0..sizex {
        for x in 0..sizey {
            isXmasHorizontal(x, y, text.lines().collect());
        }
    }
    sum
}


fn main() {
    println!("{}", part1());
    //println!("{}", part2());
}

// working