use std::collections::HashSet;
use crate::parser::{eatParagraph, makePairs, unpackInts};

pub fn parse(mut content: String) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let precedence = eatParagraph(&mut content).iter().map(unpackInts).map(makePairs).collect::<HashSet<(i32, i32)>>();
    let pages = eatParagraph(&mut content).iter().map(unpackInts).collect::<Vec<Vec<i32>>>();
    (precedence, pages)
}

pub fn d5c1 (content: String) -> i32 {
    let (precedence, pages) = parse(content);
    
    pages.iter().fold(0, |acc, update|{
        for i in 0..update.len()-1{
            let pair = (update[i], update[i+1]);
            if !precedence.contains(&pair){ break; }
            if i == update.len()-2 {return acc+update[update.len()/2];}
        } acc
    })
}

pub fn d5c2 (content: String) -> i32 {
    let (precedence, pages) = parse(content);

    let corruptedUpdated: Vec<Vec<i32>> = pages.into_iter().filter(|update| {
        for i in 0..update.len()-1{
            let pair = (update[i], update[i+1]);
            if !precedence.contains(&pair){ return true; }
        } false
    }).collect();
    
    fn cleaner (corrupted: &mut Vec<i32>, precedence: &HashSet<(i32, i32)>) {
        let mut clean = true;
        for i in 0..corrupted.len() - 1 {
            let pair = (corrupted[i], corrupted[i + 1]);
            if !precedence.contains(&pair) {
                clean = false;
                corrupted.swap(i, i + 1);
            }
        }
        if !clean {cleaner(corrupted, precedence)}
    }

    corruptedUpdated.into_iter().fold(0, |acc, mut corrupted| {
        cleaner(&mut corrupted, &precedence);
        acc + corrupted[corrupted.len()/2]
    })
}