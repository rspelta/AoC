use regex::Regex;

fn part1() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;

    sum
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

// working
fn isXmasHorizontal(x i32, y i32, input Vec<Vec<i32>>) -> bool {
    return (input[y][x] == 'X' && input[y][x+1] == 'M' && input[y][x+2] == 'A' && input[y][x+1] == 'S') || (input[y][x] == 'S' && input[y][x+1] == 'A' && input[y][x+2] == 'M' && input[y][x+1] == 'X')
}

// not working
fn isXmasVertical(x i32, y i32, input Vec<Vec<i32>>) -> bool {
    return (input[y][x] == 'X' && input[y+1][x] == 'M' && input[y+2][x] == 'A' && input[y+1][x] == 'S') || (input[y][x] == 'S' && input[y+1][x] == 'A' && input[y+1][x+2] == 'M' && input[y+1][x] == 'X')
}

// not working
fn isXmasDiagonal(x i32, y i32, input Vec<Vec<i32>>) -> bool {
    return (input[y][x] == 'X' && input[y+1][x+1] == 'M' && input[y+1][x+1] == 'A' && input[y+1][x] == 'S') || (input[y][x] == 'S' && input[y+1][x] == 'A' && input[y+1][x+2] == 'M' && input[y+1][x] == 'X')
}