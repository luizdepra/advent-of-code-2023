use std::collections::HashMap;

use regex::Regex;

fn find_distance<'a>(
    map: &'a HashMap<String, (String, String)>,
    directions: &Vec<char>,
    mut step: &'a String,
) -> u64 {
    let mut index = 0;
    let mut distance = 0;
    while !step.ends_with("Z") {
        step = if directions[index] == 'L' {
            &map[step].0
        } else {
            &map[step].1
        };

        index += 1;
        if index == directions.len() {
            index = 0;
        }

        distance += 1;
    }

    distance
}

fn lcm(numbers: Vec<u64>) -> u64 {
    let mut temp = numbers.clone();

    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        match temp
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
        {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            }
            None => panic!("Not possible"),
        }
    }
}

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

    let distances = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| find_distance(&map, &directions, k))
        .collect::<Vec<u64>>();

    println!("distance = {}", lcm(distances));
}
