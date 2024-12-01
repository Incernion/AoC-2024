#![allow(non_snake_case)]

mod day01;
mod parser;

use std::fs::read_to_string;
use crate::day01::{d1c1, d1c2};

fn getInput(d : i32) -> String{
    read_to_string(format!("inputs/in{}", d)).unwrap()
}

fn main() {
    let args = std::env::args();
    let a : Vec<i32> = args.skip(1).map(|x| x.parse().unwrap()).collect();
    let (day, challenge) = (a[0], a[1]);
    let res = match (day, challenge) {
        (1, 1) => d1c1(getInput(1)),
        (1, 2) => d1c2(getInput(1)),
        (_, _) => panic!("Invalid <3, use \"cargo run d c\", where d is the day and c is the challenge ")
    };
    
    println!("Result of day {}, challenge {}, is {}", day, challenge, res)
}
