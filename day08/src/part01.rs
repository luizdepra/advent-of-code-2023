use std::collections::HashMap;

use regex::Regex;

fn main() {
    let lines = include_str!("input01.txt").lines().collect::<Vec<&str>>();

    let directions = lines[0].chars().collect::<Vec<char>>();

    let re = Regex::new(r"^(?<key>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)$").unwrap();

    let mut map = HashMap::new();
    for line in &lines[2..] {
        let caps = re.captures(line).unwrap();

        map.insert(
            caps["key"].to_string(),
            (caps["left"].to_string(), caps["right"].to_string()),
        );
    }

    let mut step = "AAA".to_string();
    let mut index = 0;
    let mut count = 0;
    while step != "ZZZ" {
        step = if directions[index] == 'L' {
            map[&step].0.clone()
        } else {
            map[&step].1.clone()
        };

        index += 1;
        if index == directions.len() {
            index = 0;
        }

        count += 1;
    }

    println!("count = {}", count);
}
