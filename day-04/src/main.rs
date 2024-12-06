
fn is_xmas_horizontal(x: usize, y:  usize, input: &Vec<&str>) -> bool {
    (input[y].chars().nth(x).unwrap() == 'X' && input[y].chars().nth(x+1).unwrap() == 'M' && input[y].chars().nth(x+2).unwrap() == 'A' && input[y].chars().nth(x+3).unwrap() == 'S') || 
            (input[y].chars().nth(x).unwrap() == 'S' && input[y].chars().nth(x+1).unwrap() == 'A' && input[y].chars().nth(x+2).unwrap() == 'M' && input[y].chars().nth(x+3).unwrap() == 'X')
}

fn is_xmax_vertical(x: usize, y: usize, input: &Vec<&str>) -> bool {
    let normal: bool = input[y].chars().nth(x).unwrap() == 'X' && input[y+1].chars().nth(x).unwrap() == 'M' && input[y+2].chars().nth(x).unwrap() == 'A' && input[y+3].chars().nth(x).unwrap() == 'S';
    let reverse: bool = input[y].chars().nth(x).unwrap() == 'S' && input[y+1].chars().nth(x).unwrap() == 'A' && input[y+2].chars().nth(x).unwrap() == 'M' && input[y+3].chars().nth(x).unwrap() == 'X';
    
    normal || reverse
}

fn is_xmax_diagonal(x: usize, y: usize, input: &Vec<&str>) -> bool {
    let diagonal_right: bool = input[y].chars().nth(x).unwrap() == 'X' && input[y+1].chars().nth(x+1).unwrap() == 'M' && input[y+2].chars().nth(x+2).unwrap() == 'A' && input[y+3].chars().nth(x+3).unwrap() == 'S';
    let diagonal_right_reverse: bool = input[y].chars().nth(x).unwrap() == 'S' && input[y+1].chars().nth(x+1).unwrap() == 'A' && input[y+2].chars().nth(x+2).unwrap() == 'M' && input[y+3].chars().nth(x+3).unwrap() == 'X';
 
    let diagonal_left: bool = input[y].chars().nth(x).unwrap() == 'X' && input[y-1].chars().nth(x-1).unwrap() == 'M' && input[y-2].chars().nth(x-2).unwrap() == 'A' && input[y-3].chars().nth(x-3).unwrap() == 'S';
    let diagonal_left_reverse: bool = input[y].chars().nth(x).unwrap() == 'S' && input[y-1].chars().nth(x-1).unwrap() == 'A' && input[y-2].chars().nth(x-2).unwrap() == 'M' && input[y-3].chars().nth(x-3).unwrap() == 'X';
 
 
    diagonal_right || diagonal_right_reverse || diagonal_left || diagonal_left_reverse
}

/*

fn is_mas(x: usize, y: usize, input: &Vec<&str>) -> bool {
    let normal: bool = input[y].chars().nth(x).unwrap() == 'M' && input[y+1].chars().nth(x+1).unwrap() == 'A' && input[y+2].chars().nth(x+2).unwrap() == 'S' &&
                        input[y].chars().nth(x+2).unwrap() == 'M' && input[y].chars().nth(x+2).unwrap() == 'S';

    let reverse: bool = input[y].chars().nth(x).unwrap() == 'M' && input[y+1].chars().nth(x+1).unwrap() == 'A' && input[y+2].chars().nth(x+2).unwrap() == 'S' &&
                        input[y].chars().nth(x+2).unwrap() == 'M' && input[y].chars().nth(x+2).unwrap() == 'S';

    let reverse1: bool = input[y].chars().nth(x).unwrap() == 'M' && input[y+1].chars().nth(x+1).unwrap() == 'A' && input[y+2].chars().nth(x+2).unwrap() == 'S' &&
                         input[y].chars().nth(x+2).unwrap() == 'S' && input[y].chars().nth(x+2).unwrap() == 'M';

    let reverse2: bool = input[y].chars().nth(x).unwrap() == 'S' && input[y+1].chars().nth(x+1).unwrap() == 'A' && input[y+2].chars().nth(x+2).unwrap() == 'M' &&
                         input[y].chars().nth(x+2).unwrap() == 'M' && input[y].chars().nth(x+2).unwrap() == 'S';
    
    normal || reverse || reverse1 || reverse2
}
    */

/*
// not working
fn isXmasVertical(x i32, y i32, input Vec<Vec<i32>>) -> bool {
    return (input[y][x] == 'X' && input[y+1][x] == 'M' && input[y+2][x] == 'A' && input[y+1][x] == 'S') || (input[y][x] == 'S' && input[y+1][x] == 'A' && input[y+1][x+2] == 'M' && input[y+1][x] == 'X')
}

// not working
fn isXmasDiagonal(x i32, y i32, input Vec<Vec<i32>>) -> bool {
    return (input[y][x] == 'X' && input[y+1][x+1] == 'M' && input[y+1][x+1] == 'A' && input[y+1][x] == 'S') || (input[y][x] == 'S' && input[y+1][x] == 'A' && input[y+1][x+2] == 'M' && input[y+1][x] == 'X')
}

*/
fn part1() -> i32 {
    let text = include_str!("./input.txt");
    let mut sum : i32 = 0;

    let lines: Vec<&str> = text.lines().collect();

    let sizex: usize = lines[0].len() as usize;
    let sizey: usize = lines.len() as usize;

    for y in 0..sizex {
        for x in 0..sizey {
            if sizex - x >= 4 {   
                if is_xmas_horizontal(x, y, &lines) {
                    sum += 1;
                }
            }

            if sizey - y >= 4 {   
                if is_xmax_vertical(x, y, &lines) {
                    sum += 1;
                }
            }
        }
    }
    sum
}


fn main() {
    println!("{}", part1());
    //println!("{}", part2());
}

// working