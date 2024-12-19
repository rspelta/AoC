
#[derive(Debug,PartialEq,Clone)]
enum DataType {
    File,
    FreeSpace,
}
#[derive(Debug,PartialEq,Clone)]
struct Block {
    // String is a struct
    block_type: DataType,
    id: u32,
    size: u32,
}


fn first_none_from_head(vec: &Vec<Option<u32>>, start : u32) -> Option<u32> {
    for i in start as usize..vec.len() {
        if vec[i].is_none() {
            return Some(i as u32);
        }
    }
    None
}

fn first_some_form_tail(vec: &Vec<Option<u32>>, end : u32) -> Option<u32> {
    for i in (0 as usize..end as usize).rev() {
        if vec[i].is_some() {
            return Some(i as u32);
        }
    }
    None
}

fn printa( vec: &Vec<Option<u32>>) {
    for i in 0..vec.len() {
        if vec[i].is_some() {
            print!("{} ", vec[i].unwrap());    
        } else {
            print!(".");
        }
    }
    println!("");
}

fn part1() -> u64 {
    let text = include_str!("input.txt");

    let mut filesystem = Vec::new();
    let mut ID = 0;
    let mut index = 0;
    let mut freespace = 0;
    let mut crc: u64 = 0;

    // creo filesystem
    for line in text.lines() {
        for chars in line.chars() {
            let num = chars.to_digit(10).unwrap();

            if index % 2 == 0 {
                filesystem.extend(std::iter::repeat(Some(ID)).take(num as usize));
                ID += 1;
            } else {
                filesystem.extend(std::iter::repeat(None).take(num as usize));
                freespace += num;
            }
            index += 1;
        }
    }

    //printa(&filesystem );

    let mut left: u32 = 0;
    let mut right: u32 = filesystem.len() as u32;
    // compatto
    while left < right {
        let l = first_none_from_head(&filesystem, left);
        let r = first_some_form_tail(&filesystem, right);


        if l.is_some() && r.is_some() {
            left = l.unwrap();
            right = r.unwrap();
            if left < right {
                filesystem[left as usize] = filesystem[right as usize];
                filesystem[right as usize] = None;
                freespace -= 1;
                //printa(&filesystem );
            }
        }
    }

    for (index, value) in filesystem.iter().enumerate() {
        if value.is_some() {
            crc += value.unwrap() as u64 * index as u64;
        }
    }
    //println!("{:?}", filesystem);
    
    crc
}

fn printa_blocks( filesystem: &Vec<Block> )
{
    for block in filesystem {
        for _ in 0..block.size {
            if block.block_type == DataType::File {
                print!("{}", block.id);
            } else {
                print!(".");
            }
        }
    }
    println!("");
}

fn part2() -> u64 {
    let text = include_str!("input.txt");

    let mut filesystem: Vec<Block> = Vec::new();
    let mut id = 0;
    let mut index = 0;
    let mut crc: u64 = 0;

    // creo filesystem
    for line in text.lines() {
        for chars in line.chars() {
            let num = chars.to_digit(10).unwrap();

            if index % 2 == 0 {
                filesystem.push(Block{ block_type: DataType::File, id: id, size: num } );//.take(num as usize));
                id += 1;
            } else {
                filesystem.push(Block{ block_type: DataType::FreeSpace, id: num, size: num } );//.take(num as usize));
            }
            index += 1;
        }
    }

    printa_blocks(&filesystem );

    let mut left: usize = 0;
    let mut right: usize = filesystem.len()-1;

    while  right > 0 {
        if filesystem[right].block_type == DataType::File {
            if filesystem[left].block_type == DataType::FreeSpace && filesystem[left].size >= filesystem[right].size  {

                let tmp: Block = filesystem[left].clone();
                filesystem[left] = filesystem[right].clone();
                filesystem[right] = Block{ block_type: DataType::FreeSpace, id: tmp.id, size: filesystem[left].size };
                if tmp.size - filesystem[right].size > 0 {
                    if left+1 < filesystem.len() && filesystem[left+1].block_type == DataType::FreeSpace {
                        filesystem[left+1].size += tmp.size - filesystem[left].size;
                    } else {
                        filesystem.insert(left+1, Block{ block_type: DataType::FreeSpace, id: tmp.id, size: tmp.size - filesystem[right].size });
                    }
                }
                left = 0;
                right -= 1;


            } else {
                left += 1;
                if left >= right {
                    left = 0;
                    right -= 1;
                }
            }
        } else {
            right -= 1;
        }
    }

    printa_blocks(&filesystem );

    index = 0;
    for  block in filesystem {
        for _ in 0..block.size {
            if block.block_type == DataType::File {
                crc += block.id as u64 * index as u64;
            }
            index += 1;
        }
    }
    //println!("{:?}", filesystem);
    
    crc
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}