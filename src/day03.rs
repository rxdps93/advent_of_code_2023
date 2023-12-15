use crate::utils::input_as_str_vec;
use regex::{Regex, Match};

pub fn part01() {
    let mut lines = input_as_str_vec("day03");
    // let mut lines: Vec<String> = vec![
    //     "467..114..".to_string(),
    //     "...*......".to_string(),
    //     "..35..633.".to_string(),
    //     "......#...".to_string(),
    //     "617*......".to_string(),
    //     ".....+.58.".to_string(),
    //     "..592.....".to_string(),
    //     "......755.".to_string(),
    //     "...$.*....".to_string(),
    //     ".664.598..".to_string()
    // ];

    let blank = str::repeat(".", lines[0].len());
    lines.splice(0..0, vec![blank.clone()].iter().cloned());
    lines.push(blank);
    lines = lines.iter().map(|line| {
        format!(".{line}.")
    }).collect();

    let mut nums: Vec<Vec<Match>> = Vec::new();
    let reg_num = Regex::new(r"(?<num>\d+)").unwrap();
    for line in lines.as_slice() {
        let matches: Vec<Match> = reg_num.captures_iter(line).map(|caps| {
            caps.name("num").unwrap()
        }).collect();

        nums.push(matches);
    }

    let mut sum: u32 = 0;
    let chars = [ '.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' ];
    for (row, matches) in nums.iter().enumerate() {
        if matches.is_empty() {
            continue;
        }

        'match_loop: for m in matches {
            let start_col = m.start();
            let end_col = m.end();

            let rows_to_check = row - 1..=row + 1;
            let cols_to_check = start_col - 1..end_col + 1;

            for r in rows_to_check.clone() {
                for c in cols_to_check.clone() {
                    if r == row && (start_col..end_col).contains(&c) {
                        continue;
                    }

                    if !chars.contains(&lines[r].chars().nth(c).unwrap()) {
                        sum += m.as_str().parse::<u32>().unwrap();
                        continue 'match_loop;
                    }
                }
            }
        }
    }

    println!("{sum}");

}

pub fn part02() {
    println!("d3p2");
}