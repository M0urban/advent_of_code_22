use std::io::{self, BufRead};
use std::fs::File;
use std::collections::hash_set::HashSet;

fn main() {
    println!("{}", stage1("./day3/input.txt"));
    println!("{}", stage2("./day3/input.txt"));
}


fn stage1(file: &str) -> u32{
    let reader = io::BufReader::new(File::open(file).unwrap());
    reader.split(b'\n')
        .map(|slice| slice.unwrap())
        .map(|slice| {
            let len = slice.len();
            let (former, latter) = slice.as_slice().split_at(len / 2);
            let mut set: HashSet<u8> = HashSet::new();
            for byte in former {
                set.insert(*byte);
            }
            let mut double = 0u8;
            for byte in latter {
                if set.contains(byte) {
                    double = *byte;
                    break;
                }
            }
            if double == 0 {
                0u32
            } else {
                prio(double)
            }
        }).sum()
}

fn stage2(file: &str) -> u32{
    let reader = io::BufReader::new(File::open(file).unwrap());
    let mut sum = 0u32;
    let mut iter = reader.split(b'\n');
    'outer: loop {
        let (first, second, third) = (iter.next(), iter.next(), iter.next());
        match (first, second, third) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => {
                let mut set: HashSet<u8> = HashSet::new();
                for byte in a.iter() {
                    set.insert(*byte);
                }
                for byte in b.iter() {
                    if set.contains(byte) {
                        for cbyte in c.iter() {
                            if *byte == *cbyte {
                                sum += prio(*byte);
                                continue 'outer;
                            }
                        }
                    }
                }
            },
            _ => break 'outer,
        }
    }
    sum
}

fn prio(item:u8) -> u32{
    if item.is_ascii_lowercase() {
        (item - 96) as u32
    } else {
        (item - 38) as u32
    }
}
