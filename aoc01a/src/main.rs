use std::fs::File;
use std::io::{self, BufRead};

fn lastd(line: &str) -> u32 {
    for c in line.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }

    return 0;
}
fn firstd(line: &str) -> u32 {
    for c in line.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }

    return 0;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let bufreader = io::BufReader::new(file);
    let mut result = 0;

    for line in bufreader.lines() {
        let line = line.unwrap();
        println!("current line:\n   {line}");
        let firstd = firstd(&line);
        println!("   first digit {firstd}");
        let lastd = lastd(&line);
        println!("   last digit {lastd}");
        result = result + firstd * 10 + lastd;
        println!("   result {result}");
        //break;
    }
}
