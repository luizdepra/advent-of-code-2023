use std::collections::HashSet;

fn extract_numbers(section: &str) -> HashSet<u32> {
    section
        .split(' ')
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}

fn main() {
    let input = include_str!("input01.txt");

    let sum: u32 = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|line| {
            let sections = line.split(':').collect::<Vec<&str>>();
            let numbers_sections = sections[1].split('|').collect::<Vec<&str>>();

            let winning_numbers = extract_numbers(numbers_sections[0]);
            let card_numbers = extract_numbers(numbers_sections[1]);

            let count = card_numbers.intersection(&winning_numbers).count() as u32;

            if count > 0 {
                2u32.pow(count - 1)
            } else {
                0
            }
        })
        .sum();

    println!("Sum = {}", sum);
}
