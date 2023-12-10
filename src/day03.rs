use crate::utils::input_as_str_vec;
use regex::{Regex, Match};

pub fn part01() {
    let _lines = input_as_str_vec("day03");
    let test = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
    ];

    let mut nums: Vec<Vec<Match>> = Vec::new();
    let re = Regex::new(r"(?<num>\d+)").unwrap();
    for line in test {
        // println!("Line {idx}: {line}");

        let matches: Vec<Match> = re.captures_iter(line).map(|caps| {
            caps.name("num").unwrap()
        }).collect();

        nums.push(matches);
    }

    for (idx, m) in nums.iter().enumerate() {
        // check each match from [m.start..m.end) all around for something other than digit or .
        // perhaps I could get the symbols via regex and just compare indices rather than iterating?
    }

}

pub fn part02() {
    println!("d3p2");
}