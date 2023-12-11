use std::fs::File;
use std::io::{self, BufRead};
use std::usize;

fn main() {
    let file = File::open("input.txt").unwrap();
    let bufreader = io::BufReader::new(file);
    let mut result: usize = 0;

    for line in bufreader.lines() {
        let line = line.unwrap();
        println!("{line}");
        let data: Vec<&str> = line.split(':').collect();
        let game_id = data[0];
        let games_data = data[1];
        let data: Vec<&str> = game_id.split(' ').collect();
        let game_id = data[1];
        println!("  game id: {game_id}");

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for game_data in games_data.split(';') {
            for color in game_data.split(',') {
                let color: Vec<&str> = color.trim().split(' ').collect();
                let num: usize = color[0].parse().unwrap();
                let color = color[1];

                if color.eq("red") {
                    if num > max_red {
                        max_red = num;
                    }
                }

                if color.eq("green") {
                    if num > max_green {
                        max_green = num;
                    }
                }

                if color.eq("blue") {
                    if num > max_blue {
                        max_blue = num;
                    }
                }
            }
        }
        println!("   r={max_red} b={max_blue} g={max_green}");
        result = result + (max_red * max_blue * max_green);
    }
    println!("\nresult {result}");
}
