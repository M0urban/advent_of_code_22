use std::io::{self, BufRead, Read};
use std::fs::File;


fn main() {
    println!("{}", stage1("/home/marius/Documents/advent_of_code_22/day2/input.txt"));
    println!("{}", stage2("/home/marius/Documents/advent_of_code_22/day2/input.txt"));
}

fn stage1(filename: &str) -> u32 {
    let reader = io::BufReader::new(File::open(filename).unwrap());
    let mut score: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 3 {
            let (me, opp) = RPS::new(line.chars().nth(2).unwrap(), line.chars().next().unwrap());
            score += RPS::score(opp, me);
        }
    }
    score
}

#[derive(Clone, Copy)]
pub enum RPS {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl RPS {
    pub fn new(me: char, opp: char) -> (Self, Self){
        let opp = match opp {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissor,
            _ => panic!("failed making rps"),
        };

        let me = match me {
            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissor,
            _ => panic!("failed making rps"),
        };
        (me, opp)
    }
    pub fn new2(me: char, opp: char) -> (Winner, Self){
        let opp = match opp {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissor,
            _ => panic!("failed making rps"),
        };

        let me = Winner::new(me);
        (me, opp)
    }

    pub fn score(opp:Self, me: Self) -> u32{
        RPS::winner(me, opp) as u32 + me as u32
    }
    
    pub fn score2(opp:Self, me: Winner) -> u32{
        me as u32 + match (me, opp) {
            (Winner::Lose, RPS::Rock) => RPS::Scissor as u32,
            (Winner::Lose, RPS::Paper) => RPS::Rock as u32,
            (Winner::Lose, RPS::Scissor) => RPS::Paper as u32,
            (Winner::Draw, RPS::Rock) => RPS::Rock as u32,
            (Winner::Draw, RPS::Paper) => RPS::Paper as u32,
            (Winner::Draw, RPS::Scissor) => RPS::Scissor as u32,
            (Winner::Win, RPS::Rock) => RPS::Paper as u32,
            (Winner::Win, RPS::Paper) => RPS::Scissor as u32,
            (Winner::Win, RPS::Scissor) => RPS::Rock as u32,
        }
    }

    pub fn winner(me: Self, opp:Self) -> Winner {
        match me {
            RPS::Rock => {
                match opp {
                    RPS::Rock => Winner::Draw,
                    RPS::Paper => Winner::Lose,
                    RPS::Scissor => Winner::Win,
                }
            },
            RPS::Paper => {
                match opp {
                    RPS::Rock => Winner::Win,
                    RPS::Paper => Winner::Draw,
                    RPS::Scissor => Winner::Lose,
                }
            },
            RPS::Scissor => {
                match opp {
                    RPS::Rock => Winner::Lose,
                    RPS::Paper => Winner::Win,
                    RPS::Scissor => Winner::Draw,
                }
            },
        }
    }
}

#[derive(Clone,Copy)]
pub enum Winner {
    Lose = 0,
    Draw = 3,
    Win = 6, 
}

impl Winner {
    fn new(me: char) -> Self {
        match me {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("failed making rps"),
        }
    }
}

fn stage2(filename: &str) -> u32 {
    let reader = io::BufReader::new(File::open(filename).unwrap());
    let mut score: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 3 {
            let (me, opp) = RPS::new2(line.chars().nth(2).unwrap(), line.chars().next().unwrap());
            score += RPS::score2(opp, me);
        }
    }
    score
}