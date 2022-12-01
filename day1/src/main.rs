use std::io::{self, BufRead, Read};
use std::fs::File;

fn main() {
    let calories = most_calories("./day1.txt");
    println!("{}", calories);
    let calories = top3_calories("./day1.txt");
    println!("{}", calories);
}

fn most_calories(file: &str) -> u32 {
    let reader = io::BufReader::new(File::open(file).unwrap());
    let mut max_calories = 0u32;
    let mut current_calories = 0u32;

    for slice in  reader.lines() {
        match slice.unwrap().parse::<u32>() {
            Ok(num) => current_calories += num,
            Err(_) => {
                if max_calories < current_calories{
                    max_calories = current_calories;
                }
                current_calories = 0;
            },
        }

    }
    if max_calories < current_calories{
        max_calories = current_calories;
    }
    max_calories
}

fn top3_calories(file: &str) -> u32 {
    let reader = io::BufReader::new(File::open(file).unwrap());
    let mut max_calories = [0u32; 3];
    let mut current_calories = 0u32;

    for slice in  reader.lines() {
        match slice.unwrap().parse::<u32>() {
            Ok(num) => current_calories += num,
            Err(_) => {
                max_calories.sort();
                'l: for cal in max_calories.iter_mut() {
                    if *cal < current_calories{
                        *cal = current_calories;
                        break 'l;
                    }
                }
                current_calories = 0;
            },
        }

    }
    max_calories.sort();
    'l: for cal in max_calories.iter_mut() {
        if *cal < current_calories{
            *cal = current_calories;
            break 'l;
        }
    }
    max_calories[0] + max_calories[1] + max_calories[2]
}
