use std::io::{self, BufRead};
use std::fs;
use std::iter::zip;
use std::collections::HashSet;


fn main() {
    println!("{}", stage1("./day6/input.txt"));
    println!("{}", stage2("./day6/input.txt", 14));
}

fn stage1(file: &str) -> u32 {
    let file = fs::read(file).unwrap();
    let mut index: u32 = 0;
    for (i, slc) in zip(4.., file.windows(4)) {
        let (a, b, c, d) = (slc[0], slc[1], slc[2], slc[3]);
        if a != b && a != c && a != d && b != c && b !=d && c != d {
            index = i;
            break;
        }
    }
    index
}

fn stage2(file: &str, head_len: usize) -> u32 {
    let file = fs::read(file).unwrap();
    let mut index: u32 = 0;
    let mut set = HashSet::<u8>::new();
    'outer: for (i, slc) in zip(head_len as u32.., file.windows(head_len)) {
        'inner: for num in slc {
            if !set.insert(*num) {
                set.drain();
                continue 'outer;
            }
        }
        index = i;
        break 'outer; 
    }
    index
}

