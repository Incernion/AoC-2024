use std::iter::zip;
use crate::parser::{getLines, words};

fn stringLsToIntPairs(ls: Vec<String>) -> (i32, i32){
    (ls[0].clone().parse().unwrap(), ls[1].clone().parse().unwrap())
}

fn getLists(content: String) -> (Vec<i32>, Vec<i32>) {
    getLines(&*content).iter().map(|x| words(x)).map(stringLsToIntPairs).unzip() // Black magic ✨
}

pub fn d1c1(content: String) -> i32 {
    let (mut fst, mut snd) = getLists(content);
    fst.sort();
    snd.sort();
    let sorted : Vec<(i32, i32)> = zip(fst, snd).collect();
    let mut acc = 0;
    for (num) in sorted {
        acc += (num.1 - num.0).abs();
    }
    acc
}

pub fn d1c2(content: String) -> i32 { // Yeah I'm solving this in O(n^2), cry about it
    let (fst, snd) = getLists(content);
    let mut acc = 0;
    for num in fst {
        let mut acc2 = 0;
        for &num2 in &snd {
            if (num == num2) {
                acc2 += 1;
            }
        }
        acc += num * acc2
    }
    acc
}



