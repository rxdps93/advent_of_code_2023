use std::collections::HashMap;

use crate::utils::input_as_str_vec;

pub fn part01() {
    let lines = input_as_str_vec("day02");

    let mut sum: u32 = 0;
    let mut idx: u32 = 0;
    'game: for line in lines {
        idx += 1;
        let tmp = line.chars().skip(line.find(":").unwrap() + 2).collect::<String>();
        let split: Vec<&str> = tmp.split(";").map(|x| x.trim()).collect();

        for entry in split {
            let mapped: HashMap<&str, u32> = entry
                .split(", ")
                .map(|x| {
                    let pair: Vec<&str> = x.split(" ").collect();
                    (
                        *pair.last().unwrap(),
                        pair.first().unwrap().parse::<u32>().unwrap(),
                    )
                })
                .collect();

            let rgb = (mapped.get("red"), mapped.get("green"), mapped.get("blue"));

            if (rgb.0.is_some() && rgb.0.unwrap() > &12)
                || (rgb.1.is_some() && rgb.1.unwrap() > &13)
                || (rgb.2.is_some() && rgb.2.unwrap() > &14)
            {
                continue 'game;
            }
        }
        sum += idx;
    }

    println!("{}", sum);
}

pub fn part02() {
    println!("d2p2");
}
