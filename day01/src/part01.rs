fn main() {
    let input = include_str!("input01.txt");

    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let value = first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();

        sum += value;
    }

    println!("{}", sum);
}
