use std::env::args;
use regex::Regex;
mod day01;
mod utils;

const FUNCS: [[fn(); 2]; 1] = [
    [ day01::part01, day01::part02 ],
    // [ day02::part01, day02::part02 ],
    // [ day03::part01, day03::part02 ],
    // [ day04::part01, day04::part02 ],
    // [ day05::part01, day05::part02 ],
    // [ day06::part01, day06::part02 ],
    // [ day07::part01, day07::part02 ],
    // [ day08::part01, day08::part02 ],
    // [ day09::part01, day09::part02 ],
    // [ day10::part01, day10::part02 ],
    // [ day11::part01, day11::part02 ],
    // [ day12::part01, day12::part02 ],
    // [ day13::part01, day13::part02 ],
    // [ day14::part01, day14::part02 ],
    // [ day15::part01, day15::part02 ],
    // [ day16::part01, day16::part02 ],
    // [ day17::part01, day17::part02 ],
    // [ day18::part01, day18::part02 ],
    // [ day19::part01, day19::part02 ],
    // [ day20::part01, day20::part02 ],
    // [ day21::part01, day21::part02 ],
    // [ day22::part01, day22::part02 ],
    // [ day23::part01, day23::part02 ],
    // [ day24::part01, day24::part02 ],
    // [ day25::part01, day25::part02 ]
];

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len() != 2 {
        println!("Must specify one day and puzzle to run (e.g. d7p2)");
        return;
    }

    let re = Regex::new(r"d(?<day>\d+)p(?<puzzle>[1,2])").unwrap();
    let Some(caps) = re.captures(&args[1]) else {
        println!("No valid argument found");
        return;
    };

    let day = caps["day"].parse::<usize>().unwrap_or(1);
    let puz = caps["puzzle"].parse::<usize>().unwrap_or(1);

    if !(1..=25).contains(&day) {
        println!("Day must be 1 through 25 (inclusive), puzzle must be 1 or 2 (e.g. d23p2)");
        return;
    }

    if day > FUNCS.len() {
        println!("Specified day has not yet been implemented.");
        return;
    }

    FUNCS[day - 1][puz - 1]();
}
