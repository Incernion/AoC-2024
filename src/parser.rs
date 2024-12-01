pub fn words (line: &str) -> Vec<String> {
    line.split(" ").filter(|x| x != &"").map(str::to_string).collect()
}

pub fn getLines(ctn: &str) -> Vec<String> {
    ctn.lines().map(str::to_string).collect()
}