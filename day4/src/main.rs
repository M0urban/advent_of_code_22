use std::io::{self, BufRead};
use std::fs::File;

fn main() {
    println!("{}", stage1("./day4/input.txt"));
    println!("{}", stage2("./day4/input.txt"));
}

fn stage1(file: &str) -> u32 
{
    let reader = io::BufReader::new(File::open(file).unwrap());
    let mut sum = 0u32;
    for line in reader.lines() {
        let mut nums: Vec<u32> = Vec::new();
        for elve in line.unwrap().split(",") {
            let f: Vec<u32> = elve.split("-").map(|f| f.parse::<u32>().unwrap()).collect();
            nums.extend_from_slice(f.as_slice())
        }
        if (nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[0] >= nums[2] && nums[1] <= nums[3]) {
            sum +=1;
        }
    }
    sum
}

fn stage2(file: &str) -> u32 
{
    let reader = io::BufReader::new(File::open(file).unwrap());
    let mut sum = 0u32;
    for line in reader.lines() {
        let mut nums: Vec<u32> = Vec::new();
        for elve in line.unwrap().split(",") {
            let f: Vec<u32> = elve.split("-").map(|f| f.parse::<u32>().unwrap()).collect();
            nums.extend_from_slice(f.as_slice())
        }
        if nums[0] <= nums[3] && nums[0] >= nums[2] {
            sum +=1;
        } else if nums[1] <= nums[3] && nums[1] >= nums[2] {
            sum +=1;
        } else if (nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[0] >= nums[2] && nums[1] <= nums[3]) {
            sum += 1;
        }
    }
    sum
}
