use crate::utils::input_as_str_vec;
use regex::{Regex, Match};

pub fn part01() {
    let _lines = input_as_str_vec("day03");
    let mut test: Vec<String> = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string()
    ];

    let blank = str::repeat(".", test[0].len());
    test.splice(0..0, vec![blank.clone()].iter().cloned());
    test.push(blank);
    test = test.iter().map(|line| {
        format!(".{line}.")
    }).collect();

    // let mut nums: Vec<Vec<Match>> = Vec::new();
    // let mut syms: Vec<Vec<Match>> = Vec::new();
    // let reg_num = Regex::new(r"(?<num>\d+)").unwrap();
    // let reg_sym = Regex::new(r"(?<sym>[^\d.\n])").unwrap();
    // for line in test.as_slice() {
    //     // println!("Line {idx}: {line}");

    //     let matches: Vec<Match> = reg_num.captures_iter(line).map(|caps| {
    //         caps.name("num").unwrap()
    //     }).collect();

    //     nums.push(matches);

    //     let matches: Vec<Match> = reg_sym.captures_iter(line).map(|caps| {
    //         caps.name("sym").unwrap()
    //     }).collect();

    //     syms.push(matches);
    // }

    // let mut sum: usize = 0;
    // for idx in 0..test.len() {

        // if nums[idx].is_empty() {
        //     continue;
        // }

        // remember, the digit starts on m.start and the last digit is m.end - 1 a.k.a. start + (len - 1)
        // for m in nums[idx].as_slice() {
            // if syms[idx].contains(m.start() - 1) {

            // }
        // }

        // println!("Line {idx}: {}\n\tNumber Matches: {:?}\n\tSymbol Matches: {:?}", test[idx], nums[idx], syms[idx]);
    // }

    // for (idx, m) in nums.iter().enumerate() {
        // check each match from [m.start..m.end) all around for something other than digit or .
        // perhaps I could get the symbols via regex and just compare indices rather than iterating?

    // }

}

pub fn part02() {
    println!("d3p2");
}