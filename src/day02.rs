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
    let lines = input_as_str_vec("day02");
    // let lines = vec![
    //     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    //     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    //     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    //     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    //     "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    // ];

    let mut sum: u32 = 0;
    for line in lines {
        let tmp = line.chars().skip(line.find(":").unwrap() + 2).collect::<String>();
        let split: Vec<&str> = tmp.split(";").map(|x| x.trim()).collect();

        let mut red: u32 = 0;
        let mut grn: u32 = 0;
        let mut blu: u32 = 0;
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

            if rgb.0.is_some() && *rgb.0.unwrap() > red {
                red = *rgb.0.unwrap();
            }

            if rgb.1.is_some() && *rgb.1.unwrap() > grn {
                grn = *rgb.1.unwrap();
            }

            if rgb.2.is_some() && *rgb.2.unwrap() > blu {
                blu = *rgb.2.unwrap();
            }
        }

        sum += red * grn * blu;
    }

    println!("{sum}");
}
