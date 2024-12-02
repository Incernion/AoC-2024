use crate::parser::{getLines, stringsToInts, words};

fn checkSafety (ls : &Vec<i32>, dampen: bool) -> bool {
    fn checkCriteria (ls : Vec<i32>, dampen: bool) -> bool{
        for i in 0..ls.len()-1{
            if !(ls[i] < ls[i+1]) || !(1..4).contains(&(ls[i] - ls[i + 1]).abs()){
                if dampen {
                    for j in 0..2 {
                        let mut clone = ls.clone();
                        clone.remove(i + j);
                        if checkCriteria(clone, false) {
                            return true;
                        }
                    }
                }
                return false
            }
        }
        true
    }
    checkCriteria(ls.clone(), dampen) || checkCriteria(ls.iter().rev().cloned().collect(), dampen)
}

pub fn d2 (content: String, dampen: bool) -> i32 {
    let parsed : Vec<Vec<i32>> = getLines(&*content).into_iter().map(|x| stringsToInts(words(&*x))).collect();
    let a : Vec<bool> = parsed.iter().map(|x| checkSafety(x, dampen)).collect();
    a.iter().map(|&elem| elem as i32).sum()
}

pub fn d2c1 (content: String) -> i32 {
    d2 (content, false)
}

pub fn d2c2 (content: String) -> i32 {
    d2 (content, true)
}