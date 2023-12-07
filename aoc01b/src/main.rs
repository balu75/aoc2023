use std::fs::File;
use std::io::{self, BufRead};

fn stars_with_number(i: usize, line: &str) -> u32 {
    let slice = &line[i..];
    if slice.starts_with("one") {
        return 1;
    } else if slice.starts_with("two") {
        return 2;
    } else if slice.starts_with("three") {
        return 3;
    } else if slice.starts_with("four") {
        return 4;
    } else if slice.starts_with("five") {
        return 5;
    } else if slice.starts_with("six") {
        return 6;
    } else if slice.starts_with("seven") {
        return 7;
    } else if slice.starts_with("eight") {
        return 8;
    } else if slice.starts_with("nine") {
        return 9;
    } else if slice.starts_with("zero") {
        return 0;
    } else {
        return 999;
    }
}

fn lastd(line: &str) -> u32 {
    for (i, c) in line.chars().rev().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
        let num = stars_with_number(line.len() - i - 1, line);
        if num != 999 {
            return num;
        }
    }

    return 0;
}
fn firstd(line: &str) -> u32 {
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
        let num = stars_with_number(i, line);
        if num != 999 {
            return num;
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
