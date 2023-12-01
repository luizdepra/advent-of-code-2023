use regex::Regex;

fn main() {
    let input = include_str!("input01.txt");
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let numbers = re
            .find_iter(line)
            .map(|x| match x.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                x => x.parse::<i32>().unwrap(),
            })
            .collect::<Vec<i32>>();

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        let value = first * 10 + last;

        println!("{}: {:?} -> {} {} = {}", line, numbers, first, last, value);

        sum += value;
    }

    println!("result: {}", sum);
}
