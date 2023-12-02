fn main() {
    let input = include_str!("input01.txt");

    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                c if c.is_numeric() => c.to_digit(10).unwrap(),
                _ => match line[i..].to_string() {
                    x if x.starts_with("one") => 1,
                    x if x.starts_with("two") => 2,
                    x if x.starts_with("three") => 3,
                    x if x.starts_with("four") => 4,
                    x if x.starts_with("five") => 5,
                    x if x.starts_with("six") => 6,
                    x if x.starts_with("seven") => 7,
                    x if x.starts_with("eight") => 8,
                    x if x.starts_with("nine") => 9,
                    _ => 0,
                },
            })
            .filter(|n| *n != 0)
            .collect();

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        sum += first * 10 + last;
    }

    println!("{}", sum);
}
