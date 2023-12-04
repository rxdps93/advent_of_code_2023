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
    let mut sum: u32 = 0;
    for line in input_as_str_vec("day01") {
            let transformed_line = line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6e")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .replace(char::is_alphabetic, "");

            let first = transformed_line.chars().find(char::is_ascii_digit).unwrap();
            let last = transformed_line.chars().rev().find(char::is_ascii_digit).unwrap();
    
            sum += (10 * first.to_digit(10).unwrap()) + last.to_digit(10).unwrap();
    }
    println!("{}", sum);
}