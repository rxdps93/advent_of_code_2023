use core::panic;

use regex::Regex;

pub fn part01() {
    let lines = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" ];

        // let da_textier:Vec<&str> = lines.split("\nGame ").map(|x| x.trim().split(": ").last().unwrap()).collect();
        let mut idx: u32 = 1;
        for line in lines {

            let pos = line.find(":").unwrap() + 2;
            let tmp = line.chars().skip(pos).collect::<String>();
            // println!("{}\n\t{}", line, tmp);

            // let split: Vec<&str> = tmp.split("; ").map(|x| x.trim().split(", ").last().unwrap()).collect();
            // println!("{:?}\n", split);
            let split: Vec<&str> = tmp.split(";").map(|x| x.trim()).collect();
            // println!("{:?}\n", split);

            println!("{}", line);
            for entry in split {
                // let mapped: Vec<&str> = entry.split(", ").map(|x| (x.split(" ").unzip().0 x.split(" ").unzip().1)).collect();
                // println!("{:?}", mapped);
            }
        }

    // let mut sum: u32 = 0;
    // let mut idx: u32 = 0;
    // 'parse: for line in lines {

    //     idx += 1;
    //     let re = Regex::new(r"(?<count>[\d]) (?<color>red|green|blue)").unwrap();

    //     let Some(caps) = re.captures(line) else {
    //         println!("nothin");
    //         continue;
    //     };
        
    //     let matches: Vec<(u32, &str)> = re.captures_iter(line).map(|_caps| {
    //         let count: u32 = caps.name("count").unwrap().as_str().parse().unwrap();
    //         let color = caps.name("color").unwrap().as_str();
    //         (count, color)
    //     }).collect();

    //     println!("{:?}: {}", matches, matches.len());

    //     for entry in matches {
    //         println!("{:?}", entry);
    //         match entry.1 {
    //             "red" => if entry.0 > 12 { 
    //                 println!("no!");
    //                 continue 'parse 
    //             },
    //             "green" => if entry.0 > 13 { 
    //                 println!("no!");
    //                 continue 'parse
    //             },
    //             "blue" => if entry.0 > 14 { 
    //                 println!("no!");
    //                 continue 'parse
    //             },
    //             _ => panic!("Unexpected color found")
    //         };
    //     }

    //     println!("{} is possible", line);
    //     sum += idx;
    // }

    // println!("{}", sum);
}

pub fn part02() {
    println!("d2p2");
}