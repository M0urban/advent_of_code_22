
use std::io::{self, BufRead};
use std::fs;
use std::str::from_utf8;
use regex::Regex;

fn main() {
    stage1("./day5/input.txt").iter().for_each(|x| print!("{}", from_utf8([*x].as_slice()).unwrap()));
    println!("");
    stage2("./day5/input.txt").iter().for_each(|x| print!("{}", from_utf8([*x].as_slice()).unwrap()));
}

fn stage1(file: &str) -> Vec<u8> {
    let reader = fs::read_to_string(file).unwrap();
    let halfs: Vec<&str> = reader.split("\n\n").collect();
    let move_regex = Regex::new(r"\s(\d)\s$").unwrap();

    let finds = move_regex.captures(halfs[0]).unwrap();
    let number_of_stacks: usize = finds[0][1..2].parse().unwrap();
    let mut stacks: Vec<Vec<u8>> = vec![Vec::with_capacity(10); number_of_stacks];
    for line in halfs[0].lines()
    {
        if line.contains("[") {
            let line = line.as_bytes();
            for index in 0usize..9 {
                if line[4 * index + 1].is_ascii_alphabetic() {
                    stacks[index].push(line[4 * index + 1]);
                }
            }
        }
        
    }
    //Reverse order
    for stack in stacks.iter_mut() {
        stack.as_mut_slice().reverse();
    }
    let command_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in halfs[1].lines() {
        let capt = command_regex.captures(line).unwrap();
        let ammount: usize = capt.get(1)
                .unwrap()
                .as_str()
                .parse().unwrap();
        let src: usize = capt.get(2).unwrap().as_str().parse().unwrap(); 
        let dest: usize = capt.get(3).unwrap().as_str().parse().unwrap(); 
        for i in 0..ammount {
            let crates = stacks[src - 1].pop().unwrap();
            stacks[dest - 1].push(crates);
        }
    }
    let ret = Vec::<u8>::with_capacity(number_of_stacks);
    stacks.into_iter().map(|mut vec| vec.pop().unwrap()).collect()
}

fn stage2(file: &str) -> Vec<u8> {
    let reader = fs::read_to_string(file).unwrap();
    let halfs: Vec<&str> = reader.split("\n\n").collect();
    let move_regex = Regex::new(r"\s(\d)\s$").unwrap();

    let finds = move_regex.captures(halfs[0]).unwrap();
    let number_of_stacks: usize = finds[0][1..2].parse().unwrap();
    let mut stacks: Vec<Vec<u8>> = vec![Vec::with_capacity(10); number_of_stacks];
    for line in halfs[0].lines()
    {
        if line.contains("[") {
            let line = line.as_bytes();
            for index in 0usize..9 {
                if line[4 * index + 1].is_ascii_alphabetic() {
                    stacks[index].push(line[4 * index + 1]);
                }
            }
        }
        
    }
    //Reverse order
    for stack in stacks.iter_mut() {
        stack.as_mut_slice().reverse();
    }
    let command_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in halfs[1].lines() {
        let capt = command_regex.captures(line).unwrap();
        let ammount: usize = capt.get(1)
                .unwrap()
                .as_str()
                .parse().unwrap();
        let src: usize = capt.get(2).unwrap().as_str().parse().unwrap(); 
        let dest: usize = capt.get(3).unwrap().as_str().parse().unwrap(); 
        
        let mut crates = Vec::<u8>::with_capacity(ammount);
        for i in 0..ammount {
            crates.push(stacks[src - 1].pop().unwrap());
        }
        crates.as_mut_slice().reverse();
        stacks[dest - 1].extend_from_slice(crates.as_slice());
    }
    let ret = Vec::<u8>::with_capacity(number_of_stacks);
    stacks.into_iter().map(|mut vec| vec.pop().unwrap()).collect()
}
