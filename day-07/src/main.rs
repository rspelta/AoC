
fn get_union(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}

 fn foo2( now: u64, mut values: Vec<u64>, result: u64, counter: &mut u64) -> ()
 {
    if values.len() == 0 {
        //println!("res: {}", now);
        if now == result {
            *counter += 1;
        }
        return ();
    }

    let val = values.pop().unwrap();
    //println!("{} * {} = {}", now, val, now * val);
    foo2( now * val, values.clone(), result, counter);

    foo2( get_union(now, val), values.clone(), result, counter);

    //println!("{} + {} = {}", now, val, now + val);
    foo2( now + val, values.clone(), result, counter);
 }

 fn foo1( now: u64, mut values: Vec<u64>, result: u64, counter: &mut u64) -> ()
 {
    if values.len() == 0 {
        //println!("res: {}", now);
        if now == result {
            *counter += 1;
        }
        return ();
    }

    let val = values.pop().unwrap();
    //println!("{} * {} = {}", now, val, now * val);
    foo1( now * val, values.clone(), result, counter);

    //println!("{} + {} = {}", now, val, now + val);
    foo1( now + val, values.clone(), result, counter);
 }

fn part1() -> u64 {
    let mut sum = 0;
    let mut counter: u64;
    let input = include_str!("input.txt");

    for line in input.lines() {
        counter = 0;
        let mut values = line.split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let result = line.split(':').nth(0).unwrap().parse::<u64>().unwrap();
        values.reverse();
        //println!("result: {} values: {:?}", result, values);
        let val = values.pop().unwrap();

        foo1( val, values, result, &mut counter);

        //println!("counter: {}", counter);
        if counter > 0 {
            sum += result;   
        }
    }
    sum
}

fn part2() -> u64 {
    let mut sum = 0;
    let mut counter: u64;
    let input = include_str!("input.txt");

    for line in input.lines() {
        counter = 0;
        let mut values = line.split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let result = line.split(':').nth(0).unwrap().parse::<u64>().unwrap();
        values.reverse();
        //println!("result: {} values: {:?}", result, values);
        let val = values.pop().unwrap();

        foo2( val, values, result, &mut counter);

        //println!("counter: {}", counter);
        if counter > 0 {
            sum += result;   
        }
    }
    sum
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}