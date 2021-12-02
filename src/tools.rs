use std::fs;
use std::str::FromStr;

// return the file content as a vector of T (Where T can be any type that implements FromStr)
pub fn read_lines<T>(filename: &str) -> Vec<T>
    where T: FromStr {
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines: Vec<T> = vec!();
    for line in content.lines() {
        if let Ok(value) = line.parse::<T>()  {
            lines.push(value);
        }
    }
    lines
}