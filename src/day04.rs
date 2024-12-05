use std::ops::Add;
use crate::parser::getLines;

fn getGridIndex(grid: &Vec<Vec<char>>, y: i32, x: i32) -> Option<char>{
    if let Some(line) = grid.get(y as usize) {
        if let Some(ch) = line.get(x as usize) {
            return Some(*ch);
        }
    }
    None
}

fn findWord(grid: &Vec<Vec<char>>, y: i32, x: i32) -> i32{ 
    let word = "XMAS";
    let tests : Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut wordsFound = 0;
    
    for test in tests{
        let mut foundWord = getGridIndex(grid, y, x).unwrap().to_string();
        for i in 1..word.len(){
            let (checkY, checkX) = (y + test.0 * i as i32, x + test.1 * i as i32);
            if let Some(ch) = getGridIndex(grid, checkY, checkX) { foundWord.push(ch) }
        }
        if word == foundWord { wordsFound += 1; }
    }
    wordsFound
}

fn checkPattern(grid: &Vec<Vec<char>>, y: i32, x: i32, tests: &Vec<(i32, i32)>) -> bool {
    let mut test_string = String::new();
    for i in 0..=1 {
        let check1 = y + tests[i].0;
        let check2 = x + tests[i].1;
        if let Some(v) = getGridIndex(grid, check1, check2) {
            test_string.push(v);
        }
    }
    test_string == "MS" || test_string == "SM"
}

fn findCrossMas(grid: &Vec<Vec<char>>, y: i32, x: i32) -> i32 {
    let test1: Vec<(i32, i32)> = vec![(-1, -1), (1, 1)];
    let test2: Vec<(i32, i32)> = vec![(-1, 1), (1, -1)];

    if getGridIndex(grid, y, x).unwrap().to_string() == "A" {
        if checkPattern(grid, y, x, &test1) && checkPattern(grid, y, x, &test2) {
            return 1;
        }
    }
    0
}

pub fn d4c1 (content: String) -> i32 {
    let grid: Vec<Vec<char>> = getLines(&content).iter().map(|x| x.chars().collect()).collect();
    let mut acc = 0;
    for y in 0..grid.len(){
        let line = &grid[y];
        for x in 0..line.len(){
            acc += findWord(&grid, y as i32, x as i32)
        }
    }
    acc
}

pub fn d4c2 (content: String) -> i32 {
    let grid: Vec<Vec<char>> = getLines(&content).iter().map(|x| x.chars().collect()).collect();
    let mut acc = 0;
    for y in 0..grid.len(){
        let line = &grid[y];
        for x in 0..line.len(){
            acc += findCrossMas(&grid, y as i32, x as i32)
        }
    }
    acc
}