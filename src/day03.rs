use crate::parser::{findRegexPatterns, funcRegexPattern, unpackInts};

pub fn d3c1(content: String) -> i32 {
    funcRegexPattern(&content, r"mul\(\d{1,3},\d{1,3}\)", unpackInts)
        .iter()
        .fold(0, |acc, x| acc + x[0] * x[1])
}

pub fn d3c2(content: String) -> i32 {
    let matches = findRegexPatterns(&content, r"do\(\)|mul\(\d{1,3},\d{1,3}\)|don't\(\)");
    let mut active : bool = true;
    let mut acc : i32 = 0;
    for unit in matches {
        match unit.as_str() {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                if active{
                    let vals = unpackInts(&unit);
                    acc += vals[0] * vals[1]
                }
            } 
        }
    }
    acc
}