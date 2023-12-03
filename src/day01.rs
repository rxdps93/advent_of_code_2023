use crate::utils::input_as_str_vec;

pub fn part01() {
    
    let mut sum: u32 = 0;
    for line in input_as_str_vec("day01") {
        let first = line.chars().find(char::is_ascii_digit).unwrap();
        let last = line.chars().rev().find(char::is_ascii_digit).unwrap();

        sum += (10 * first.to_digit(10).unwrap()) + last.to_digit(10).unwrap();
    }

    println!("{}", sum);
}

pub fn part02() {
    println!("Day 01 Part 02");
}