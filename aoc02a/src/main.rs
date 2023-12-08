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

        for game_data in games_data.split(';') {
            print!("  ");
            for color in game_data.split(',') {
                let color: Vec<&str> = color.trim().split(' ').collect();
                let num: usize = color[0].parse().unwrap();
                let color = color[1];
                print!("{color}:{num}\t\t");
            }
            println!();
        }
        result = result + game_id.parse::<usize>().unwrap();
    }
    println!("\nresult {result}");
}
