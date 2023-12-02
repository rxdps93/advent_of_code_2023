use std::env;
use regex::Regex;
mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let re = Regex::new(r"d(?<day>\d+)p(?<puzzle>[1,2])").unwrap();
    let Some(caps) = re.captures(&args[1]) else {
        println!("No valid argument found");
        return;
    };

    println!("day: {}, puzzle: {}", &caps["day"], &caps["puzzle"]);
}
