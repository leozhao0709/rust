use std::fs;

pub fn main() {
    let first_line = read_first_line();
    println!("{}", first_line.unwrap());
}

fn read_first_line() -> Option<String> {
    fs::read_to_string("src/main.rs")
        .ok()
        .and_then(|s| s.lines().next().map(|line| line.to_owned()))
}
