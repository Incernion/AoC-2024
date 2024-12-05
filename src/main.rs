#![allow(non_snake_case)]

mod day01;
mod day02;
mod day03;
mod parser;
mod day04;
mod day05;

use crate::day01::{d1c1, d1c2};
use crate::day02::{d2c1, d2c2};
use crate::day03::{d3c1, d3c2};
use std::fs::read_to_string;
use crate::day04::{d4c1, d4c2};
use crate::day05::{d5c1, d5c2};

fn getInput(d: i32) -> String {
    read_to_string(format!("inputs/in{}", d)).unwrap()
}

fn main() {
    let args = std::env::args();
    let a: Vec<i32> = args.skip(1).map(|x| x.parse().unwrap()).collect();
    let (day, challenge) = (a[0], a[1]);
    let res = match (day, challenge) {
        (1, 1) => d1c1(getInput(1)),
        (1, 2) => d1c2(getInput(1)),
        (2, 1) => d2c1(getInput(2)),
        (2, 2) => d2c2(getInput(2)),
        (3, 1) => d3c1(getInput(3)),
        (3, 2) => d3c2(getInput(3)),
        (4, 1) => d4c1(getInput(4)),
        (4, 2) => d4c2(getInput(4)),
        (5, 1) => d5c1(getInput(5)),
        (5, 2) => d5c2(getInput(5)),
        (_, _) => panic!("Invalid <3, use \"cargo run d c\", where d is the day and c is the challenge ")
    };
    println!("Result of day {}, challenge {}, is {}", day, challenge, res)
}
