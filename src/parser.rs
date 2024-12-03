use regex::Regex;

pub fn words(line: &str) -> Vec<String> {
    line.split(" ").filter(|x| x != &"").map(str::to_string).collect()
}

pub fn getLines(ctn: &str) -> Vec<String> {
    ctn.lines().map(str::to_string).collect()
}

pub fn stringsToInts(ls: Vec<String>) -> Vec<i32> {
    ls.iter().map(|x| x.parse().unwrap()).collect()
}

pub fn funcRegexPattern<T>(text: &str, pattern: &str, f: impl Fn(&str) -> T) -> Vec<T> {
    let reg = Regex::new(pattern).unwrap();
    reg.find_iter(&text).map(|x| f(x.as_str())).collect()
}

pub fn findRegexPatterns(text: &str, pattern: &str) -> Vec<String> {
    funcRegexPattern(text, pattern, ToOwned::to_owned)
}

pub fn unpackInts(line: &str) -> Vec<i32> {
    let reg = Regex::new(r"\d+").unwrap();
    reg.find_iter(&*line).map(|x| x.as_str().parse().unwrap()).collect()
}
