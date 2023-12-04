use std::collections::HashSet;

const MAX_CARD_ID: usize = 213;

fn extract_numbers(section: &str) -> HashSet<u32> {
    section
        .split(' ')
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}

fn main() {
    let mut card_quantities = [0; MAX_CARD_ID];

    for line in include_str!("input01.txt")
        .split('\n')
        .filter(|l| l.len() > 0)
    {
        let sections = line.split(':').collect::<Vec<&str>>();
        let numbers_sections = sections[1].split('|').collect::<Vec<&str>>();

        let index = sections[0][5..]
            .chars()
            .filter(|c| c.is_numeric())
            .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap()) as usize
            - 1;

        card_quantities[index] += 1;

        let winning_numbers = extract_numbers(numbers_sections[0]);
        let card_numbers = extract_numbers(numbers_sections[1]);

        let count = card_numbers.intersection(&winning_numbers).count();
        for i in 1..=count {
            let step_index = (index + i) % MAX_CARD_ID;

            card_quantities[step_index] += 1 * card_quantities[index];
        }
    }

    println!("{:?}", card_quantities.iter().sum::<u32>());
}
